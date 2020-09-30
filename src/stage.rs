use crate::P2SAddressString;
pub use sigma_tree::ast::Constant;
use sigma_tree::chain::address::{Address, AddressEncoder, NetworkPrefix};
pub use sigma_tree::chain::ergo_box::ErgoBox;
pub use sigma_tree::chain::token::TokenAmount;
use sigma_tree::serialization::serializable::SigmaSerializable;
use sigma_tree::ErgoTree;
use std::collections::HashMap;
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
    FailedTokenPredicate,
    #[error("The number of register predicates defined for your `StageChecker` are greater than the number of registers used in the box.")]
    LessRegistersThanPredicates,
    #[error("One of the register predicates failed for the provided box.")]
    FailedRegisterPredicate,
}

/// A trait for defining `Stage`s in your multi-stage smart contract protocol
/// off-chain code.
trait Stage {
    fn new() -> Self;
}

/// A wrapper struct for `ErgoBox`es which have been verified to be at a
/// given stage.
struct StageCheckerBox<T: Stage> {
    stage: T,
    pub ergo_box: ErgoBox,
}

/// A predicate which takes a `Constant` value from an `ErgoBox` register and
/// evaluates the validity of said value. This is a function which is
/// implemented by the developer to verify that a given register holds data
/// which is allowed within the protocol.
#[derive(Clone)]
struct RegisterPredicate {
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
struct TokenPredicate {
    predicate: fn(&TokenAmount) -> bool,
}
impl TokenPredicate {
    pub fn new(predicate: fn(&TokenAmount) -> bool) -> Self {
        TokenPredicate {
            predicate: predicate,
        }
    }
}

/// A struct which represents a specification of a single stage in a
/// multi-stage smart contract protocol. This struct defines all of the key
/// essentials of a stage and thus provides an interface for performing
/// validation checks that a given `ErgoBox` is indeed at said stage.
#[derive(Clone)]
struct StageChecker<T: Stage> {
    pub stage: T,
    /// The P2S smart contract address of the StageChecker
    pub ergo_tree: ErgoTree,
    /// The allowed range of nanoErgs to be held in a box at the given stage
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
    /// Values which are hardcoded within the smart contract and need
    /// to be used when performing Actions in the protocol.
    pub hardcoded_values: HashMap<String, Constant>,
}

impl<T: Stage> StageChecker<T> {
    /// Using the `StageChecker`'s `ergo_tree` field, return the P2S address of the
    /// stage as a Base58 string.
    pub fn get_p2s_address_string(&self) -> P2SAddressString {
        let address = Address::P2S(self.ergo_tree.sigma_serialise_bytes());
        let encoder = AddressEncoder::new(NetworkPrefix::Mainnet);
        encoder.address_to_str(&address)
    }

    /// Verify that a provided `ErgoBox` is indeed at the given `StageChecker`.
    /// In other words, check that the box is at the right P2S address,
    /// holds Ergs within the correct range, hold tokens which succeed
    /// all provided predicates, and has values in its registers which
    /// pass all of the register predicates.
    pub fn verify_box(&self, b: &ErgoBox) -> Result<StageCheckerBox<T>> {
        // Verify box P2S Address
        let address = Address::P2S(b.ergo_tree.sigma_serialise_bytes());
        let encoder = AddressEncoder::new(NetworkPrefix::Mainnet);
        let address_check = match self.get_p2s_address_string() == encoder.address_to_str(&address)
        {
            true => Ok(true),
            false => Err(BoxVerificationError::InvalidP2SAddress),
        }?;

        // Verify value held in the box is within the valid range
        let value_within_range = match self.value_range.contains(b.value.as_i64()) {
            true => Ok(true),
            false => Err(BoxVerificationError::InvalidErgsValue),
        }?;

        // Verify the number of unique tokens is at least equal to the number
        // of token predicates.
        if b.tokens.len() < self.token_predicates.len() {
            return Err(BoxVerificationError::LessTokensThanPredicates);
        }
        // Verify tokens held in box pass all provided predicates
        for i in 0..(self.token_predicates.len() - 1) {
            let token = &b.tokens[i];
            let p = &self.token_predicates[i];
            match (p.predicate)(token) {
                true => (),
                false => return Err(BoxVerificationError::FailedTokenPredicate),
            }
        }

        // Verify the number of used registers is at least equal to the number
        // of register predicates.
        let registers = b.additional_registers.get_ordered_values();
        if registers.len() < self.register_predicates.len() {
            return Err(BoxVerificationError::LessRegistersThanPredicates);
        }
        // Verify registers in box pass all provided predicates
        for i in 0..(self.register_predicates.len() - 1) {
            let register = &registers[i];
            let p = &self.register_predicates[i];
            match (p.predicate)(register) {
                true => (),
                false => return Err(BoxVerificationError::FailedRegisterPredicate),
            }
        }

        let stage_box = StageCheckerBox {
            stage: T::new(),
            ergo_box: b.clone(),
        };

        Ok(stage_box)
    }
}