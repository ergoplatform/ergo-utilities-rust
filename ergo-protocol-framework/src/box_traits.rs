use crate::box_spec::{BoxSpec, Result};
use ergo_lib::chain::data_input::DataInput;
use ergo_lib::chain::ergo_box::{ErgoBox, ErgoBoxCandidate};
use ergo_lib::chain::input::UnsignedInput;
use ergo_offchain_utilities::encoding::serialize_p2s_from_ergo_tree;
use ergo_offchain_utilities::{NanoErg, P2SAddressString};

/// A trait which represents an `ErgoBox` wrapped in an overarching struct.
pub trait WrappedBox {
    fn get_box(&self) -> ErgoBox;
    // Converts the `WrappedBox` into a `DataInput`
    fn as_data_input(&self) -> DataInput {
        self.get_box().box_id().into()
    }
    // Converts the `WrappedBox` into an `UnsignedInput`
    fn as_unsigned_input(&self) -> UnsignedInput {
        self.get_box().into()
    }
    // Returns the Box ID of the wrapped `ErgoBox` as a base16 String
    fn box_id(&self) -> String {
        self.get_box().box_id().into()
    }
    // Returns the amount of nanoErgs held in the wrapped `ErgoBox` as u64
    fn nano_ergs(&self) -> NanoErg {
        self.get_box().value.as_u64().clone()
    }
    // Returns the P2S Address of wrapped `ErgoBox` as a String
    fn p2s_address(&self) -> P2SAddressString {
        serialize_p2s_from_ergo_tree(self.get_box().ergo_tree)
    }
    // // Returns the registers of the wrapped `ErgoBox`
    // fn registers(&self) -> Vec<Constant>
}
