pub use ergo_lib::ast::Constant;
use ergo_lib::chain::address::{Address, AddressEncoder, NetworkPrefix};
pub use ergo_lib::chain::ergo_box::ErgoBox;
pub use ergo_lib::chain::token::{TokenAmount, TokenId};
use ergo_lib::serialization::serializable::SigmaSerializable;
use ergo_lib::ErgoTree;
use ergo_lib_wasm::box_coll::ErgoBoxes;
use ergo_lib_wasm::ergo_box::ErgoBox as WErgoBox;
use ergo_offchain_utilities::{ErgoAddressString, NanoErg};
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
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct TokenSpec {
    value_range: Range<NanoErg>,
    token_id: String,
}
impl TokenSpec {
    pub fn new(value_range: Range<NanoErg>, token_id: &str) -> Result<Self> {
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
#[wasm_bindgen]
#[derive(Clone)]
pub struct BoxSpec {
    /// The script that locks said box as a `ErgoTree`
    ergo_tree: ErgoTree,
    /// The allowed range of nanoErgs
    value_range: Range<NanoErg>,
    /// A sorted list of `Constant`s which define registers
    /// of an `ErgoBox`.
    /// First element is treated as R4, second as R5, and so on.
    registers: Vec<Constant>,
    /// A sorted list of `TokenSpec`s which define tokens
    /// of an `ErgoBox`.
    tokens: Vec<TokenSpec>,
}

#[wasm_bindgen]
impl BoxSpec {
    #[wasm_bindgen]
    /// Acquire the address of the `BoxSpec` based on the `ErgoTree` inside
    /// of the struct.
    pub fn address_string(&self) -> ErgoAddressString {
        let address = Address::P2S(self.ergo_tree.sigma_serialise_bytes());
        let encoder = AddressEncoder::new(NetworkPrefix::Mainnet);
        encoder.address_to_str(&address)
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
    pub fn w_find_boxes_in_explorer(&self) -> ErgoBoxes {
        todo!()
    }
}

impl BoxSpec {
    // /// Create a new `BoxSpec`
    // pub fn new() -> BoxSpec {}

    pub fn verify_box(&self, ergo_box: ErgoBox) -> Result<()> {
        let address_check = match self.ergo_tree == ergo_box.ergo_tree {
            true => Ok(()),
            false => Err(ProtocolFrameworkError::InvalidAddress),
        }?;

        // Verify value held in the box is within the valid range
        let value_within_range = match self.value_range.contains(&ergo_box.value.as_u64()) {
            true => Ok(()),
            false => Err(ProtocolFrameworkError::InvalidErgsValue),
        }?;

        todo!()
    }

    /// Finds boxes which match your `BoxSpec` via using an instance of
    /// the Ergo Explorer Backend.
    /// `explorer_api_url` must be formatted as such:
    /// `https://api.ergoplatform.com/api/v0/`
    pub fn find_boxes_in_explorer(&self, explorer_api_url: &str) -> Vec<ErgoBox> {
        let explorer_api_url = "https://api.ergoplatform.com/api/v0/".to_string();
        let endpoint =
            explorer_api_url + "transactions/boxes/byAddress/unspent/" + &self.address_string();
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn create_ergo_box_spec() {}
}
