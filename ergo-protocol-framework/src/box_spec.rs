pub use ergo_lib::ast::Constant;
use ergo_lib::chain::address::{Address, AddressEncoder, NetworkPrefix};
pub use ergo_lib::chain::ergo_box::ErgoBox;
pub use ergo_lib::chain::token::{TokenAmount, TokenId};
use ergo_lib::serialization::serializable::SigmaSerializable;
use ergo_lib::ErgoTree;
use ergo_lib_wasm::box_coll::ErgoBoxes;
use ergo_lib_wasm::ergo_box::ErgoBox as WErgoBox;
use ergo_offchain_utilities::encoding::address_string_to_ergo_tree;
use ergo_offchain_utilities::{ErgoAddressString, NanoErg};
use serde_json::from_str;
use std::ops::Range;
use thiserror::Error;
use wasm_bindgen::prelude::*;

pub type Result<T> = std::result::Result<T, ProtocolFrameworkError>;

#[derive(Error, Debug)]
pub enum ProtocolFrameworkError {
    #[error("The address of the box does not match the address.")]
    InvalidAddress,
    #[error("The number of Ergs held within the box is outside of the valid range.")]
    InvalidErgsValue,
    #[error("One of the token predicates failed for the provided box.")]
    FailedTokenSpec,
    #[error("One of the register predicates failed for the provided box.")]
    FailedRegisterSpec,
    #[error("The provided TokenId is invalid.")]
    InvalidTokenId,
    #[error("{0}")]
    Other(String),
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct TokenSpec {
    value_range: Range<u64>,
    token_id: String,
}
impl TokenSpec {
    pub fn new(value_range: Range<u64>, token_id: &str) -> Result<Self> {
        Ok(TokenSpec {
            value_range: value_range,
            token_id: token_id.to_string(),
        })
    }
}

/// A specification which specifies parameters of a `ErgoBox`.
/// This spec is used as a "source of truth" to both verify and find
/// `ErgoBox`es which match the spec. This is often used for defining
/// Stages in multi-stage smart contract protocols, but can also be used
/// to define input boxes for Actions.
/// All fields are wrapped in `Option`s to allow ignoring specifying
/// the field.
#[wasm_bindgen]
#[derive(Clone)]
pub struct BoxSpec {
    /// The script that locks said box as a `ErgoTree`
    ergo_tree: Option<ErgoTree>,
    /// The allowed range of nanoErgs
    value_range: Option<Range<NanoErg>>,
    /// A sorted list of `Constant`s which define registers
    /// of an `ErgoBox`.
    /// First element is treated as R4, second as R5, and so on.
    registers: Vec<Option<Constant>>,
    /// A sorted list of `TokenSpec`s which define tokens
    /// of an `ErgoBox`.
    tokens: Vec<Option<TokenSpec>>,
}

#[wasm_bindgen]
impl BoxSpec {
    #[wasm_bindgen]
    /// Acquire the address of the `BoxSpec` based on the `ErgoTree` inside
    /// of the struct.
    pub fn address_string(&self) -> Option<ErgoAddressString> {
        if let Some(ergo_tree) = self.ergo_tree.clone() {
            let address = Address::P2S(ergo_tree.sigma_serialize_bytes());
            let encoder = AddressEncoder::new(NetworkPrefix::Mainnet);
            return Some(encoder.address_to_str(&address));
        }
        None
    }

    #[wasm_bindgen]
    pub fn utxo_scan_json(&self) {
        todo!()
    }

    #[wasm_bindgen]
    pub fn w_verify_box(&self, ergo_box: WErgoBox) -> bool {
        todo!()
    }

    #[wasm_bindgen]
    pub fn w_find_boxes_in_explorer(&self, explorer_api_url: &str) -> ErgoBoxes {
        // Look into these pages to figure out requests:
        // - https://www.fpcomplete.com/blog/serverless-rust-wasm-cloudflare/
        // - https://rustwasm.github.io/docs/wasm-bindgen/examples/fetch.html

        todo!()
    }
}

impl BoxSpec {
    /// Create a new `BoxSpec`
    pub fn new(
        address: Option<ErgoAddressString>,
        value_range: Option<Range<NanoErg>>,
        registers: Vec<Option<Constant>>,
        tokens: Vec<Option<TokenSpec>>,
    ) -> Result<BoxSpec> {
        let mut ergo_tree = None;
        // If an address was provided, convert it into an ErgoTree
        if let Some(a) = address {
            if let Ok(et) = address_string_to_ergo_tree(&a) {
                ergo_tree = Some(et);
            } else {
                return Err(ProtocolFrameworkError::InvalidAddress);
            }
        }
        // Create the BoxSpec
        return Ok(BoxSpec {
            ergo_tree: ergo_tree,
            value_range: value_range,
            registers: registers,
            tokens: tokens,
        });
    }

    /// Verify that a provided `ErgoBox` matches the spec
    pub fn verify_box(&self, ergo_box: ErgoBox) -> Result<()> {
        // Verify the address/ErgoTree locking script
        if let Some(tree) = self.ergo_tree.clone() {
            match tree == ergo_box.ergo_tree {
                true => Ok(()),
                false => Err(ProtocolFrameworkError::InvalidAddress),
            }?;
        }
        // Verify value held in the box is within the valid range
        if let Some(value_range) = self.value_range.clone() {
            match value_range.contains(&ergo_box.value.as_u64()) {
                true => Ok(()),
                false => Err(ProtocolFrameworkError::InvalidErgsValue),
            }?;
        }

        todo!()
    }

    /// Finds boxes which match your `BoxSpec` via using an instance of
    /// the Ergo Explorer Backend.
    /// `explorer_api_url` must be formatted as such:
    /// `https://api.ergoplatform.com/api/v0/`
    pub fn find_boxes_in_explorer(&self, explorer_api_url: &str) -> Result<Vec<ErgoBox>> {
        // Verify an address exists
        if self.address_string().is_none() {
            return Err(ProtocolFrameworkError::Other("Using the Ergo Explorer API currently requires defining an address for your `BoxStruct`.".to_string()));
        }

        let url = explorer_api_url.to_string()
            + "transactions/boxes/byAddress/unspent/"
            + &self.address_string().unwrap();

        println!("Endpoint: {}", url);

        let client = reqwest::blocking::Client::new().get(&url);
        let resp = client.send().map_err(|_| {
            ProtocolFrameworkError::Other(
                "Failed to make GET response to the Ergo Explorer Backend API.".to_string(),
            )
        });
        let text = resp?.text().map_err(|_| {
            ProtocolFrameworkError::Other(
                "Failed to extract text from Ergo Explorer Backend API Response".to_string(),
            )
        })?;

        // Parse the json String/filter the boxes against the `BoxSpec`
        let ergo_boxes = self.parse_ergo_boxes_json_string(text)?;

        Ok(ergo_boxes)
    }

    /// Parses `ErgoBox`es from a JSON `String` and then filters them
    /// based on the `BoxSpec` using `verify_box()`.
    pub fn parse_ergo_boxes_json_string(&self, ergo_boxes_text: String) -> Result<Vec<ErgoBox>> {
        // Get the `JsonValue`
        let json = json::parse(&ergo_boxes_text).map_err(|_| {
            ProtocolFrameworkError::Other(
                "Failed to extract text from Ergo Explorer Backend API Response".to_string(),
            )
        })?;
        // Parse the json into `Vec<ErgoBox>`
        let mut box_list: Vec<ErgoBox> = vec![];
        for i in 0.. {
            let box_json = &json[i];
            if box_json.is_null() {
                break;
            } else {
                let res_ergo_box = from_str(&box_json.to_string());
                if let Ok(ergo_box) = res_ergo_box {
                    box_list.push(ergo_box);
                } else if let Err(e) = res_ergo_box {
                    let mess = format!("Box Json: {}\nError: {:?}", box_json.to_string(), e);
                    return Err(ProtocolFrameworkError::Other(mess));
                }
            }
        }

        let filtered_boxes = box_list.into_iter().fold(vec![], |mut acc, b| {
            if self.verify_box(b.clone()).is_ok() {
                acc.push(b);
            }
            return acc;
        });
        Ok(filtered_boxes.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_ergo_box_spec() {
        let address =
            "88dhgzEuTXaTHv7qHnCK2mYG32GyBrYDyKKku7HdU3kHwhYRnB3ngdP5gF7K4mzZEbk2CBVhaeXh97R8"
                .to_string();
        let value_range = 1..1000000000000;
        let registers = vec![];
        let tokens = vec![];
        let box_spec_res = BoxSpec::new(address, value_range, registers, tokens);

        assert!(box_spec_res.is_ok())
    }

    #[test]
    fn find_boxes_in_explorer() {
        let address =
            "9aFbqNsmDwSxCdcLDKmSxVTL58ms2A39Rpn2zodVzkBN5MzB8zvW5PFX551W1A5vUdFJ3yxwvwgYTTS4JrPQcb5qxBbRDJkGNikuqHRXhnbniK4ajumEj7ot2o7DbcNFaM674fWufQzSGS1KtgMw95ZojyqhswUNbKpYDV1PhKw62bEMdJL9vAvzea4KwKXGUTdYYkcPdQKFWXfrdo2nTS3ucFNxqyTRB3VtZk7AWE3eeNHFcXZ1kLkfrX1ZBjpQ7qrBemHk4KZgS8fzmm6hPSZThiVVtBfQ2CZhJQdAZjRwGrw5TDcZ4BBDAZxg9h13vZ7tQSPsdAtjMFQT1DxbqAruKxX38ZwaQ3UfWmbBpbJEThAQaS4gsCBBSjswrv8BvupxaHZ4oQmA2LZiz4nYaPr8MJtR4fbM9LErwV4yDVMb873bRE5TBF59NipUyHAir7ysajPjbGc8aRLqsMVjntFSCFYx7822RBrj7RRX11CpiGK6vdfKHe3k14EH6YaNXvGSq8DrfNHEK4SgreknTqCgjL6i3EMZKPCW8Lao3Q5tbJFnFjEyntpUDf5zfGgFURxzobeEY4USqFaxyppHkgLjQuFQtDWbYVu3ztQL6hdWHjZXMK4VVvEDeLd1woebD1CyqS5kJHpGa78wQZ4iKygw4ijYrodZpqqEwTXdqwEB6xaLfkxZCBPrYPST3xz67GGTBUFy6zkXP5vwVVM5gWQJFdWCZniAAzBpzHeVq1yzaBp5GTJgr9bfrrAmuX8ra1m125yfeT9sTWroVu"
                .to_string();
        let value_range = 1..1000000000000;
        let registers = vec![];
        let tokens = vec![];
        let box_spec = BoxSpec::new(address, value_range, registers, tokens).unwrap();

        let matching_boxes = box_spec
            .find_boxes_in_explorer("https://api.ergoplatform.com/api/v0/")
            .unwrap();

        assert!(matching_boxes.len() > 0)
    }
}
