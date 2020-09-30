pub use sigma_tree::ast::Constant;
pub use sigma_tree::chain::ergo_box::ErgoBox;
pub use sigma_tree::chain::token::TokenAmount;
use std::ops::Range;

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
    p2s_address: String,
    /// The allowed range of nanoErgs to be held in a box at the given stage
    nano_ergs_range: Range<i64>,
    /// A sorted list of `RegisterPredicate`s which are used to
    /// evaluate values within registers of a box.
    /// First predicate will be used for R4, second for R5, and so on.
    register_predicates: Vec<RegisterPredicate>,
    /// A sorted list of `TokenPredicate`s which are used to
    /// evaluate `TokenAmount`s in an `ErgoBox`.
    /// First predicate will be used for the first `TokenAmount`, second for
    /// the second `TokenAmount`, and so on.
    tokens_predicates: Vec<TokenPredicate>,

    /// Boxes which have been imported into our `Stage` struct that
    /// have passed all validation checks.
    boxes: Vec<ErgoBox>,
}

// - Hardcoded Values
