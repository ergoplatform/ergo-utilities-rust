# Ergo Protocol Framework

A Rust framework that provides a streamlined experience in developing dApp off-chain code for single and multi-stage smart contract protocols. The EPF is completely pure, thus providing developers with the very first portable UTXO-based dApp development framework on any blockchain.

## Project Goals
1. Enable developers to write their off-chain logic once using the EPF, and have it be completely portable for any target platform (desktop, web, mobile).
2. Provide developers with an easy experience to go from [Ergo dApp Specifications](https://github.com/ergoplatform/eips/blob/master/eip-0006.md) to off-chain code with greater assurance and a straightforward path of implementation.
3. Separating the dApp off-chain logic from any front-end logic, thereby allowing reusability no matter what application or front-end is attempting to implement support for your dApp.
4. Providing easy-to-use methods for front-end implementors to easily access current state of the dApp protocol without having to understand how any of it works.
5. Abstracting the concept of defining and acquiring input UTXOs for your protocol by using a novel design pattern to specify `BoxSpec`s for the required input UTXOs.
6. Enabling scripts, (arbitrage) bots, and other software to be trivially built on top multiple dApps built using the EPF, thus offering a standardized interface and a level of composability.


## Understanding The Ergo Protocol Framework

Before you get started using the EPF, there are a number of terms and concepts that are important to understand. The overarching design of the EPF is based off of [EIP-6: Ergo Smart Contract Protocol Specification Format](https://github.com/ergoplatform/eips/blob/master/eip-0006.md).

What this means is that at the highest level, your dApp is defined as a [smart contract protocol](https://github.com/ergoplatform/eips/blob/master/eip-0006.md#smart-contract-protocol). If your dApp only has a single [stage](https://github.com/ergoplatform/eips/blob/master/eip-0006.md#stage), then it is defined as a "single-stage smart contract protocol". If your dApp has multiple stages, then it is a "multi-stage smart contract protocol. The EPF supports building both single and multi-stage protocol dApps.

Each stage can be considered a state in the protocol where a UTXO with Ergs, tokens, and data (within registers) is at in a given point in time. There may be a single box(UTXO) which moves from one stage to the next throughout the entire protocol, multiple boxes which go through all of the stages in parallel, or a variety of boxes asynchronously moving through certain sub-sets of stages.

No matter the specific design/complexity of your given smart contract protocol, each of these stages require "Actions". Actions are the state transitions (transaction logic) which allow:
1. Ergs/tokens/data to enter the protocol (aka. a bootstrap action)
2. Ergs/tokens/data to go from one stage in the protocol to another stage (or exiting the protocol).
3. Ergs/tokens/data to leave the protocol.

Each of these actions is made up of two key parts in the context of off-chain code:
1. Acquiring inputs (UTXOs/user input/external data from the outside world)
2. Creating output UTXOs with the result of the state transition

As such your dApp may either be a single or multi-stage smart contract protocol. Each stage in your dApp's protocol may have one or more actions. These actions are then defined by you the developer via specifying the required inputs required for a given action, and encoding the required state transition logic in order to create output UTXOs which are embedded within a newly created `UnsignedTx`.

The EPF provides you with the required tools to specify each of these building blocks in order to build your dApp from the ground-up. In the sections below, we will go through further details about how the EPF is built, and how you yourself can get started using it today.


## Modules Of The Ergo Protocol Framework

### Box Spec
This module exposes the `BoxSpec` struct, which allows you to create a specification of a UTXO. This is used for defining the boxes which are required for the actions of your protocol.

```rust
/// A specification which specifies parameters of an `ErgoBox`.
/// This spec is used as a "source of truth" to both verify and find
/// `ErgoBox`es which match the spec. This is often used for defining
/// Stages in multi-stage smart contract protocols, but can also be used
/// to define input boxes for Actions.
/// All fields are wrapped in `Option`s to allow ignoring specifying
/// the field.
#[wasm_bindgen]
#[derive(Clone)]
pub struct BoxSpec {
    /// The address of the box
    address: Option<ErgoAddressString>,
    /// The allowed range of nanoErgs
    value_range: Option<Range<NanoErg>>,
    /// A sorted list of `Constant`s which define registers
    /// of an `ErgoBox`.
    /// First element is treated as R4, second as R5, and so on.
    registers: Vec<Option<Constant>>,
    /// A sorted list of `TokenSpec`s which define tokens
    /// of an `ErgoBox`.
    tokens: Vec<Option<TokenSpec>>,
    /// An optional predicate which allows for defining custom
    /// specification logic which gets processed when verifying
    /// the box.
    predicate: Option<fn(&ErgoBox) -> bool>,
}
```

Once you've constructed a `BoxSpec`, you have a number of essential methods that simplify the experience of writing off-chain code for dApps.

For example, `verify_box` allows you to test whether an `ErgoBox` you provide as input matches the specification you created with your `BoxSpec`.

```rust
pub fn verify_box(&self, ergo_box: &ErgoBox) -> Result<()> {
```

### Box Traits
This module exposes two traits:
1. `WrappedBox`
2. `SpecifiedBox`

All `SpecifiedBox`es are also `WrappedBox`es. In your off-chain code you will be defining all of your inputs UTXOs to actions as structs that implement both `WrappedBox` and `SpecifiedBox`.

`WrappedBox`es provide a simplified interface for interacting with `ErgoBox`es. `SpecifiedBox`es on the other hand specify that a given `WrappedBox` also implements a `BoxSpec` via the `box_spec()` method.


### Specified Boxes
This module exposes generic "Specified Box" structs that implement `SpecifiedBox` and `WrappedBox` traits, and can be used as inputs for Actions in your off-chain protocol code.


### Output Builders
This module exposes structs which provide you with a basic interface
for creating common output UTXOs within your Actions. These are often
used for creating outputs that hold a user's change or pay a tx fee.

Example Output Builders:
1. ChangeBox
2. TokensChangeBox
3. TxFeeBox


### Tx Creation
This module exposes a few basic functions for making your life easier when building `UnsignedTransaction`s inside of your Actions.


## Getting Started

To learn how to use the EPF a tutorial series has been created which takes you step-by-step from writing the portable off-chain library to implement a basic CLI frontend.

Currently available parts:
1. [Math Bounty dApp - Getting Started Writing Your First Action](tutorials/Math_Bounty/1-math-bounty-dApp-getting-started.md)



# Documentation

To read the documentation for the EPF run the command below:

```
cargo doc --open
```

