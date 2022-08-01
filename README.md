# Cross-contract Smart Contract [Modified from paritech/ink/examples/delegator]

The cross-contract interaction smart contract.

It consists in total of 5 different smart contract:

- Forward (root): Forward calls to other smart contract
- Adder: Increases a value in the Accumulator smart contract
- Subber: Decreases a value in the Accumulator smart contract
- Accumulator: Owns a simple `i32` value that can be incremented or decremented
- Counter: simple-counter example that we already implemented before

You can test cross-contract interaction between Adder, Subber, Accumulator and counter.

You can upload the contracts using local node.

1. Compile all contracts using the `./build.sh` script. (It will take 10 ~ 15 minutes)
   You will receive the respective `.contract` bundles for all the smart contracts in the `target/ink/` folder:
   * `target/ink/forward.contract`
   * `target/ink/adder/adder.contract`
   * `target/ink/subber/subber.contract`
   * `target/ink/accumulator/accumulator.contract`
   * `target/ink/counter/counter.contract`
1. Upload the `.contract` bundle of Accumulator, Adder, Subber and Counter to the chain.
1. Instantiate the Forward smart contract given all of the code hashes and a starting value.
   Make sure the endowment is big enough (if you're using our `substrate-contracts-node` it's `1000000`).
   The Forward smart contract will take over the work of instantiating the other smart contracts for you.
