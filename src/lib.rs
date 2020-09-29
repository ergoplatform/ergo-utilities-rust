#[macro_use]
extern crate json;
pub mod node_interface;
pub use node_interface::NodeInterface;

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
