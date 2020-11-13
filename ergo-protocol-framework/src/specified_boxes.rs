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
/// The predicate simply requires the box to simply have at least `1000000`
/// nanoErgs inside.
#[wasm_bindgen]
pub struct ErgsBox {
    ergo_box: ErgoBox,
}

// /// WASM ErgsBox Methods
// #[wasm_bindgen]
// impl ErgsBox {
//     /// Create a new `ErgsBox`
//     #[wasm_bindgen(constructor)]
//     pub fn w_new(wb: WErgoBox) -> Result<ErgsBox, JsValue> {
//         let b: ErgoBox = wb.into();
//         ErgsBox::new(&b).map_err(|e| JsValue::from_str(&format! {"{:?}", e}))
//     }
// }

impl WrappedBox for ErgsBox {
    fn get_box(&self) -> ErgoBox {
        self.ergo_box.clone()
    }
}

impl SpecifiedBox for ErgsBox {
    fn box_spec() -> BoxSpec {
        BoxSpec::new(None, Some(1000000..10000000000000), vec![], vec![])
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

// PredicatedBox ErgsBox Methods
// impl PredicatedBox for ErgsBox {
//     /// Predicate to check that a box has more than `1000000` nanoErgs
//     fn predicate(b: &ErgoBox) -> Result<(), BoxVerificationError> {
//         if b.value.as_u64().clone() >= 1000000 {
//             Ok(())
//         } else {
//             Err(BoxVerificationError::InvalidErgsValue(
//                 "ErgoBox did not have more than 999999 nanoErgs inside.".to_string(),
//             ))
//         }
//     }
// }
