# 3. Math Bounty dApp - Writing A CLI Frontend

In the last two tutorials we created the Math Bounty dApp off-chain library which provided us with a pure interface for interacting with our dApp. In this tutorial we are going to use this off-chain library and create a textual front-end for it as a CLI app.

All of the design patterns and code we write will be equally as applicable to GUI-based front-ends as well, however to keep this tutorial concise we are going to be focused on creating a CLI interface instead.


## Creating The Project

We will create a new rust project (best to keep it in the same folder as your library) for our Math Bounty CLI app:

```
cargo new math-bounty-cli
```

In your new project folder edit the `Cargo.toml` and add your `math-bounty-lib` as a dependency, as well as the `ergo-node-interface`.

```rust
[dependencies]
math-bounty-lib     = {path = "../math-bounty-lib"}
ergo-node-interface = "0.2.0"
```

The `ergo-node-interface` is a Rust crate(library) which provides with all the functions you will need to interface with an Ergo node wallet so that you can do things such as acquiring the user's address, or asking the node wallet to sign and submit a generated `UnsignedTransaction`.

Continuing to your `main.rs` we will start coding by importing everything from your `math-bounty-lib`.

```rust
use math_bounty_lib::*;
use ergo_node_interface::*;
```


