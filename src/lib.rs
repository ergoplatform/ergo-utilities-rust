/// General utilities to make writing off-chain Ergo code in Rust simpler.
#[macro_use]
extern crate json;
pub mod encoding;
pub mod node_interface;
pub mod scanning;

pub use node_interface::NodeInterface;
pub use scanning::Scan;

/// A Base58 encoded String of a Ergo P2PK address. Using this type def until sigma-rust matures further with the actual Address type.
pub type P2PKAddress = String;
/// A Base58 encoded String of a Ergo P2S address. Using this type def until sigma-rust matures further with the actual Address type.
pub type P2SAddress = String;
/// Transaction ID
pub type TxId = String;
/// The smallest unit of the Erg currency.
pub type NanoErg = u64;
/// A block height of the chain.
pub type BlockHeight = u64;
/// Duration in number of blocks.
pub type BlockDuration = u64;
/// A Base58 encoded String of a Token ID.
pub type TokenID = String;
/// Integer which is provided by the Ergo node to reference a given scan.
pub type ScanID = String;
