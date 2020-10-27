pub use ergo_lib::ast::Constant;
use ergo_lib::chain::address::{Address, AddressEncoder, NetworkPrefix};
pub use ergo_lib::chain::ergo_box::ErgoBox;
pub use ergo_lib::chain::token::{TokenAmount, TokenId};
use ergo_lib::serialization::serializable::SigmaSerializable;
use ergo_lib::ErgoTree;
use ergo_offchain_utilities::{ErgoAddressString, NanoErg};
use std::ops::Range;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, BoxVerificationError>;

#[derive(Error, Debug)]
pub enum BoxVerificationError {
    #[error("The P2S address of the box does not match the `StageChecker` P2S address.")]
    InvalidP2SAddress,
    #[error("The number of Ergs held within the box is outside of the valid range.")]
    InvalidErgsValue,
    #[error("The number of token predicates defined for your `StageChecker` are greater than the number of unique tokens held in the box. In other words, the box holds an insufficient number of different types of tokens.")]
    LessTokensThanPredicates,
    #[error("One of the token predicates failed for the provided box.")]
    FailedTokenSpec,
    #[error("The number of register predicates defined for your `StageChecker` are greater than the number of registers used in the box.")]
    LessRegistersThanPredicates,
    #[error("One of the register predicates failed for the provided box.")]
    FailedRegisterSpec,
    #[error("The provided TokenId is invalid.")]
    InvalidTokenId,
}

#[derive(Clone)]
pub struct TokenSpec {
    pub value_range: Range<NanoErg>,
    pub token_id: String,
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
pub struct BoxSpec {
    /// The script that locks said box as a `ErgoTree`
    pub ergo_tree: ErgoTree,
    /// The allowed range of nanoErgs
    pub value_range: Range<NanoErg>,
    /// A sorted list of `Constant`s which define registers
    /// of an `ErgoBox`.
    /// First element is treated as R4, second as R5, and so on.
    pub registers: Vec<Constant>,
    /// A sorted list of `TokenSpec`s which define tokens
    /// of an `ErgoBox`.
    pub tokens: Vec<TokenSpec>,
}

impl BoxSpec {
    pub fn address_string(&self) -> ErgoAddressString {
        todo!()
    }

    pub fn utxo_scan_json(&self) {
        todo!()
    }

    pub fn verify_box(&self, ergo_box: ErgoBox) -> bool {
        todo!()
    }

    pub fn find_boxes_in_explorer(&self) -> Vec<ErgoBox> {
        todo!()
    }
}
