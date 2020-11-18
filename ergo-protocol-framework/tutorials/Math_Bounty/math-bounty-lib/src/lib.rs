use ergo_lib::chain::ergo_box::ErgoBox;
use ergo_protocol_framework::*;

#[derive(Debug, Clone, WrapBox)]
pub struct MathProblemBox {
    ergo_box: ErgoBox,
}
