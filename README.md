# Ergo Utilities
General utilities to make writing off-chain Ergo code in Rust simpler.

This library was born from abstracting out reusable components from the [Oracle Core](https://github.com/ergoplatform/oracle-core/). As such, it does not support currently aim to have full support for all relevant features. Features are currently added on an "as-needed" basis, though in the future building this out into a stable library may make sense.

## Current Modules

### node_interface
This module defines a `NodeInterface` struct which allows you to interface with an Ergo
Node via Rust.

Example basic usage:

```rust
use ergo_utilities::NodeInterface;

let node_interface = NodeInterface::new("hello", "0.0.0.0", "9053");
node_interface.current_block_height();
```

### scanning
This module defines a `Scan` struct which allows you to easily register and interact
with UTXO-set scans with an Ergo Node.

Example basic usage:

```rust
use ergo_utilities::Scan;

let my_scan = Scan::new("My Scan", "21", &node_interface);
my_scan.get_boxes();
```



## Documentation
For documentation run:


```rust
cargo doc --open
```