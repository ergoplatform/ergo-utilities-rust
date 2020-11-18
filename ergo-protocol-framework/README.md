# Ergo Protocol Framework

A Rust framework that provides a streamlined experience in developing dApp off-chain code for single and multi-stage smart contract protocols. The EPF is completely pure, thus providing developers with the very first portable UTXO-based dApp development framework on any blockchain.

## Project Goals
1. Enable developers to write their off-chain logic once using the EPF, and have it be completely portable for any target platform (desktop, web, mobile).
2. Provide developers with an easy experience to go from [Ergo dApp Specifications](https://github.com/ergoplatform/eips/blob/master/eip-0006.md) to off-chain code with greater assurance and a straightforward path of implementation.
3. Separating the dApp off-chain logic from any front-end logic, thereby allowing reusability no matter what application or front-end is attempting to implement support for your dApp.
4. Providing easy-to-use methods for front-end implementors to easily access current state of the dApp protocol without having to understand how any of it works.
5. Abstracting the concept of defining and acquiring input UTXOs for your protocol by using a novel design pattern to specify `BoxSpec`s for the required input UTXOs.
6. Enabling scripts, (arbitrage) bots, and other software to be trivially built on top multiple dApps built using the EPF, thus offering a standardized interface and a level of composability.


## Current Workflow
1. For each stage create an empty struct with the name of your Stage.
```rust
pub struct LiveEpoch {}
```