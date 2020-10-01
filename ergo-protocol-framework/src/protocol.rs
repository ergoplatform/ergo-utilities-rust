use crate::stage::Stage;

pub trait Protocol {}

// Users will create their own structs which implement `Protocol`
// and encode Actions as methods on said struct using
// `ErgoBox`es & `StageBox`es as inputs/outputs.
