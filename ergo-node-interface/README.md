# Ergo Node Interface Library

A Rust library for interacting with an Ergo Node. Uses [Ergo-Lib](https://github.com/ergoplatform/sigma-rust) for the `ErgoBox` struct and doing the majority of the encoding/decoding.

This library currently provides:
1. Core Ergo Node endpoints for writing off-chain dApps.
2. Helper functions on top of #1 which simplifies a dApp developers life.
3. A higher level interface for UTXO-set scanning.

The library does not currently support 100% of all Ergo Node endpoints, as the current goal is to make the off-chain dApp developer experience as solid as possible. 100% coverage however is indeed a goal for the long-term.


Modules
========

The below are the currently implemented modules part of the library.

Node Interface
--------------
This module contains the core `NodeInterface` struct which is used to interact with an Ergo Node. All endpoints are implemented as methods for the `NodeInterface` struct.


```rust
let node = NodeInterface::new(api_key, ip, port);
println!("Current height: {}", node.current_block_height());
```

Furthermore a number of helper methods are implemented as well, such as:

```rust
/// A CLI interactive interface for prompting a user to select an address
pub fn select_wallet_address(&self) -> Result<P2PKAddressString>


/// Returns a sorted list of unspent boxes which cover at least the
/// provided value `total` of nanoErgs.
/// Note: This box selection strategy simply uses the largest
/// value holding boxes from the user's wallet first.
pub fn unspent_boxes_with_min_total(&self, total: NanoErg) -> Result<Vec<ErgoBox>>

```

Scanning
---------
This module contains the `Scan` struct which allows a developer to easily work with UTXO-set scans. Each `Scan` is tied to a specific `NodeInterface`, which matches the reality as scans are saved on a per-node basis.

The `Scan` struct provides you with the ability to easily:
1. Register new scans with your Ergo Node.
2. Acquire boxes/serialized boxes from your registered scans.
3. Save/read scan ids to/from a local file.

Example using the scanning interface to register a scan to track an Oracle Pool:

```rust
let oracle_pool_nft_id = "08b59b14e4fdd60e5952314adbaa8b4e00bc0f0b676872a5224d3bf8591074cd".to_string();

let tracking_rule = object! {
        "predicate": "containsAsset",
        "assetId": oracle_pool_nft_id,
};

let scan = Scan::register(
    &"Oracle Pool Box Scan".to_string(),
    tracking_rule,
    node,
).unwrap();

```


Local Config
------------
This module provides a few helper functions to save/read from a local `node-interface.yaml` file which holds the Ergo Node ip/port/api key. This makes it much quicker for a dApp developer to get their dApp running without having to manually implement such logic himself.

Example functions which are available:

```rust
/// Create a new `node-interface.config` with the barebones yaml inside
pub fn create_new_local_config_file() -> Result<()>

/// Opens a local `node-interface.yaml` file and uses the
/// data inside to create a `NodeInterface`
pub fn new_interface_from_local_config() -> Result<NodeInterface> {
```






Contributing
============
If you find a mistake, want to add a new endpoint, or wish to include a novel feature, please feel free to submit a PR.