# Ergo Node Interface

A Rust library for interacting with an Ergo Node. Uses [Ergo-Lib](https://github.com/ergoplatform/sigma-rust) for the `ErgoBox` struct and doing the majority of the encoding/decoding.

This library currently provides:
1. Core Ergo Node endpoints for writing off-chain dApps.
2. Helper functions on top of #1 which simplifies a dApp developers life.
3. A higher level interface for UTXO-set scanning.


The library does not currently support 100% of all Ergo Node endpoints, as the current goal is to make the off-chain dApp developer experience as solid as possible. 100% coverage however is indeed a goal for the long-term.















Contributing
------------
If you find a mistake, add a new endpoint, or wish to include a novel feature, please feel free to submit a PR.