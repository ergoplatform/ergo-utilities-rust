use crate::stage::{Stage, StageType};

pub trait Protocol {
    // fn list_stages() -> Vec<Box<dyn StageType>>;
}

// Users will create their own structs which implement `Protocol`
// and encode Actions as methods on said struct using
// `ErgoBox`es & `StageBox`es as inputs/outputs.

// Predicated Boxes may just be completely useless, or there may be a scenario
// where `Vec<Box<dyn PredicatedBox>>` will be useful.
