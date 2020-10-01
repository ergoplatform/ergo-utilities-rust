use crate::stage::Result;
use crate::stage::StageType;
pub use sigma_tree::chain::ergo_box::ErgoBox;

pub trait PredicatedBox {
    fn predicate(&self) -> fn(&ErgoBox) -> Result<()>;
    fn get_box(&self) -> ErgoBox;
}

/// A wrapper struct for `ErgoBox`es which have been verified to be at a
/// given stage. A `StageBox<T:StageType>` provides a guarantee at the type
/// level that said StageBox can be used as input safely in an Action.
/// The only creation method is provided on the `Stage` struct.
#[derive(Clone)]
pub struct StageBox<ST: StageType> {
    pub ergo_box: ErgoBox,
    pub predicate: fn(&ErgoBox) -> Result<()>,
    pub stage: ST,
}
impl<ST: StageType> PredicatedBox for StageBox<ST> {
    fn predicate(&self) -> fn(&ErgoBox) -> Result<()> {
        self.predicate
    }
    fn get_box(&self) -> ErgoBox {
        self.ergo_box.clone()
    }
}

/// A wrapper struct for `ErgoBox`es which have no predicate. In other
/// words, these are boxes which have not undergone any validity checks
/// and may or may not be valid for your given use case.
#[derive(Debug, Clone)]
pub struct NoPredicateBox {
    ergo_box: ErgoBox,
}
impl PredicatedBox for NoPredicateBox {
    /// Empty predicate that always passes.
    fn predicate(&self) -> fn(&ErgoBox) -> Result<()> {
        |_: &ErgoBox| Ok(())
    }
    fn get_box(&self) -> ErgoBox {
        self.ergo_box.clone()
    }
}
impl NoPredicateBox {
    /// Create a new `NoPredicateBox`
    pub fn new(b: &ErgoBox) -> NoPredicateBox {
        NoPredicateBox {
            ergo_box: b.clone(),
        }
    }
}
