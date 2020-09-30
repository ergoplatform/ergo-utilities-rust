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
    #[error("The P2S address of the box does not match the `Stage` P2S address.")]
    InvalidP2SAddress,
    #[error("The number of Ergs held within the box is outside of the valid range.")]
    InvalidErgsValue,
}

/// A predicate which takes a `Constant` value from an `ErgoBox` register and
/// evaluates the validity of said value. This is a function which is
/// implemented by the developer to verify that a given register holds data
/// which is allowed within the protocol.
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
/// evaluates the validity of said tokens. This predicae is a function which
/// is implemented by the developer to verify that a given token has the right
/// token id + the correct amount.
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

/// A struct which represents a `Stage` in a multi-stage smart contract
/// protocol. This struct defines all of the key essentials of a stage
/// and thus provides an interface for importing boxes and thus performing
/// a validation check that a given box is indeed a valid input to be
/// used in any actions.
struct Stage {
    /// The P2S smart contract address of the Stage
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
    pub tokens_predicates: Vec<TokenPredicate>,
    /// Values which are hardcoded within the smart contract and need
    /// to be used when performing Actions in the protocol.
    pub hardcoded_values: HashMap<String, Constant>,
    /// Boxes which have been imported into our `Stage` struct that
    /// have passed all validation checks.
    pub boxes: Vec<ErgoBox>,
}

impl Stage {
    /// Using the `Stage`'s `ergo_tree` field, return the P2S address of the
    /// stage as a Base58 string.
    pub fn get_p2s_address_string(&self) -> P2SAddressString {
        let address = Address::P2S(self.ergo_tree.sigma_serialise_bytes());
        let encoder = AddressEncoder::new(NetworkPrefix::Mainnet);
        encoder.address_to_str(&address)
    }

    /// Verify that a provided `ErgoBox` is indeed at the given `Stage`.
    /// In other words, check that the box is at the right P2S address,
    /// holds Ergs within the correct range, hold tokens which succeed
    /// all provided predicates, and has values in its registers which
    /// pass all of the register predicates.
    pub fn verify_box(&self, b: &ErgoBox) -> Result<bool> {
        // Verify box P2S Address
        let address = Address::P2S(b.ergo_tree.sigma_serialise_bytes());
        let encoder = AddressEncoder::new(NetworkPrefix::Mainnet);
        let address_check = match self.get_p2s_address_string() == encoder.address_to_str(&address)
        {
            true => Ok(true),
            false => Err(BoxVerificationError::InvalidP2SAddress),
        };

        // Verify value held in the box is within the valid range
        let value_within_range = match self.value_range.contains(b.value.as_i64()) {
            true => Ok(true),
            false => Err(BoxVerificationError::InvalidErgsValue),
        };

        Ok(address_check? && value_within_range?)
    }

    /// First verifies whether the provided box is indeed at the given `Stage`
    /// using `verify_box()`, then if it succeeds, adds the box to the
    /// `Stage`'s `boxes` field.
    /// On success returns the number of boxes currently held by the struct.
    pub fn import_box(ergo_box: &ErgoBox) {
        // -> Result<u64> {
        todo!();
    }
}
