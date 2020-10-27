pub use ergo_lib::ast::Constant;
use ergo_lib::chain::address::{Address, AddressEncoder, NetworkPrefix};
pub use ergo_lib::chain::ergo_box::ErgoBox;
pub use ergo_lib::chain::token::TokenAmount;
use ergo_lib::serialization::serializable::SigmaSerializable;
use ergo_lib::ErgoTree;
use std::ops::Range;
use thiserror::Error;

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
}
/// A predicate which takes a `Constant` value from an `ErgoBox` register and
/// evaluates the validity of said value. This is a function which is
/// implemented by the developer to verify that a given register holds data
/// which is allowed within the protocol.
#[derive(Clone)]
pub struct RegisterSpec {
    predicate: fn(&Constant) -> bool,
}
impl RegisterSpec {
    pub fn new(predicate: fn(&Constant) -> bool) -> Self {
        RegisterSpec {
            predicate: predicate,
        }
    }
}

/// A predicate which takes a `TokenAmount` value and
/// evaluates the validity of said tokens. This predicate is a function which
/// is implemented by the developer to verify that a given token has the right
/// token id + the correct amount.
#[derive(Clone)]
pub struct TokenSpec {
    predicate: fn(&TokenAmount) -> bool,
}
impl TokenSpec {
    pub fn new(predicate: fn(&TokenAmount) -> bool) -> Self {
        TokenSpec {
            predicate: predicate,
        }
    }
}

/// A specification which specifies parameters of a `ErgoBox`.
/// This spec is used as a "source of truth" to both verify and find
/// `ErgoBox`es which match the spec. This is often used for defining
/// Stages in multi-stage smart contract protocols, but can also be used
/// to define input boxes for Actions.
pub struct BoxSpec {
    /// The P2S smart contract as `ErgoTree`
    pub ergo_tree: ErgoTree,
    /// The allowed range of nanoErgs
    pub value_range: Range<i64>,
    /// A sorted list of `RegisterSpec`s which define registers
    /// of an `ErgoBox`.
    /// First element is treated as R4, second as R5, and so on.
    pub registers: Vec<RegisterSpec>,
    /// A sorted list of `TokenSpec`s which define tokens
    /// of an `ErgoBox`.
    pub token_predicates: Vec<TokenSpec>,
}
