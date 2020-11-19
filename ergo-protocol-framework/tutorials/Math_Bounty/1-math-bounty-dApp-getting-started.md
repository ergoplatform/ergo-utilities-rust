# 1. Math Bounty dApp - Getting Started Writing Your First Action

In this tutorial series we will be building a simple "Math Bounty" dApp using the Ergo Protocol Framework. In short, this dApp allows individuals to lock Ergs up under a contract which requires a person to solve the math problem encoded in the contract in order to withdraw the funds inside. The idea for this dApp originally came from [this Ergo Forum Thread](https://www.ergoforum.org/t/mathematical-fun-with-ergoscript/76).

In our case we'll be using a simpler problem/contract to make it easy to follow along. Do note that this dApp isn't 100% secure because either because bad actors/bots can front-run your answer submission by watching the mempool. Nonetheless, this is an instructive example that you will be able to run live on testnet/mainnet for educational purposes. (Refer to the above linked thread for more details about how to make a more complicated, but secure Math Bounty smart contract)

In this first tutorial of the series, we will be covering the basics of how to get started in creating your dApp's off-chain library all the way to writing your first protocol action.

## The Smart Contract

Before we dive into building the off-chain portion of our dApp, let's take a look at the contract we'll be using.

```scala
{
 OUTPUTS(0).R4[Long].get * 2 == 4
}
```
As can be seen, the contract is extremely simple.

In short, a user can withdraw funds if they can figure out what number multiplied by `2` is equal to `4`. Specifically, funds can be withdrawn if the output UTXO with the index 0 has a register R4, with a Long Integer in the register, and said register is equal to the number `2` (in order for the equation to be true).

Compiling this contract into a P2S address results in the address: `94hWSMqgxHtRNEWoKrJFGVNQEYX34zfX68FNxWr`. ([Click here to try compiling it for yourself on the PlutoMonkey Playground](https://wallet.plutomonkey.com/p2s/?source=ewogSU5QVVRTKDApLlI0W0xvbmddLmdldCAqIDIgPT0gNAp9))


In the rest of this tutorial we will be writing the off-chain library for our dApp which performs all of the transaction creation logic for someone to perform the "Solve Problem" action, and thus withdraw the funds locked.


## 




First we're going to import all of the Ergo Protocol Framework structs/functions/macros to keep things simple:

```rust
use ergo_protocol_framework::*;
use ergo_lib::chain::ergo_box::ErgoBox;
```
You may have noticed that we also imported `ErgoBox` from the `ergo_lib` library. This is the Rust struct representation of an Ergo box(UTXO) which we will use shortly.


------



The next thing we are going to do is define the stages of our protocol. In our case we have a simple single-stage smart contract protocol in our dApp. This means we need to only create a single Rust stage-representing struct for our dApp.

This stage in our dApp will be called the `Math Problem` stage. As such, we will name the struct which will wrap an `ErgoBox` at this stage the `MathProblemBox`.

```rust
pub struct MathProblemBox {
    ergo_box: ErgoBox,
}
```

Now that we've defined the `MathProblemBox`, we can also derive a few traits automatically to make our lives easier:

```rust
#[derive(Debug, Clone, WrapBox)]
pub struct MathProblemBox {
    ergo_box: ErgoBox,
}
```

To the Rust-initiated, `Debug` and `Clone` are typical, but `WrapBox` is new. This is a procedural macro which automatically implements the `WrappedBox` trait for our `MathProblemBox`. In other words, we have access to new helper methods without writing any extra code ourselves.


-----

Next we are going to implement the `SpecifiedBox` trait on our `MathProblemBox`.

```rust
impl SpecifiedBox for MathProblemBox {
    fn box_spec() -> BoxSpec {
        todo!();
    }
}
```

Now this is where things get interesting. This trait requires us to implement a method which returns a `BoxSpec`.

A `BoxSpec` is a specification in the form of a Rust struct which specifies parameters of an `ErgoBox`. This spec struct is used as a "source of truth" to both verify and find `ErgoBox`es which match the spec.

As such, we are going to create a `BoxSpec` for the Math Problem stage, which will be used by our `MathProblemBox` struct. We will be doing this using the `BoxSpec::new()` method which allows us to specify the address, value range, registers, and tokens for our specification. In our case we will only be using the address due to the simplicity of our dApp.


```rust
impl SpecifiedBox for MathProblemBox {
    fn box_spec() -> BoxSpec {
        let address = Some("94hWSMqgxHtRNEWoKrJFGVNQEYX34zfX68FNxWr".to_string());
        BoxSpec::new(address, None, vec![], vec![])
    }
}
```

Our Rust-based spec of the Math Problem stage states that an `ErgoBox` which has an address of `94hWSMqgxHtRNEWoKrJFGVNQEYX34zfX68FNxWr` is a valid `MathProblemBox`. Furthermore this means that no matter what Ergs/registers/tokens the `ErgoBox` has inside, it is still considered a valid `MathProblemBox`. For our use case, this is a valid spec for what we were going for.


---

Next up, we are going to implement the `new` method so that a `MathProblemBox` can be created.

```rust
impl MathProblemBox {
    pub fn new(ergo_box: &ErgoBox) -> Option<MathProblemBox> {
        // Using the automatically implemented `verify_box` method
        // from the `BoxSpec` to verify the `ErgoBox` is a valid
        // `MathProblemBox`.
        Self::box_spec().verify_box(ergo_box).ok()?;

        // Creating the `MathProblemBox`
        let math_problem_box = MathProblemBox {
            ergo_box: ergo_box.clone(),
        };

        // Returning the `MathProblemBox`
        Some(math_problem_box)
    }
}
```

As can be seen above, we use the `verify_box` method to ensure that the `ErgoBox` is indeed a valid `MathProblemBox` according to the spec we defined. This `verify_box` method is automatically available for use once the `SpecifiedBox` trait has been implemented.


---

Going forward, we are going to begin defining the actions of our protocol. Before we get there, first we must create an empty struct which represents our dApp protocol. In our case, we are just going to call it `MathProblemProtocol`.


```rust
pub struct MathProblemProtocol {}
```

With a struct that represents our smart contract protocol we can implement protocol actions as methods on said struct. Thus when we expose this `MathProblemProtocol` publicly, it will be easy in the future to implement front-ends for our dApp.

We will now begin to write our first action, `Bootstrap Math Problem Box`, by making it a method.

```rust
use ergo_lib::chain::transaction::unsigned::UnsignedTransaction;


impl MathProblemProtocol {
    /// A bootstrap action which allows a user to create a `MathProblemBox`
    /// with funds locked inside as a bounty for solving the math problem.
    pub action_bootstrap_math_problem_box() -> UnsignedTransaction (
        todo!()
    }
}
```

When writing actions with the Ergo Protocol Framework, we must keep in mind that we are building pure, portable, and reusable code.

What this means is that all of our transaction creation logic within our actions must be self-contained. This is why we are creating/returning an `UnsignedTransaction` from our action. Furthermore this means that any external data (from the blockchain, or user input) must be provided to the action method via arguments. Thus for our `Bootstrap Math Problem Box` action, we will need the following inputs:

```rust
    pub fn action_bootstrap_math_problem_box(
        bounty_amount_in_nano_ergs: u64,
        ergs_box_for_bounty: ErgsBox,
        current_height: u64,
        transaction_fee: u64,
        ergs_box_for_fee: ErgsBox,
        user_address: String,
    ) -> UnsignedTransaction
```

The current height is required for tx building, the transaction fee is to be decided by the front-end implementor when the action method is used, the `ergs_box_for_fee` is a wrapped `ErgoBox` which is used to pay for the fee for the transaction, and the user's address is required to send change back to the user. These are the minimum arguments required for any action you will ever write.

Furthermore, in our current scenario, we also have the `ergs_box_for_bounty` input argument and `bounty_amount_in_nano_ergs`. In the front-end the user will provide the amount of nanoErgs they want to submit as a bounty to the dApp, and the front-end implementation must find an input `ErgsBox` with sufficient nanoErgs to cover the bounty amount which is owned by the user.

This is actually a lot simpler than it all may sound thanks to the EPF implementing a number of key helper methods on top of `SpecifiedBox`s (an `ErgsBox` being one of the default provided `SpecifiedBox`es by the EPF) for acquiring UTXOs easily. This will all be tackled in a future tutorial once we are working on building a front-end for our dApp.

Next let's write the basic scaffolding for creating our `UnsignedTransaction` that we are returning in our method:

```rust
{
    let tx_inputs = vec![];
    let data_inputs = vec![];
    let output_candidates = vec![];

    UnsignedTransaction::new(tx_inputs, data_inputs, output_candidates)
}
```

As can be seen, to create an unsigned transaction we simply need three things:
1. An ordered list of input boxes
2. An ordered list of data-inputs
3. An ordered list of output box candidates

In our case we will not be using any data-inputs (aka. read-only inputs) because our protocol is simple and does not rely on any other UTXOs on the blockchain. Thus we will move forward by defining our `tx_inputs` and our `output_candidates` in such a way that the resulting `UnsignedTransaction` is a valid implementation of our `Bootstrap Math Problem Box` action.

Let's implement the inputs first as these are very simple.

```rust
let tx_inputs = vec![
    ergs_box_for_bounty.as_unsigned_input(),
    ergs_box_for_fee.as_unsigned_input(),
];
```

All we are doing here is making the bounty `ErgsBox` the first input (index 0) and the fee `ErgsBox` the second input (index 1). To convert from a `SpecifiedBox`(or `WrappedBox`, which `ErgsBox`es are both) into an input we can feed to `UnsignedTransaction::new`, then we simply call the `.as_unsigned_input()` method.

We've already completed 2/3 of the requirements for creating an `UnsignedTransaction`, but now we get to the more interesting part where we encode the logic of our action.


### Implementing Action Logic

First of all we need to figure out how much extra change is held within the `ErgsBox`es that were provided as inputs. This is very simple math:

```rust
let total_nano_ergs = ergs_box_for_bounty.nano_ergs() + ergs_box_for_fee.nano_ergs();
let total_change = total_nano_ergs - bounty_amount_in_nano_ergs - transaction_fee;
```

In short, whatever we don't use for the bounty or the tx fee has to go back to the user as change.

Now with that out of the way we can begin creating our outputs. First we are going to create our Math Problem Box output candidate. We are going to use the `create_candidate` function provided by the EPF to do this.

```rust
// Creating our Math Problem Box output candidate
let math_problem_candidate = create_candidate(
    bounty_amount_in_nano_ergs,
    &"94hWSMqgxHtRNEWoKrJFGVNQEYX34zfX68FNxWr".to_string(),
    &vec![],
    &vec![],
    current_height,
)
.unwrap();
```

The `create_candidate` function takes the following inputs:
1. nanoErgs to be held in the resulting output box.
2. The address which the output box will be at.
3. Tokens that the output box will hold.
4. Register values that the box will hold.
5. Current block height

In our case we simply want to create an output at the P2S smart contract address we compiled at the start of this tutorial, `94hWSMqgxHtRNEWoKrJFGVNQEYX34zfX68FNxWr`, and for it to hold an amount of nanoErgs equal to `bounty_amount_in_nano_ergs`. We do not require any tokens to be held in the output, nor any data in the registers, and as such we leave these empty.

As a result, we have created our Math Problem Box output candidate which will lock the bounty Ergs under the smart contract. Now we can finish off building the two final candidates for our action.

### Creating The Tx Fee And Change Boxes

Rather than manually using the `create_candidate` function for every single output candidate we are building, which can get tedious, the EPF provides us with some default "output builders". These are structs that off associated functions which build output candidates for easily, thereby making our lives easier.

Thus to create a tx fee box output candidate, all we have to do is:
```rust
let transaction_fee_candidate =
    TxFeeBox::output_candidate(transaction_fee, current_height).unwrap();
```

Similarly the EPF also offers an output builder for a change box which we will use as well:

```rust
let change_box_candidate = ChangeBox::output_candidate(
    &vec![],
    total_change,
    &user_address,
    current_height,
)
```

And just like that we've finished creating all three required output candidates for our action. To reiterate, the three output box candidates we created were:
1. The Math Problem Box
2. Transaction Fee Box
3. The Change Box

And now to finish implementing our action at last, all we have to do is add the candidates we created into our list of `output_candidates` in the correct order. (In our case, our smart contract only specifies that the Math Problem Box must be the first output)


```rust
let output_candidates = vec![
    math_problem_candidate,
    transaction_fee_candidate,
    change_box_candidate,
];

```




There we have it...



Final code for this tutorial:


```rust
use ergo_lib::chain::ergo_box::ErgoBox;
use ergo_lib::chain::transaction::unsigned::UnsignedTransaction;
use ergo_protocol_framework::*;

#[derive(Debug, Clone, WrapBox)]
pub struct MathProblemBox {
    ergo_box: ErgoBox,
}

impl SpecifiedBox for MathProblemBox {
    fn box_spec() -> BoxSpec {
        let address = Some("94hWSMqgxHtRNEWoKrJFGVNQEYX34zfX68FNxWr".to_string());
        BoxSpec::new(address, None, vec![], vec![])
    }
}

impl MathProblemBox {
    pub fn new(ergo_box: &ErgoBox) -> Option<MathProblemBox> {
        // Using the automatically implemented `verify_box` method
        // from the `BoxSpec` to verify the `ErgoBox` is a valid
        // `MathProblemBox`.
        Self::box_spec().verify_box(ergo_box).ok()?;

        // Creating the `MathProblemBox`
        let math_problem_box = MathProblemBox {
            ergo_box: ergo_box.clone(),
        };

        // Returning the `MathProblemBox`
        Some(math_problem_box)
    }
}

pub struct MathProblemProtocol {}

impl MathProblemProtocol {
    /// A bootstrap action which allows a user to create a `MathProblemBox`
    /// with funds locked inside as a bounty for solving the math problem.
    pub fn action_bootstrap_math_problem_box(
        bounty_amount_in_nano_ergs: u64,
        ergs_box_for_bounty: ErgsBox,
        current_height: u64,
        transaction_fee: u64,
        ergs_box_for_fee: ErgsBox,
        user_address: String,
    ) -> UnsignedTransaction {
        let tx_inputs = vec![
            ergs_box_for_bounty.as_unsigned_input(),
            ergs_box_for_fee.as_unsigned_input(),
        ];

        // Calculating left over change nanoErgs
        let total_nano_ergs = ergs_box_for_bounty.nano_ergs() + ergs_box_for_fee.nano_ergs();
        let total_change = total_nano_ergs - bounty_amount_in_nano_ergs - transaction_fee;

        // Creating our Math Problem Box output candidate
        let math_problem_candidate = create_candidate(
            bounty_amount_in_nano_ergs,
            &"94hWSMqgxHtRNEWoKrJFGVNQEYX34zfX68FNxWr".to_string(),
            &vec![],
            &vec![],
            current_height,
        )
        .unwrap();

        // Create the Transaction Fee box candidate
        let transaction_fee_candidate =
            TxFeeBox::output_candidate(transaction_fee, current_height).unwrap();

        // Create the Change box candidate
        let change_box_candidate =
            ChangeBox::output_candidate(&vec![], total_change, &user_address, current_height)
                .unwrap();

        // Our output candidates list, specifically with the Math Problem box
        // candidate being the first, meaning Output #0.
        let output_candidates = vec![
            math_problem_candidate,
            transaction_fee_candidate,
            change_box_candidate,
        ];

        UnsignedTransaction::new(tx_inputs, vec![], output_candidates)
    }
}
```
