use ergo_lib::chain::ergo_box::ErgoBox;
use ergo_protocol_framework::*;

#[derive(Debug, Clone, WrapBox)]
pub struct MathProblemBox {
    ergo_box: ErgoBox,
}

impl SpecifiedBox for MathProblemBox {
    fn box_spec() -> BoxSpec {
        let address = Some("94hWSMqgxHtRNEWoKrJFGVNQEYX34zfX68FNxWr".to_string());
        BoxSpec::new(address, None, vec![], vec![])
    }
}

impl MathProblemBox {
    pub fn new(ergo_box: &ErgoBox) -> Option<MathProblemBox> {
        // Using the automatically implemented `verify_box` method
        // from the `BoxSpec` to verify the `ErgoBox` is a valid
        // `MathProblemBox`.
        Self::box_spec().verify_box(ergo_box).ok()?;

        // Creating the `MathProblemBox`
        let math_problem_box = MathProblemBox {
            ergo_box: ergo_box.clone(),
        };

        // Returning the `MathProblemBox`
        Some(math_problem_box)
    }
}
