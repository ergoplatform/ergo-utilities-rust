use crate::box_spec::{BoxSpec, Result};
use crate::box_traits::{SpecifiedBox, WrappedBox};
use ergo_lib::ast::ConstantVal;
use ergo_lib::chain::ergo_box::{BoxValue, ErgoBox, ErgoBoxCandidate, NonMandatoryRegisters};
use ergo_lib::chain::token::{Token, TokenAmount, TokenId};
use ergo_lib_wasm::box_coll::ErgoBoxes;
use ergo_lib_wasm::ergo_box::ErgoBox as WErgoBox;
use ergo_offchain_utilities::{BlockHeight, ErgoAddressString, NanoErg, P2PKAddressString};
use thiserror::Error;
use wasm_bindgen::prelude::*;

/// A predicated box which is intended to be spent for the Ergs inside
/// The spec simply requires the box to simply have at least `1000000`
/// nanoErgs inside.
#[wasm_bindgen]
pub struct ErgsBox {
    ergo_box: ErgoBox,
}
/// WASM ErgsBox Methods
#[wasm_bindgen]
impl ErgsBox {
    /// Create a new `ErgsBox`
    #[wasm_bindgen(constructor)]
    pub fn w_new(wb: WErgoBox) -> std::result::Result<ErgsBox, JsValue> {
        let b: ErgoBox = wb.into();
        ErgsBox::new(&b).map_err(|e| JsValue::from_str(&format! {"{:?}", e}))
    }
}
/// WrappedBox impl
impl WrappedBox for ErgsBox {
    fn get_box(&self) -> ErgoBox {
        self.ergo_box.clone()
    }
}
/// SpecifiedBox impl
impl SpecifiedBox for ErgsBox {
    fn box_spec() -> BoxSpec {
        BoxSpec::new(None, Some(1000000..10000000000000000000), vec![], vec![])
    }
}
/// Rust ErgsBox Methods
impl ErgsBox {
    /// Create a new `ErgsBox`
    pub fn new(b: &ErgoBox) -> Result<ErgsBox> {
        ErgsBox::box_spec().verify_box(b)?;
        return Ok(ErgsBox {
            ergo_box: b.clone(),
        });
    }

    /// Converts from the WASM wrapper `ErgoBoxes` into a vector of
    /// `ErgsBox`es.
    pub fn convert_from_ergo_boxes(ergo_boxes: &ErgoBoxes) -> Result<Vec<ErgsBox>> {
        // Mutable list of `ErgsBox`es
        let mut ergs_boxes: Vec<ErgsBox> = vec![];
        // Unwrapped list of `ErgoBox`es
        let unwrapped_boxes: Vec<ErgoBox> = ergo_boxes.clone().into();
        // Converting all unwrapped `ErgoBox`es into `ErgsBox`es
        for b in unwrapped_boxes {
            let ergs_box = ErgsBox::new(&b)?;
            ergs_boxes.push(ergs_box);
        }
        Ok(ergs_boxes)
    }

    /// Sums the nanoErg value of a list of `ErgsBox`es
    pub fn sum_ergs_boxes_value(boxes: &Vec<ErgsBox>) -> u64 {
        boxes
            .into_iter()
            .fold(0, |acc, pb| pb.get_box().value.as_u64().clone() + acc)
    }
}

// /// A predicated box which indicates it is an
// /// oracle box which stores a `Long` integer datapoint inside of R4.
// /// This may be an Oracle Pool box, or any other kind of oracle box.
// /// This predicated box automatically extracts the long datapoint from the
// /// box and exposes it as a public field to be easily used.
// /// The predicate also checks that the box has a single type of Token
// /// and said token has a value of 1. (Checking that it has an NFT)
// #[wasm_bindgen]
// pub struct OracleBoxLong {
//     ergo_box: ErgoBox,
//     pub datapoint: i64,
//     /// The token id of the oracle's NFT
//     nft_id: String,
// }

// /// WASM OracleBoxLong Methods
// #[wasm_bindgen]
// impl OracleBoxLong {
//     #[wasm_bindgen(constructor)]
//     pub fn w_new(wb: WErgoBox) -> Result<OracleBoxLong, JsValue> {
//         let b: ErgoBox = wb.into();
//         OracleBoxLong::new(&b).map_err(|e| JsValue::from_str(&format! {"{:?}", e}))
//     }

//     #[wasm_bindgen(getter)]
//     pub fn nft_id(&self) -> String {
//         self.nft_id.clone()
//     }
// }

// impl WrappedBox for OracleBoxLong {
//     fn get_box(&self) -> ErgoBox {
//         self.ergo_box.clone()
//     }
// }

// /// PredicatedBox OracleBoxLong Methods
// impl PredicatedBox for OracleBoxLong {
//     /// Predicate to check that a box has a valid Long datapoint in R4.
//     fn predicate(b: &ErgoBox) -> Result<(), BoxVerificationError> {
//         // Using `?` to verify that a valid Long datapoint was extracted.
//         // If it failed, it will push the error upwards.
//         OracleBoxLong::extract_long_datapoint(b)?;

//         // Check only a single token type is held in the box
//         if b.tokens.len() != 1 {
//             return Err(BoxVerificationError::InvalidTokens(
//                 "The oracle box is required to only hold a single NFT token.".to_string(),
//             ));
//         }
//         // Check that said single type of token is value == 1. (Aka is an NFT)
//         if u64::from(b.tokens[0].amount) != 1 {
//             return Err(BoxVerificationError::InvalidTokens(
//                 "The oracle box is required to only hold a single NFT token.".to_string(),
//             ));
//         }
//         Ok(())
//     }
// }

// /// Rust OracleBoxLong Methods
// impl OracleBoxLong {
//     // Create a new `OracleBoxLong`
//     pub fn new(b: &ErgoBox) -> Result<OracleBoxLong, BoxVerificationError> {
//         // Error Checking
//         OracleBoxLong::predicate(&b)?;
//         let datapoint = OracleBoxLong::extract_long_datapoint(&b)?;
//         return Ok(OracleBoxLong {
//             ergo_box: b.clone(),
//             datapoint: datapoint,
//             nft_id: b.tokens[0].token_id.0.clone().into(),
//         });
//     }

//     /// Extracts a Long out of register R4 of the provided `ErgoBox`.
//     /// Does error-checking along the way.
//     fn extract_long_datapoint(b: &ErgoBox) -> Result<i64, BoxVerificationError> {
//         let registers = b.additional_registers.get_ordered_values();
//         if registers.len() < 1 {
//             return Err(BoxVerificationError::InvalidOracleBox(
//                 "No datapoint in R4.".to_string(),
//             ));
//         } else {
//             // Match on the ConstantVal::Long of Register R4
//             match registers[0].v {
//                 ConstantVal::Long(i) => return Ok(i),
//                 _ => {
//                     return Err(BoxVerificationError::InvalidOracleBox(
//                         "Value in R4 is not a Long.".to_string(),
//                     ))
//                 }
//             };
//         }
//     }
// }
