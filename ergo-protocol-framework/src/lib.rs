pub mod predicated_boxes;
pub mod protocol;
pub mod stage;

pub use stage::{Stage, StageType, BoxVerificationError, Result};
pub use protocol::Protocol;
pub use ergo_offchain_utilities::P2SAddressString;
pub use ergo_lib::chain::ergo_box::ErgoBox;
pub use ergo_lib::ast::Constant;