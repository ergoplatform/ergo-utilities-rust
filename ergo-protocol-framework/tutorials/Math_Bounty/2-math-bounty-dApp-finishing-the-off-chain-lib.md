# 2. Math Bounty dApp - Finishing The Off-Chain Library

In the first tutorial we went from 0 to having a fully functioning dApp off-chain library. Granted, this library only implemented half of our dApp (creating a Math Bounty box), and as such today we are going to fix this glaring hole by implementing the second half.

## Recap

As you may recall from the previous tutorial, we created a `MathBountyBox` which implemented the `SpecifiedBox` trait. This means that our `MathBountyBox` has a `BoxSpec` attached to it as a method which defines exactly what kind of `ErgoBox` is a valid `MathBountyBox`. (This `BoxSpec` also automatically provides an extremely useful interface for front-end developers to find the required UTXOs for Actions on the Blockchain with next to 0 work on their end.)

Furthermore, thanks to the derive procedural macros `#[derive(WrapBox, SpecBox)]`, our `MathBountyBox` has several helper methods to make reading data from the box easier, as well as an auto-generated `new` method which automatically verifies that an `ErgoBox` matches our `BoxSpec` while creating a new `MathBountyBox`.

Thus we have an interface for both creating and using our `MathBountyBox`. This is the overarching design pattern which you will use for implementing both stages as well as more generic input boxes to Actions in your protocol.


## Using Your First Specified Box In An Action

As you may recalled, previously we used the `ErgsBox` struct for two of the inputs for our "BootStrap Math Bounty Box" Action. This `ErgsBox` struct is itself a `SpecifiedBox` which we got to take advantage of.

Today we are going to be writing our first Action using a `SpecifiedBox` that we created ourselves. That being our `MathBountyBox`.

The action which we will be implementing is the "Solve Math Problem" action. This action allows a user who knows the answer to the math problem we encoded inside of our smart contract to withdraw the bounty funds inside of a box at the Math Bounty stage (aka. a `MathBountyBox`).

As with the first action we wrote, we will be implementing this one on our `MathBountyProtocol` as a method:

```rust
impl MathBountyProtocol {
    pub fn action_solve_math_problem() -> UnsignedTransaction {
        todo!()
    }

    ...
}
```

This time the input arguments to our action method are going to be a bit different. We will need:
1. The answer to the math problem (which would be acquired from user-input in front-end)
2. The `MathBountyBox` which we will be spending in order to withdraw the bounty funds inside.
3. Current Block Height
4. Transaction Fee
5. ErgsBox For Fee
6. User Address


```rust
/// An action to solve the math problem inside of a `MathBountyBox`
/// and thus to withdraw the bounty nanoErgs inside as a reward.
pub fn action_solve_math_problem(
    math_problem_answer: u64,
    math_bounty_box: MathBountyBox,
    current_height: u64,
    transaction_fee: u64,
    ergs_box_for_fee: ErgsBox,
    user_address: String,
) -> UnsignedTransaction {
    todo!()
}
```

The majority of these arguments are the same, except this time around we are using the `math_problem_answer` in order to make the smart contract that the `math_bounty_box` is locked under evaluate to `true` and thus allowing us to spend it (withdraw the bounty funds).

To start off, let's fill out our method with the same boilerplate as from the first tutorial.

```rust
{
    let tx_inputs = vec![];
    let output_candidates = vec![];

    UnsignedTransaction::new(tx_inputs, vec![], output_candidates)
}
```

Just like in the previous action, we will not be using any data-inputs (the 2nd argument to `new`), and as such we can just fill that in as being empty from the start.

Next we will be filling in the `tx_inputs` vector. Because our smart contract did not have any checks on the inputs of the transaction, the order of our inputs makes no difference. Nonetheless, we will put the `math_bounty_box` first and the `ergs_box_for_fee` second, as this is usually the default that you will be using in more complex dApps in the future.

```rust
{

    let tx_inputs = vec![
        math_bounty_box.as_unsigned_input(),
        ergs_box_for_fee.as_unsigned_input(),
    ];
    let output_candidates = vec![];

    UnsignedTransaction::new(tx_inputs, vec![], output_candidates)
}
```































This is the final code from everything we've accomplished in this tutorial:

```rust
```


## Conclusion
