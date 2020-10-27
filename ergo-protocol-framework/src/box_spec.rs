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
    FailedTokenPredicate,
    #[error("The number of register predicates defined for your `StageChecker` are greater than the number of registers used in the box.")]
    LessRegistersThanPredicates,
    #[error("One of the register predicates failed for the provided box.")]
    FailedRegisterPredicate,
}
/// A predicate which takes a `Constant` value from an `ErgoBox` register and
/// evaluates the validity of said value. This is a function which is
/// implemented by the developer to verify that a given register holds data
/// which is allowed within the protocol.
#[derive(Clone)]
pub struct RegisterPredicate {
    predicate: fn(&Constant) -> bool,
}
impl RegisterPredicate {
    pub fn new(predicate: fn(&Constant) -> bool) -> Self {
        RegisterPredicate {
            predicate: predicate,
        }
    }
}

/// A predicate which takes a `TokenAmount` value and
/// evaluates the validity of said tokens. This predicate is a function which
/// is implemented by the developer to verify that a given token has the right
/// token id + the correct amount.
#[derive(Clone)]
pub struct TokenPredicate {
    predicate: fn(&TokenAmount) -> bool,
}
impl TokenPredicate {
    pub fn new(predicate: fn(&TokenAmount) -> bool) -> Self {
        TokenPredicate {
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
    /// A sorted list of `RegisterPredicate`s which are used to
    /// evaluate values within registers of a box.
    /// First predicate will be used for R4, second for R5, and so on.
    pub register_predicates: Vec<RegisterPredicate>,
    /// A sorted list of `TokenPredicate`s which are used to
    /// evaluate `TokenAmount`s in an `ErgoBox`.
    /// First predicate will be used for the first `TokenAmount`, second for
    /// the second `TokenAmount`, and so on.
    pub token_predicates: Vec<TokenPredicate>,
}
