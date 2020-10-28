// // Intended use:
// // 1. Create an empty struct with the name of your Stage.
// // 2. Implement `StageType` trait on your struct.
// // 3. Create a `Stage` struct using Stage::new()
// // 4. Use `verify_box()` to create verified `StageBox<T:StageType>`s. These represent boxes that are guaranteed to valid boxes at a given stage, and thus can be used for performing Actions without any further checks.
// // 5. Write functions that represent Actions in your protocol using `StageBox<t>`s for the inputs and output types to guarantee that your Action(state transition) logic is valid.

// use crate::predicated_boxes::StageBox;
// use crate::predicated_boxes::{BoxVerificationError, Result};
// use ergo_lib::ast::Constant;
// use ergo_lib::chain::address::{Address, AddressEncoder, NetworkPrefix};
// pub use ergo_lib::chain::ergo_box::{ErgoBox, ErgoBoxCandidate};
// use ergo_lib::chain::transaction::TxId;
// use ergo_lib::serialization::serializable::SigmaSerializable;
// use ergo_lib::ErgoTree;
// use ergo_offchain_utilities::P2SAddressString;
// use std::collections::HashMap;

// /// A trait for defining the datatype (effectively the name
// /// on the type level) of your `Stage` in your off-chain code.
// pub trait StageType {
//     /// Create a new `StageType`
//     fn new() -> Self;
// }

// // A struct which represents a `Stage` in a
// // multi-stage smart contract protocol. This struct defines all of the key
// // essentials and thus provides an interface for performing
// // validation checks that a given `ErgoBox` is indeed at said stage.
// #[derive(Clone)]
// pub struct Stage<ST: StageType> {
//     /// Hardcoded values within the `Stage` contract
//     pub hardcoded_values: HashMap<String, Constant>,
//     /// The P2S Address of the `Stage` as a base58 `String`
//     pub ergo_tree: ErgoTree,
//     /// A predicate that an `ErgoBox` must pass in order to be classified
//     /// as being at the current `Stage`. This predicate can check
//     /// any data within the ErgoBox matches given requirements.
//     pub verification_predicate: fn(&ErgoBox) -> Result<()>,
//     /// The `Stage` data type that this `StageChecker` is created for.
//     /// Only used for carrying the type forward into this struct and
//     /// for any `StageBox<T>`s created.
//     stage_type: ST,
// }

// impl<ST: StageType> Stage<ST> {
//     /// Create a new Stage<ST>
//     pub fn new(
//         hardcoded_values: HashMap<String, Constant>,
//         p2s_address: &P2SAddressString,
//         verification_predicate: fn(&ErgoBox) -> Result<()>,
//     ) -> Result<Stage<ST>> {
//         let encoder = AddressEncoder::new(NetworkPrefix::Mainnet);
//         let address = encoder
//             .parse_address_from_str(&p2s_address)
//             .map_err(|_| BoxVerificationError::InvalidP2SAddress)?;
//         let ergo_tree = ErgoTree::sigma_parse_bytes(address.content_bytes())
//             .map_err(|_| BoxVerificationError::InvalidP2SAddress)?;
//         Ok(Stage {
//             hardcoded_values: hardcoded_values,
//             ergo_tree: ergo_tree.clone(),
//             verification_predicate: verification_predicate,
//             stage_type: ST::new(),
//         })
//     }

//     /// Acquire the base58 encoded P2S Address from the stage `ErgoTree`
//     pub fn get_p2s_address(&self) -> P2SAddressString {
//         let address = Address::P2S(self.ergo_tree.sigma_serialise_bytes());
//         let encoder = AddressEncoder::new(NetworkPrefix::Mainnet);
//         encoder.address_to_str(&address)
//     }

//     /// Verify that a provided `ErgoBox` is indeed at the given `Stage`.
//     /// In other words, check that the box is at the right P2S address,
//     /// holds Ergs within the correct range, hold tokens which succeed
//     /// all provided predicates, and has values in its registers which
//     /// pass all of the register predicates.
//     pub fn verify_box(&self, b: &ErgoBox) -> Result<StageBox<ST>> {
//         // Verify box `ErgoTree`
//         match self.ergo_tree == b.ergo_tree {
//             true => Ok(()),
//             false => Err(BoxVerificationError::InvalidErgoTree),
//         }?;

//         // Apply verification predicate to the `ErgoBox`. If it returns
//         // an error, then the `?` will prevent the function from proceeding
//         let stage_box = StageBox::new(b, self.verification_predicate, ST::new())?;

//         Ok(stage_box)
//     }

//     /// Verify that a provided `ErgoBoxCandidate` passes the verification
//     /// predicate. This is primarily used within `Actions` defined as methods
//     /// which create new `ErgoBoxCandidate`s which need to be checked to be
//     /// valid boxes at a given `Stage<ST>`.
//     pub fn verify_box_candidate(&self, bc: &ErgoBoxCandidate) -> Result<StageBox<ST>> {
//         let processed_box = &ErgoBox::from_box_candidate(bc, TxId::zero(), 0);
//         self.verify_box(processed_box)
//     }
// }
