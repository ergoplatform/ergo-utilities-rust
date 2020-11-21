# 3. Math Bounty dApp - Writing A CLI Frontend

In the last two tutorials we created the Math Bounty headless dApp which provides us with a pure interface for interacting with our smart contract protocol. In this tutorial we are going to use our headless dApp to create a textual front-end for it as a CLI app.

The vast majority of the design patterns and code we write will be equally as applicable to GUI-based front-ends as well, however to keep this tutorial concise we are going to be focused on creating a CLI interface instead.


## Creating The Project

We will create a new rust project (best to keep it in the same outer folder as your headless dApp) for our Math Bounty CLI app:

```
cargo new math-bounty-cli
```

In your new project folder edit the `Cargo.toml` and add your `math-bounty-headless` as a dependency, as well as the `ergo-node-interface` lib and `nano-get`.

```rust
[dependencies]
math-bounty-headless     = {path = "../math-bounty-headless"}
ergo-node-interface      = "0.2.2"
nano-get                 = { version = "0.2.4", features = ["https"] }
```

The `ergo-node-interface` is a Rust crate(library) which provides with all the functions you will need to interface with an Ergo node wallet so that you can do things such as acquiring the user's address, or asking the node wallet to sign and submit a generated `UnsignedTransaction`.

The `nano-get` library is just a lightweight library for issuing GET requests. Feel free to use any other Rust library that fulfills this task.


## Setting Up And Using The `NodeInterface`

Continuing to your `main.rs` we will start by importing everything from your `math-bounty-headless` and the `ergo-node-interface` lib (plus the `get` function from `nano_get`).

```rust
use math_bounty_headless::*;
use ergo_node_interface::*;
use nano_get::get;
```

Next we will create a new Ergo `NodeInterface` instance. This will allow us to interact with an Ergo Node via Rust. Do note, a user of the CLI app will need to have an unlocked Ergo Node wallet available in order for the CLI dApp to function.

We will be using `acquire_node_interface_from_local_config` from the Ergo Node Interface library to simplify the process of creating an `NodeInterface`. In short from the documentation:
```rust
/// A ease-of-use function which attempts to acquire a `NodeInterface`
/// from a local file. If the file does not exist, it generates a new
/// config file, tells the user to edit the config file, and then closes
/// the running application
/// This is useful for CLI applications, however should not be used by
/// GUI-based applications.
pub fn acquire_node_interface_from_local_config() -> NodeInterface;
```

As such on first run of our CLI application, the user will have a config file automatically generated for them, and be prompted to edit it with information about how to connect to their Ergo Node (ip/port/api_key). After that initial setup, the function will automatically generate a `NodeInterface` without any prompts, allowing the application to function normally.

```rust
fn main() {
    // Get a `NodeInterface`
    let node = acquire_node_interface_from_local_config();
}
```

Now that we have a `NodeInterface` which will query an Ergo Node wallet for us, we can use a couple methods to easily acquire the user's first address in their wallet, as well the current block height.

```rust
{
    // Get a `NodeInterface`
    let node = acquire_node_interface_from_local_config();
    // Get the current Ergo Blockchain block height
    let block_height = node.current_block_height().unwrap();
    // Get the first address in the user's wallet
    let user_address = node.wallet_addresses().unwrap()[0].clone();
}
```

And just like that we have all the information we need from the user's node wallet in 3 lines of code.


### Implement Argument Checking

Next we are going to implement argument checking for our CLI application. In our `main` function we will simply add this line to acquire the arguments that were submit by the user who ran our application.

```rust
    // Acquire CLI arguments
    let args: Vec<String> = std::env::args().collect();
```

Next we will do some basic checks to ensure that the user is either trying submit a bounty (create/bootstrap a new `MathBountyBox`), or is trying to solve the math problem and be awarded the bounty held in an existing `MathBountyBox`.

```rust
if args.len() == 2 {
    // User wishes to submit nanoErgs to create a new `MathBountyBox`
    if args[1] == "bounty" {
        let bounty_amount_in_nano_ergs = args[2].parse::<u64>().unwrap();
        todo!();
    }
    // User wishes to solve the math problem to be rewarded with the
    // bounty.
    if args[1] == "solve" {
        let math_problem_answer = args[2].parse::<u64>().unwrap();
        todo!();
    }
}
```

### Implementing The Submit Bounty CLI Logic

The CLI should allow a user to use the `bounty` command and provide an integer in order to build the "Bootstrap Math Bounty Box" Action using our headless dApp.

From the argument checking code block above, we will be filling out the logic for the `bounty` command in this section.

If you can recall, these are the inputs that are required for the "Bootstrap Math Bounty Box" action:

```rust
bounty_amount_in_nano_ergs: u64,
ergs_box_for_bounty: ErgsBox,
current_height: u64,
transaction_fee: u64,
ergs_box_for_fee: ErgsBox,
user_address: String,
```

Currently we are missing the following inputs:
1. `transaction_fee`
2. `ergs_box_for_bounty`
3. `ergs_box_for_fee`

We can get the transaction fee out of the way as it is quite simple:

```rust
let tx_fee = 1000000;
```

Now we can begin working on the two `ErgsBox`es.