use crate::error::{ProtocolFrameworkError, Result};
use ergo_lib::ast::constant::Constant;
use ergo_lib::chain::ergo_box::ErgoBox;
use ergo_lib::ergo_tree::ErgoTree;
use ergo_lib::types::stype::SType;
use ergo_lib_wasm::box_coll::ErgoBoxes;
use ergo_lib_wasm::ergo_box::ErgoBox as WErgoBox;
use ergo_offchain_utilities::encoding::address_string_to_ergo_tree;
use ergo_offchain_utilities::{ErgoAddressString, NanoErg};
use serde_json::from_str;
use std::ops::Range;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
/// A struct which allows a developer to create a specification of a
/// token in a box. This `TokenSpec` is used in a `BoxSpec.
pub struct TokenSpec {
    value_range: Range<u64>,
    token_id: String,
}
impl TokenSpec {
    pub fn new(value_range: Range<u64>, token_id: &str) -> TokenSpec {
        TokenSpec {
            value_range: value_range,
            token_id: token_id.to_string(),
        }
    }
}

/// A struct which allows a developer to create a specification of a
/// Register in a box. This `RegisterSpec` is used in a `BoxSpec.
// Offers both fields as `Option`s, thus allowing a developer to specify
// how many Registers are expected, potentially the types of each register,
// and potentially the specific value of a register.
#[wasm_bindgen]
#[derive(Clone)]
pub struct RegisterSpec {
    value: Option<Constant>,
    value_type: Option<SType>,
}
impl RegisterSpec {
    pub fn new(value_type: Option<SType>, value: Option<Constant>) -> RegisterSpec {
        RegisterSpec {
            value: value,
            value_type: value_type,
        }
    }
}

/// A specification which specifies parameters of an `ErgoBox`.
/// This spec is used as a "source of truth" to both verify and find
/// `ErgoBox`es which match the spec. This is often used for defining
/// Stages in multi-stage smart contract protocols, but can also be used
/// to define input boxes for Actions.
/// All fields are wrapped in `Option`s to allow ignoring specifying
/// the field.
#[wasm_bindgen]
#[derive(Clone)]
pub struct BoxSpec {
    /// The address of the box
    address: Option<ErgoAddressString>,
    /// The allowed range of nanoErgs
    value_range: Option<Range<NanoErg>>,
    /// A sorted list of `RegisterSpec`s which define registers
    /// of an `ErgoBox`.
    /// First element is treated as R4, second as R5, and so on.
    /// The fields of a `RegisterSpec` are Options, thus
    /// removing the need for an Option on the field type.
    registers: Vec<RegisterSpec>,
    /// A sorted list of `TokenSpec`s which define tokens
    /// of an `ErgoBox`.
    tokens: Vec<Option<TokenSpec>>,
    /// An optional predicate which allows for defining custom
    /// specification logic which gets processed when verifying
    /// the box.
    predicate: Option<fn(&ErgoBox) -> bool>,
}

/// Method definitions for `BoxSpec` that are WASM-compatible by default
/// and/or are wrapper functions for Rust methods so that they
/// are made WASM-compatible.
#[wasm_bindgen]
impl BoxSpec {
    #[wasm_bindgen]
    pub fn utxo_scan_json(&self) -> String {
        todo!()
    }

    /// Returns a new `BoxSpec` with all fields exactly the same
    /// except the address is set to the String provided as input.
    /// This method is generally used to hone down a more generic
    /// `BoxSpec` definition into a more specific one for your given
    /// use case. Ie. Add a user's P2PK address to find boxes matching
    /// the `BoxSpec` in their wallet.
    #[wasm_bindgen]
    pub fn modified_address(&self, address: Option<ErgoAddressString>) -> BoxSpec {
        BoxSpec {
            address: address,
            ..self.clone()
        }
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

/// Method definitions for `BoxSpec` that are intended to be used in
/// Rust.
impl BoxSpec {
    /// Create a new basic `BoxSpec` with no predicate.
    pub fn new(
        address: Option<ErgoAddressString>,
        value_range: Option<Range<NanoErg>>,
        registers: Vec<RegisterSpec>,
        tokens: Vec<Option<TokenSpec>>,
    ) -> BoxSpec {
        BoxSpec::new_predicated(address, value_range, registers, tokens, None)
    }

    /// Create a new `BoxSpec` with a custom predicate defined.
    pub fn new_predicated(
        address: Option<ErgoAddressString>,
        value_range: Option<Range<NanoErg>>,
        registers: Vec<RegisterSpec>,
        tokens: Vec<Option<TokenSpec>>,
        predicate: Option<fn(&ErgoBox) -> bool>,
    ) -> BoxSpec {
        // Create the BoxSpec
        return BoxSpec {
            address: address,
            value_range: value_range,
            registers: registers,
            tokens: tokens,
            predicate: predicate,
        };
    }

    /// Acquire the `ErgoTree` of the address in the `BoxSpec`
    pub fn ergo_tree(&self) -> Result<ErgoTree> {
        if let Some(address) = self.address.clone() {
            return address_string_to_ergo_tree(&address)
                .map_err(|_| ProtocolFrameworkError::InvalidSpecAddress);
        }
        Err(ProtocolFrameworkError::InvalidSpecAddress)
    }

    /// Verify that a provided `ErgoBox` matches the spec
    pub fn verify_box(&self, ergo_box: &ErgoBox) -> Result<()> {
        let ergo_box_regs = ergo_box.additional_registers.get_ordered_values();

        // Verify the address/ErgoTree locking script
        if let Ok(tree) = self.ergo_tree() {
            match tree == ergo_box.ergo_tree {
                true => Ok(()),
                false => Err(ProtocolFrameworkError::InvalidAddress(
                    self.address.clone().unwrap_or_default(),
                )),
            }?;
        }
        // Verify value held in the box is within the valid range
        if let Some(value_range) = self.value_range.clone() {
            match value_range.contains(&ergo_box.value.as_u64()) {
                true => Ok(()),
                false => Err(ProtocolFrameworkError::InvalidSpecErgsValue),
            }?;
        }

        // Verify all of the RegisterSpecs
        if self.registers.len() > 0 {
            // Error if more registers specified than exist in box.
            if self.registers.len() > ergo_box_regs.len() {
                return Err(ProtocolFrameworkError::FailedRegisterSpec);
            }
            for i in 0..(self.registers.len()) {
                let rspec = self.registers[i].clone();

                // Verify that the register's type matches the spec
                if let Some(reg_type) = rspec.value_type {
                    match reg_type == ergo_box_regs[i].tpe {
                        true => (),
                        false => return Err(ProtocolFrameworkError::FailedRegisterSpec),
                    }
                }

                // Verify that the register's value matches the spec
                if let Some(constant) = rspec.value {
                    match constant == ergo_box_regs[i] {
                        true => (),
                        false => return Err(ProtocolFrameworkError::FailedRegisterSpec),
                    }
                }
            }
        }

        // Verify all of the TokensSpecs
        if self.tokens.len() > 0 {
            for i in 0..(self.tokens.len()) {
                if let Some(spec) = self.tokens[i].clone() {
                    let tok = ergo_box.tokens[i].clone();
                    let tok_id: String = tok.token_id.0.into();
                    // Verify Token ID matches spec
                    let id_check = tok_id == spec.token_id;
                    // Verify Token value is within range spec
                    let range_check = spec.value_range.contains(&tok.amount.into());

                    // If either check fails then return error
                    if !id_check || !range_check {
                        return Err(ProtocolFrameworkError::FailedTokenSpec);
                    }
                }
            }
        }

        // Verify the predicate
        if let Some(predicate) = self.predicate {
            if !(predicate)(&ergo_box) {
                return Err(ProtocolFrameworkError::FailedSpecPredicate);
            }
        }

        // Verification successful
        Ok(())
    }

    /// Generates a URL for the Ergo Explorer Backend API
    /// to find boxes which may match your `BoxSpec`. This method uses
    /// the `explorer_api_url` you provide as input which
    /// must be formatted as such:
    /// `https://api.ergoplatform.com/api/v0/`
    /// This method is intended to be used in tandem with
    /// `process_explorer_response()`
    pub fn explorer_endpoint(&self, explorer_api_url: &str) -> Result<String> {
        // Verify an address exists
        if self.address.is_none() {
            return Err(ProtocolFrameworkError::Other("Using the Ergo Explorer API currently requires defining an address for your `BoxStruct`.".to_string()));
        }

        let url = explorer_api_url.to_string()
            + "transactions/boxes/byAddress/unspent/"
            + &self.address.clone().unwrap();

        Ok(url)
    }

    /// Using the response JSON (as a String) from the Ergo Explorer API
    /// endpoint generated by the `explorer_endpoint()` method,
    /// filter all returned `ErgoBox`es against the `BoxSpec`
    /// using the `verify_box()` method.
    pub fn process_explorer_response(&self, explorer_response_body: &str) -> Result<Vec<ErgoBox>> {
        // Get the `JsonValue`
        let json = json::parse(explorer_response_body).map_err(|_| {
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
            if self.verify_box(&b).is_ok() {
                acc.push(b);
            }
            return acc;
        });
        Ok(filtered_boxes.clone())
    }
}

/// Methods related to modifying fields (which aren't WASM-compatible)
impl BoxSpec {
    /// Returns a new `BoxSpec` with all fields exactly the same
    /// except the value_range is set to the range provided as input.
    pub fn modified_value_range(&self, value_range: Option<Range<u64>>) -> BoxSpec {
        BoxSpec {
            value_range: value_range,
            ..self.clone()
        }
    }
    /// Returns a new `BoxSpec` with all fields exactly the same
    /// except the registers are set to the registers provided as input.
    pub fn modified_registers(&self, registers: Vec<RegisterSpec>) -> BoxSpec {
        BoxSpec {
            registers: registers,
            ..self.clone()
        }
    }

    /// Returns a new `BoxSpec` with all fields exactly the same
    /// except the tokens are set to the `TokenSpec`s provided as input.
    pub fn modified_tokens(&self, tokens: Vec<Option<TokenSpec>>) -> BoxSpec {
        BoxSpec {
            tokens: tokens,
            ..self.clone()
        }
    }

    /// Returns a new `BoxSpec` with all fields exactly the same
    /// except the predicate is set to the one provided as input.
    pub fn modified_predicate(&self, predicate: Option<fn(&ErgoBox) -> bool>) -> BoxSpec {
        BoxSpec {
            predicate: predicate,
            ..self.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_ergo_box_spec() {
        let address = Some(
            "88dhgzEuTXaTHv7qHnCK2mYG32GyBrYDyKKku7HdU3kHwhYRnB3ngdP5gF7K4mzZEbk2CBVhaeXh97R8"
                .to_string(),
        );
        let value_range = Some(1..1000000000000);
        let registers = vec![];
        let tokens = vec![];
        let box_spec_res = BoxSpec::new(address, value_range, registers, tokens);

        assert!(box_spec_res.tokens.is_empty())
    }
    #[test]
    fn find_boxes_in_explorer() {
        let address =
            Some("9aFbqNsmDwSxCdcLDKmSxVTL58ms2A39Rpn2zodVzkBN5MzB8zvW5PFX551W1A5vUdFJ3yxwvwgYTTS4JrPQcb5qxBbRDJkGNikuqHRXhnbniK4ajumEj7ot2o7DbcNFaM674fWufQzSGS1KtgMw95ZojyqhswUNbKpYDV1PhKw62bEMdJL9vAvzea4KwKXGUTdYYkcPdQKFWXfrdo2nTS3ucFNxqyTRB3VtZk7AWE3eeNHFcXZ1kLkfrX1ZBjpQ7qrBemHk4KZgS8fzmm6hPSZThiVVtBfQ2CZhJQdAZjRwGrw5TDcZ4BBDAZxg9h13vZ7tQSPsdAtjMFQT1DxbqAruKxX38ZwaQ3UfWmbBpbJEThAQaS4gsCBBSjswrv8BvupxaHZ4oQmA2LZiz4nYaPr8MJtR4fbM9LErwV4yDVMb873bRE5TBF59NipUyHAir7ysajPjbGc8aRLqsMVjntFSCFYx7822RBrj7RRX11CpiGK6vdfKHe3k14EH6YaNXvGSq8DrfNHEK4SgreknTqCgjL6i3EMZKPCW8Lao3Q5tbJFnFjEyntpUDf5zfGgFURxzobeEY4USqFaxyppHkgLjQuFQtDWbYVu3ztQL6hdWHjZXMK4VVvEDeLd1woebD1CyqS5kJHpGa78wQZ4iKygw4ijYrodZpqqEwTXdqwEB6xaLfkxZCBPrYPST3xz67GGTBUFy6zkXP5vwVVM5gWQJFdWCZniAAzBpzHeVq1yzaBp5GTJgr9bfrrAmuX8ra1m125yfeT9sTWroVu"
                .to_string());
        let value_range = Some(1..1000000000000);
        let registers = vec![];
        let tokens = vec![];
        let box_spec = BoxSpec::new(address, value_range, registers, tokens);

        let url = box_spec
            .explorer_endpoint("https://api.ergoplatform.com/api/v0/")
            .unwrap();

        let client = reqwest::blocking::Client::new().get(&url);
        let resp = client.send().map_err(|_| {
            ProtocolFrameworkError::Other(
                "Failed to make GET response to the Ergo Explorer Backend API.".to_string(),
            )
        });
        let text = resp.unwrap().text().unwrap();

        // Currently fails until v0.4 of ergo-lib releases which fixes
        // the json parsing issue for box_id from explorer
        let matching_boxes = box_spec.process_explorer_response(&text).unwrap();

        assert!(matching_boxes.len() > 0)
    }
}
