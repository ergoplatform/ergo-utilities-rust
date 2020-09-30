// Intended use:
// 1. Create an empty struct with the name of your Stage.
// 2. Implement `StageType` trait on your struct.
// 3. Create a `Stage` struct using Stage::new()
// 4. Use `verify_box()` to create verified `StageBox<T:StageType>`s. These represent boxes that are guaranteed to valid boxes at a given stage, and thus can be used for performing Actions without any further checks.
// 5. Write functions that represent Actions in your protocol using `StageBox<t>`s for the inputs and output types to guarantee that your Action(state transition) logic is valid.

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

/// A trait for defining the datatype (effectively the name
/// on the type level) of your `Stage` in your off-chain code.
trait StageType {
    /// Create a new `StageType`
    fn new() -> Self;
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

/// A wrapper struct for `ErgoBox`es which have been verified to be at a
/// given stage. A `StageBox<T:StageType>` provides a guarantee at the type
/// level that said StageBox can be used as input safely in an Action.
struct StageBox<T: StageType> {
    pub ergo_box: ErgoBox,
    stage: T,
}

// A struct which represents a `Stage` in a
// multi-stage smart contract protocol. This struct defines all of the key
// essentials and thus provides an interface for performing
// validation checks that a given `ErgoBox` is indeed at said stage.
struct Stage<ST: StageType> {
    /// Hardcoded values within the `Stage` contract
    pub hardcoded_values: HashMap<String, Constant>,
    /// The P2S smart contract address of the StageChecker
    pub ergo_tree: ErgoTree,
    /// The allowed range of nanoErgs to be held in a box at this stage
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
    /// The `Stage` data type that this `StageChecker` is created for.
    /// Only used for carrying the type forward into this struct and
    /// for any `StageBox<T>`s created.
    stage_type: ST,
}

impl<T: StageType> Stage<T> {
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
    pub fn verify_box(&self, b: &ErgoBox) -> Result<StageBox<T>> {
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

        let stage_box = StageBox {
            stage: T::new(),
            ergo_box: b.clone(),
        };

        Ok(stage_box)
    }
}
