# Ergo Protocol Framework v0.2.0

A framework which attempts to provide a guided approach to developing the off-chain portion of multi-stage smart contract protocols.

The goal here is to provide developers with an easy experience to go from [Ergo dApp informal specifications](https://github.com/ergoplatform/eips/blob/master/eip-0006.md) to off-chain code with greater assurance and a straightforward path of implementation.


## Current Workflow
1. For each stage create an empty struct with the name of your Stage.
```rust
pub struct LiveEpoch {}
```