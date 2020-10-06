/// This file holds a number of default "Predicated Boxes".
/// These are wrapper structs for `ErgoBox`es which have predicates
/// applied to them on creation of said struct.
/// The name of the struct provides a more concrete development
/// experience when writing `Actions` with very specific input types
/// which are enforced by the predicates inside of each predicated
/// box.
use crate::stage::StageType;
pub use ergo_lib::ast::ConstantVal;
use ergo_lib::chain::ergo_box::ErgoBox;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, BoxVerificationError>;

#[derive(Error, Debug)]
pub enum BoxVerificationError {
    #[error("The P2S address of the box does not match the `StageChecker` P2S address.")]
    InvalidP2SAddress,
    #[error("The number of Ergs held within the box is invalid: {0}")]
    InvalidErgsValue(String),
    #[error("The provided `ErgoBox` did not pass the verification predicate because of a problem with the tokens held in the box: {0}")]
    InvalidTokens(String),
    #[error("The provided `ErgoBox` did not pass the verification predicate because of a problem with the values within the registers of the box: {0}")]
    InvalidRegisters(String),
    #[error("The provided `ErgoBox` is not a valid Oracle Box: {0}")]
    InvalidOracleBox(String),
    #[error("{0}")]
    OtherError(String),
}

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
    ergo_box: ErgoBox,
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
    ergo_box: ErgoBox,
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

/// A predicated box which indicates it is an
/// oracle box which stores a `Long` integer datapoint inside of R4.
/// This may be an Oracle Pool box, or any other kind of oracle box.
/// This predicated box automatically extracts the long datapoint from the
/// box and exposes it as a public field to be easily used.
pub struct OracleBoxLong {
    ergo_box: ErgoBox,
    pub predicate: fn(&ErgoBox) -> Result<()>,
    pub datapoint: i64,
}
/// Extracts a Long out of register R4 of the provided `ErgoBox`.
/// Does error-checking along the way.
fn extract_long_datapoint(b: &ErgoBox) -> Result<i64> {
    let registers = b.additional_registers.get_ordered_values();
    if registers.len() < 1 {
        return Err(BoxVerificationError::InvalidOracleBox(
            "No datapoint in R4.".to_string(),
        ));
    } else {
        // Match on the ConstantVal::Long of Register R4
        match registers[0].v {
            ConstantVal::Long(i) => return Ok(i),
            _ => {
                return Err(BoxVerificationError::InvalidOracleBox(
                    "Value in R4 is not a Long.".to_string(),
                ))
            }
        };
    }
}
/// Predicate to check that a box has a valid Long datapoint in R4.
fn oracle_box_predicate(b: &ErgoBox) -> Result<()> {
    // Using `?` to verify that a valid Long datapoint was extracted.
    // If it failed, it will push the error upwards.
    extract_long_datapoint(b)?;
    Ok(())
}
impl PredicatedBox for OracleBoxLong {
    /// Empty predicate that always passes.
    fn predicate(&self) -> fn(&ErgoBox) -> Result<()> {
        oracle_box_predicate
    }
    fn get_box(&self) -> ErgoBox {
        self.ergo_box.clone()
    }
}
impl OracleBoxLong {
    /// Create a new `NoPredicateBox`
    pub fn new(b: &ErgoBox) -> Result<OracleBoxLong> {
        // This automatically does error/predicate checking while extracting.
        let datapoint = extract_long_datapoint(b)?;
        return Ok(OracleBoxLong {
            ergo_box: b.clone(),
            predicate: oracle_box_predicate,
            datapoint: datapoint,
        });
    }
}
