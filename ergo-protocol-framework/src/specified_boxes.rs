/// This file holds a number of default general "Specified Boxes".
/// These are wrapper structs for `ErgoBox`es which meet a given
/// specification and provide you with a simple interface
/// for implementing Actions of your protocols.
use crate::box_spec::BoxSpec;
use crate::box_traits::{SpecifiedBox, WrappedBox};
use crate::error::{ProtocolFrameworkError, Result};
use ergo_lib::chain::ergo_box::ErgoBox;
use ergo_lib_wasm::box_coll::ErgoBoxes;
use ergo_lib_wasm::ergo_box::ErgoBox as WErgoBox;
use ergo_protocol_framework_derive::{SpecBox, WrapBox};
use wasm_bindgen::prelude::*;

/// A specified box which is intended to be spent for the Ergs inside.
/// The spec simply requires the box to simply have at least `1000000`
/// nanoErgs inside.
#[wasm_bindgen]
#[derive(Clone, Debug, WrapBox, SpecBox)]
pub struct ErgsBox {
    ergo_box: ErgoBox,
}
/// SpecifiedBox impl
impl SpecifiedBox for ErgsBox {
    /// A simple `BoxSpec` that just checks the value of nanoErgs is
    /// above `1000000`
    fn box_spec() -> BoxSpec {
        BoxSpec::new(None, Some(1000000..u64::MAX), vec![], vec![])
    }
}
/// WASM-compatible ErgsBox Methods
#[wasm_bindgen]
impl ErgsBox {
    /// Create a new `ErgsBox`
    #[wasm_bindgen(constructor)]
    pub fn w_new(wb: WErgoBox) -> std::result::Result<ErgsBox, JsValue> {
        let b: ErgoBox = wb.into();
        ErgsBox::new(&b).map_err(|e| JsValue::from_str(&format! {"{:?}", e}))
    }
}
/// Rust ErgsBox Methods
impl ErgsBox {
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
