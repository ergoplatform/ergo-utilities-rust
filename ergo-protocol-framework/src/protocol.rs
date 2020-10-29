// Stray idea for how to potentially create a `Protocol` struct.
// Likely a bad idea because having specific structs for each type
// of box like `OracleBoxLong` with a `BoxSpec` inside is much more
// powerful.

// use crate::box_spec::BoxSpec;
// use std::collections::HashMap;

// struct Protocol {
//     name: String,
//     box_specs: HashMap<String, BoxSpec>,
//     actions: Vec<Action>,
// }

// struct Action {
//     input_box_specs: Vec<BoxSpec>,
// }
