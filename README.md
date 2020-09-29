# Ergo Utilities
General utilities to make writing off-chain Ergo code in Rust simpler.

This library was born from abstracting out reusable components from the [Oracle Core](https://github.com/ergoplatform/oracle-core/). As such, it does not currently aim to have full support for all relevant features. Features are currently added on an "as-needed" basis, though in the future building this out into a stable library may make sense.

## Current Modules

### node_interface
This module defines a `NodeInterface` struct which allows you to interface with an Ergo
Node via Rust.

Example basic usage with a node api key of `hello`, and ip of `0.0.0.0` and a port of `9053`:

```rust
use ergo_utilities::NodeInterface;

let node_interface = NodeInterface::new("hello", "0.0.0.0", "9053");
node_interface.current_block_height();
```

### scanning
This module defines a `Scan` struct which allows you to easily register, manually add, and interact
with UTXO-set scans with an Ergo Node.

Example basic usage with an already registered scan with an id of `21` and a name `My Scan`:

```rust
use ergo_utilities::Scan;

let my_scan = Scan::new("My Scan", "21", &node_interface);
my_scan.get_boxes();
```

### encoding
This module provides an interface for serializing, deserializing, hashing, and converting various values which are useful for writing off-chain code for a dApp.

Example basic usage:

```rust
use ergo_utilities::encoding;

// Serializes a `i32` Int value into a hex-encoded string to be used inside of a register for a box
let encoded_int = encoding::serialize_int(25);

let nano_ergs = encoding::erg_to_nanoerg(102.732);
```



## Documentation
For documentation run:


```rust
cargo doc --open
```