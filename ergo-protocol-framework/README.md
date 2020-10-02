# Ergo Protocol Framework

A framework which attempts to provide a guided approach to developing the off-chain portion of multi-stage smart contract protocols.

The goal here is to provide developers with an easy experience to go from [Ergo dApp informal specifications](https://github.com/ergoplatform/eips/blob/master/eip-0006.md) to off-chain code with greater assurance and a straightforward path of implementation.



## Current Workflow
1. For each stage create an empty struct with the name of your Stage.
```rust
pub struct LiveEpoch {}
```

2. Implement `StageType` trait on your struct.
```rust
impl StageType for LiveEpoch {
    fn new() -> LiveEpoch {
        LiveEpoch {}
    }
}

3. Create a struct for your smart contract protocol which holds all of your stages as fields:
```rust
pub struct OraclePoolProtocol {
    pub live_epoch_stage: Stage<LiveEpoch>,
}
```
4. Create a `Stage` struct for each of your stages using `Stage::new()`.

5. Implement a `new` method on your Protocol:
```rust
impl OraclePoolProtocol {
    /// Create a new StableCoinProtocol
    pub fn new(live_epoch_stage: Stage<LiveEpoch>) -> OraclePoolProtocol {
        OraclePoolProtocol {
            live_epoch_stage: live_epoch_stage
        }
    }
}
```
6. Use `verify_box()` method on your `Stage<T>` to create verified `StageBox<T:StageType>`s. These represent boxes that are guaranteed to valid boxes at a given stage, and thus can be used for performing Actions without any further checks.
7. Write methods for your protocol that represent Actions in your protocol using `StageBox<t>`s for the inputs to guarantee that your Action(state transition) logic is valid.