# Ergo Utilities
This repo houses experimental libraries for writing off-chain Ergo code in Rust.


## Current Libraries

### Ergo Node Interface
A Rust interface for interacting with an Ergo Node. This library also provides a higher level wrapping around UTXO-set scanning to make it easier to use.

### Ergo Off-Chain Utilities
Utilities related to encoding/hashing/serialization values/data what is useful for off-chain application.

### Ergo Protocol Framework
A framework which attempts to provide a guided approach to developing the off-chain portion of multi-stage smart contract protocols.

The goal here is to provide developers with an easy experience to go from [Ergo dApp informal specifications](https://github.com/ergoplatform/eips/blob/master/eip-0006.md) to off-chain code with greater assurance and a straightforward path of implementation.