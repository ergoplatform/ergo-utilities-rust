/// This file holds a number of default "Predicated Boxes".
/// These are wrapper structs for `ErgoBox`es which have predicates
/// applied to them on creation of said struct.
/// The name of the struct provides a more concrete development
/// experience when writing `Actions` with very specific input types
/// which are enforced by the predicates inside of each predicated
/// box.
pub use ergo_lib::ast::ConstantVal;
use ergo_lib::chain::ergo_box::ErgoBox;
use ergo_lib::chain::input::UnsignedInput;
use ergo_lib_wasm::ergo_box::ErgoBox as WErgoBox;
pub use std::result::Result;
use thiserror::Error;
use wasm_bindgen::prelude::*;

#[derive(Error, Debug)]
pub enum BoxVerificationError {
    #[error("The P2S address is invalid.")]
    InvalidP2SAddress,
    #[error("The ErgoTree is invalid.")]
    InvalidErgoTree,
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

/// A predicated box which is is intended to be spent for the Ergs inside
/// The predicate simply requires the box to simply have more than `1000000`
/// nanoErgs inside.
// #[wasm_bindgen]
#[wasm_bindgen]
pub struct ErgsBox {
    ergo_box: ErgoBox,
}

/// WASM OracleBoxLong Methods
#[wasm_bindgen]
impl ErgsBox {
    /// Create a new `NoPredicateBox`
    #[wasm_bindgen]
    pub fn w_new(wb: WErgoBox) -> Result<ErgsBox, JsValue> {
        let b: ErgoBox = wb.into();
        ErgsBox::new(&b).map_err(|e| JsValue::from_str(&format! {"{:?}", e}))
    }
}

/// Rust OracleBoxLong Methods
impl ErgsBox {
    /// Create a new `NoPredicateBox`
    pub fn new(b: &ErgoBox) -> Result<ErgsBox, BoxVerificationError> {
        box_with_ergs_predicate(b)?;
        return Ok(ErgsBox {
            ergo_box: b.clone(),
        });
    }
    pub fn get_box(&self) -> ErgoBox {
        self.ergo_box.clone()
    }
}

/// Predicate to check that a box has more than `1000000` nanoErgs
fn box_with_ergs_predicate(b: &ErgoBox) -> Result<(), BoxVerificationError> {
    if b.value.as_u64() >= 1000000 {
        Ok(())
    } else {
        Err(BoxVerificationError::InvalidErgsValue(
            "ErgoBox did not have more than 999999 nanoErgs inside.".to_string(),
        ))
    }
}

/// Sums the nanoErg value of a list of `ErgsBox`es
pub fn sum_ergs_boxes_value(boxes: &Vec<ErgsBox>) -> u64 {
    boxes
        .into_iter()
        .fold(0, |acc, pb| pb.get_box().value.as_u64() + acc)
}

/// Unwraps a list of `ErgsBox`es into `Vec<ErgoBox>`
pub fn unwrap_ergs_boxes(boxes: &Vec<ErgsBox>) -> Vec<ErgoBox> {
    boxes.into_iter().map(|pb| pb.get_box()).collect()
}

/// Converts a list of `ErgsBox`es into `Vec<UnsignedInput>`
pub fn ergs_boxes_to_inputs(boxes: &Vec<ErgsBox>) -> Vec<UnsignedInput> {
    boxes.into_iter().map(|pb| pb.get_box().into()).collect()
}

/// A predicated box which indicates it is an
/// oracle box which stores a `Long` integer datapoint inside of R4.
/// This may be an Oracle Pool box, or any other kind of oracle box.
/// This predicated box automatically extracts the long datapoint from the
/// box and exposes it as a public field to be easily used.
/// The predicate also checks that the box has a single type of Token
/// and said token has a value of 1. (Checking that it has an NFT)
#[wasm_bindgen]
pub struct OracleBoxLong {
    ergo_box: ErgoBox,
    pub datapoint: i64,
    /// The token id of the oracle's NFT
    nft_id: String,
}

/// WASM OracleBoxLong Methods
#[wasm_bindgen]
impl OracleBoxLong {
    #[wasm_bindgen(constructor)]
    pub fn w_new(wb: WErgoBox) -> Result<OracleBoxLong, JsValue> {
        let b: ErgoBox = wb.into();
        OracleBoxLong::new(&b).map_err(|e| JsValue::from_str(&format! {"{:?}", e}))
    }

    #[wasm_bindgen(getter)]
    pub fn nft_id(&self) -> String {
        self.nft_id.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_nft_id(&mut self, nft_id: String) {
        self.nft_id = nft_id.clone();
    }
}

/// Rust OracleBoxLong Methods
impl OracleBoxLong {
    // Create a new `OracleBoxLong`
    pub fn new(b: &ErgoBox) -> Result<OracleBoxLong, BoxVerificationError> {
        // Error Checking
        oracle_box_predicate(&b)?;
        let datapoint = extract_long_datapoint(&b)?;
        return Ok(OracleBoxLong {
            ergo_box: b.clone(),
            datapoint: datapoint,
            nft_id: b.tokens[0].token_id.0.clone().into(),
        });
    }

    pub fn get_box(&self) -> ErgoBox {
        self.ergo_box.clone()
    }
}

/// Extracts a Long out of register R4 of the provided `ErgoBox`.
/// Does error-checking along the way.
fn extract_long_datapoint(b: &ErgoBox) -> Result<i64, BoxVerificationError> {
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
fn oracle_box_predicate(b: &ErgoBox) -> Result<(), BoxVerificationError> {
    // Using `?` to verify that a valid Long datapoint was extracted.
    // If it failed, it will push the error upwards.
    extract_long_datapoint(b)?;

    // Check only a single token type is held in the box
    if b.tokens.len() != 1 {
        return Err(BoxVerificationError::InvalidTokens(
            "The oracle box is required to only hold a single NFT token.".to_string(),
        ));
    }
    // Check that said single type of token is value == 1. (Aka is an NFT)
    if b.tokens[0].amount != 1 {
        return Err(BoxVerificationError::InvalidTokens(
            "The oracle box is required to only hold a single NFT token.".to_string(),
        ));
    }
    Ok(())
}
