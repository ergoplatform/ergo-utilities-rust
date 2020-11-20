# 2. Math Bounty dApp - Finishing The dApp Off-Chain Library

In the first tutorial we went from 0 to having a fully functioning dApp off-chain library. Granted, this library only implemented half of our dApp (creating a Math Bounty box), and as such today we are going to fix this glaring hole by implementing the second half.

## Recap

As you may recall from the previous tutorial, we created a `MathBountyBox` which implemented the `SpecifiedBox` trait. This means that our `MathBountyBox` has a `BoxSpec` attached to it as a method which defines exactly what kind of `ErgoBox` is a valid `MathBountyBox`. (This `BoxSpec` also automatically provides an extremely useful interface for front-end developers to find the required UTXOs for Actions on the Blockchain with next to 0 work on their end.)

Furthermore, thanks to the derive procedural macros `#[derive(WrapBox, SpecBox)]`, our `MathBountyBox` has several helper methods to make reading data from the box easier, as well as an auto-generated `new` method which automatically verifies that an `ErgoBox` matches our `BoxSpec` while creating a new `MathBountyBox`.

Thus we have an interface for both creating and using our `MathBountyBox`. This is the overarching design pattern which you will use for implementing both stages as well as more generic input boxes to Actions in your protocol.


## Using Your First Specified Box In An Action































This is the final code from everything we've accomplished in this tutorial:

```rust
```


## Conclusion
