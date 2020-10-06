/// This file holds a number of default "Predicated Boxes".
/// These are wrapper structs for `ErgoBox`es which have predicates
/// applied to them on creation of said struct.
/// The name of the struct provides a more concrete development
/// experience when writing `Actions` with very specific input types
/// which are enforced by the predicates inside of each predicated
/// box.
use crate::stage::{BoxVerificationError, Result, StageType};
pub use ergo_lib::chain::ergo_box::ErgoBox;

pub trait PredicatedBox {
    fn predicate(&self) -> fn(&ErgoBox) -> Result<()>;
    fn get_box(&self) -> ErgoBox;
}

/// A predicated box which has been verified to be at a
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

/// A predicated box which is is intended to be spent for the Ergs inside
/// The predicate simply requires the box to simply have more than `1000000`
/// nanoErgs inside.
pub struct ErgsBox {
    pub ergo_box: ErgoBox,
    pub predicate: fn(&ErgoBox) -> Result<()>,
}
/// Predicate to check that a box has more than `1000000` nanoErgs
fn box_with_ergs_predicate(b: &ErgoBox) -> Result<()> {
    if b.value.as_u64() > 1000000 {
        Ok(())
    } else {
        Err(BoxVerificationError::InvalidErgsValue(
            "ErgoBox did not have more than 999999 nanoErgs inside.".to_string(),
        ))
    }
}
impl PredicatedBox for ErgsBox {
    /// Empty predicate that always passes.
    fn predicate(&self) -> fn(&ErgoBox) -> Result<()> {
        box_with_ergs_predicate
    }
    fn get_box(&self) -> ErgoBox {
        self.ergo_box.clone()
    }
}
impl ErgsBox {
    /// Create a new `NoPredicateBox`
    pub fn new(b: &ErgoBox) -> Result<ErgsBox> {
        box_with_ergs_predicate(b)?;
        return Ok(ErgsBox {
            ergo_box: b.clone(),
            predicate: box_with_ergs_predicate,
        });
    }
}
