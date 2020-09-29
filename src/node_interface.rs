/// The `NodeInterface` struct is defined which allows for interacting with an
/// Ergo Node via Rust.
use crate::ScanID;
use crate::{BlockHeight, P2PKAddress, P2SAddress, TxId};
use json::JsonValue;
use log::info;
use reqwest::blocking::{RequestBuilder, Response};
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use serde_json::from_str;
use sigma_tree::chain::ergo_box::ErgoBox;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, NodeError>;

#[derive(Error, Debug)]
pub enum NodeError {
    #[error("The configured node is unreachable. Please ensure your config is correctly filled out and the node is running.")]
    NodeUnreachable,
    #[error("Failed reading response from node: {0}")]
    FailedParsingNodeResponse(String),
    #[error("Failed parsing JSON box from node: {0}")]
    FailedParsingBox(String),
    #[error("No Boxes Were Found.")]
    NoBoxesFound,
    #[error("The node rejected the request you provided.\nNode Response: {0}")]
    BadRequest(String),
    #[error("The node is still syncing.")]
    NodeSyncing,
}

/// The `NodeInterface` struct which holds the relevant Ergo node data
/// and has methods implemented to interact with the node.
#[derive(Debug, Clone)]
pub struct NodeInterface {
    pub api_key: String,
    pub ip: String,
    pub port: String,
}

impl NodeInterface {
    /// Create a new `NodeInterface` using details about the Node
    pub fn new(api_key: &str, ip: &str, port: &str) -> NodeInterface {
        NodeInterface {
            api_key: api_key.to_string(),
            ip: ip.to_string(),
            port: port.to_string(),
        }
    }

    /// Returns `http://ip:port` using `ip` and `port` from self
    pub fn get_node_url(&self) -> String {
        "http://".to_string() + &self.ip + ":" + &self.port
    }

    /// Registers a scan with the node and either returns the `scan_id` or an error
    pub fn register_scan(&self, scan_json: &JsonValue) -> Result<ScanID> {
        let endpoint = "/scan/register";
        let body = scan_json.clone().to_string();
        let res = self.send_post_req(endpoint, body);
        let res_json = self.parse_response_to_json(res)?;

        info!(
            "Node Register Scan Response Json: {}",
            &res_json.pretty(2).to_string()
        );

        if res_json["error"].is_null() {
            return Ok(res_json["scanId"].to_string().clone());
        } else {
            return Err(NodeError::BadRequest(res_json["error"].to_string()));
        }
    }

    /// Using the `scan_id` of a registered scan, acquires unspent boxes which have been found by said scan
    pub fn get_scan_boxes(&self, scan_id: &ScanID) -> Result<Vec<ErgoBox>> {
        let endpoint = "/scan/unspentBoxes/".to_string() + scan_id;
        let res = self.send_get_req(&endpoint);
        let res_json = self.parse_response_to_json(res)?;

        let mut box_list = vec![];
        for i in 0.. {
            let box_json = &res_json[i]["box"];
            if box_json.is_null() {
                break;
            } else {
                let res_ergo_box = from_str(&box_json.to_string());
                if let Ok(ergo_box) = res_ergo_box {
                    box_list.push(ergo_box);
                } else if let Err(e) = res_ergo_box {
                    let mess = format!("Box Json: {}\nError: {:?}", box_json.to_string(), e);
                    return Err(NodeError::FailedParsingBox(mess));
                }
            }
        }
        Ok(box_list)
    }

    /// Acquires unspent boxes from the node wallet
    pub fn get_unspent_wallet_boxes(&self) -> Result<Vec<ErgoBox>> {
        let endpoint = "/wallet/boxes/unspent?minConfirmations=0&minInclusionHeight=0";
        let res = self.send_get_req(endpoint);
        let res_json = self.parse_response_to_json(res)?;

        let mut box_list = vec![];

        for i in 0.. {
            let box_json = &res_json[i]["box"];
            if box_json.is_null() {
                break;
            } else {
                if let Some(ergo_box) = from_str(&box_json.to_string()).ok() {
                    box_list.push(ergo_box);
                }
            }
        }
        Ok(box_list)
    }

    /// Acquires the unspent box with the highest value of Ergs inside
    /// from the wallet
    pub fn get_highest_value_unspent_box(&self) -> Result<ErgoBox> {
        let boxes = self.get_unspent_wallet_boxes()?;

        // Find the highest value amount held in a single box in the wallet
        let highest_value = boxes.iter().fold(0, |acc, b| {
            if b.value.as_u64() > acc {
                b.value.as_u64()
            } else {
                acc
            }
        });

        for b in boxes {
            if b.value.as_u64() == highest_value {
                return Ok(b);
            }
        }
        Err(NodeError::NoBoxesFound)
    }

    /// Acquires the unspent box with the highest value of Ergs inside
    /// from the wallet and serializes it
    pub fn get_serialized_highest_value_unspent_box(&self) -> Result<String> {
        let ergs_box_id: String = self.get_highest_value_unspent_box()?.box_id().into();
        self.serialized_box_from_id(&ergs_box_id)
    }

    /// Generates (and sends) a tx using the node endpoints.
    /// Input must be a json formatted request with rawInputs (and rawDataInputs)
    /// manually selected or will be automatically selected by wallet.
    /// Returns the resulting `TxId`.
    pub fn send_transaction(&self, tx_request_json: &JsonValue) -> Result<TxId> {
        let endpoint = "/wallet/transaction/send";
        let body = json::stringify(tx_request_json.clone());
        let res = self.send_post_req(endpoint, body);

        info!("{:?}", tx_request_json.dump());

        let res_json = self.parse_response_to_json(res)?;
        let error_details = res_json["detail"].to_string().clone();

        // Check if send tx request failed and returned error json
        if error_details != "null" {
            return Err(NodeError::BadRequest(error_details));
        }
        // Otherwise if tx is valid and is posted, return just the tx id
        else {
            // Clean string to be only the tx_id value
            let tx_id = res_json.dump();

            return Ok(tx_id);
        }
    }

    /// Given a P2S Ergo address, extract the hex-encoded serialized ErgoTree (script)
    pub fn address_to_tree(&self, address: &P2SAddress) -> Result<String> {
        let endpoint = "/script/addressToTree/".to_string() + address;
        let res = self.send_get_req(&endpoint);
        let res_json = self.parse_response_to_json(res)?;

        Ok(res_json["tree"].to_string().clone())
    }

    /// Given a P2S Ergo address, convert it to a hex-encoded Sigma byte array constant
    pub fn address_to_bytes(&self, address: &P2SAddress) -> Result<String> {
        let endpoint = "/script/addressToBytes/".to_string() + address;
        let res = self.send_get_req(&endpoint);
        let res_json = self.parse_response_to_json(res)?;

        Ok(res_json["bytes"].to_string().clone())
    }

    /// Given an Ergo P2PK Address, convert it to a raw hex-encoded EC point
    pub fn address_to_raw(&self, address: &P2PKAddress) -> Result<String> {
        let endpoint = "/utils/addressToRaw/".to_string() + address;
        let res = self.send_get_req(&endpoint);
        let res_json = self.parse_response_to_json(res)?;

        Ok(res_json["raw"].to_string().clone())
    }

    /// Given an Ergo P2PK Address, convert it to a raw hex-encoded EC point
    /// and prepend the type bytes so it is encoded and ready
    /// to be used in a register.
    pub fn address_to_raw_for_register(&self, address: &P2PKAddress) -> Result<String> {
        let add = self.address_to_raw(address)?;
        Ok("07".to_string() + &add)
    }

    /// Given a raw hex-encoded EC point, convert it to a P2PK address
    pub fn raw_to_address(&self, raw: &String) -> Result<P2PKAddress> {
        let endpoint = "/utils/rawToAddress/".to_string() + raw;
        let res = self.send_get_req(&endpoint);
        let res_json = self.parse_response_to_json(res)?;

        Ok(res_json["address"].to_string().clone())
    }

    /// Given a raw hex-encoded EC point from a register (thus with type encoded characters in front),
    /// convert it to a P2PK address
    pub fn raw_from_register_to_address(&self, typed_raw: &String) -> Result<P2PKAddress> {
        Ok(self.raw_to_address(&typed_raw[2..].to_string())?)
    }

    /// Given a `Vec<ErgoBox>` return the given boxes (which must be part of the UTXO-set) as
    /// a vec of serialized strings in Base16 encoding
    pub fn serialize_boxes(&self, b: &Vec<ErgoBox>) -> Result<Vec<String>> {
        Ok(b.iter()
            .map(|b| {
                self.serialized_box_from_id(&b.box_id().into())
                    .unwrap_or("".to_string())
            })
            .collect())
    }

    /// Given an `ErgoBox` return the given box (which must be part of the UTXO-set) as
    /// a serialized string in Base16 encoding
    pub fn serialize_box(&self, b: &ErgoBox) -> Result<String> {
        self.serialized_box_from_id(&b.box_id().into())
    }

    /// Given a box id return the given box (which must be part of the UTXO-set) as
    /// a serialized string in Base16 encoding
    pub fn serialized_box_from_id(&self, box_id: &String) -> Result<String> {
        let endpoint = "/utxo/byIdBinary/".to_string() + box_id;
        let res = self.send_get_req(&endpoint);
        let res_json = self.parse_response_to_json(res)?;

        Ok(res_json["bytes"].to_string().clone())
    }

    /// Get the current block height of the chain
    pub fn current_block_height(&self) -> Result<BlockHeight> {
        let endpoint = "/info";
        let res = self.send_get_req(&endpoint);
        let res_json = self.parse_response_to_json(res)?;

        // Switched from fullHeight to height to prevent errors when node is syncing headers. Need to ensure this still works as expected.
        let height_json = res_json["fullHeight"].clone();

        if height_json.is_null() {
            return Err(NodeError::NodeSyncing);
        } else {
            return height_json
                .to_string()
                .parse()
                .map_err(|_| NodeError::FailedParsingNodeResponse(res_json.to_string()));
        }
    }

    /// Builds a `HeaderValue` to use for requests with the api key specified
    fn get_node_api_header(&self) -> HeaderValue {
        match HeaderValue::from_str(&self.api_key) {
            Ok(k) => k,
            _ => HeaderValue::from_static("None"),
        }
    }

    /// Sets required headers for a request
    fn set_req_headers(&self, rb: RequestBuilder) -> RequestBuilder {
        rb.header("accept", "application/json")
            .header("api_key", self.get_node_api_header())
            .header(CONTENT_TYPE, "application/json")
    }

    /// Sends a GET request to the Ergo node
    fn send_get_req(&self, endpoint: &str) -> Result<Response> {
        let url = self.get_node_url().to_owned() + endpoint;
        let client = reqwest::blocking::Client::new().get(&url);
        self.set_req_headers(client)
            .send()
            .map_err(|_| NodeError::NodeUnreachable)
    }

    /// Sends a POST request to the Ergo node
    fn send_post_req(&self, endpoint: &str, body: String) -> Result<Response> {
        let url = self.get_node_url().to_owned() + endpoint;
        let client = reqwest::blocking::Client::new().post(&url);
        self.set_req_headers(client)
            .body(body)
            .send()
            .map_err(|_| NodeError::NodeUnreachable)
    }

    /// Parses response from node into JSON
    fn parse_response_to_json(&self, resp: Result<Response>) -> Result<JsonValue> {
        let text = resp?.text().map_err(|_| {
            NodeError::FailedParsingNodeResponse(
                "Node Response Not Parseable into Text.".to_string(),
            )
        })?;
        let json = json::parse(&text).map_err(|_| NodeError::FailedParsingNodeResponse(text))?;
        Ok(json)
    }
}
