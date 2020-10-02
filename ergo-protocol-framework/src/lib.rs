pub mod predicated_boxes;
pub mod protocol;
pub mod stage;

pub use stage::{Stage, StageType, BoxVerificationError, Result};
pub use protocol::Protocol;
pub use ergo_offchain_utilities::P2SAddressString;