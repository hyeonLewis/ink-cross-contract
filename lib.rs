//Some of codes are from Paritytech/ink/examples/delegator
//You can find an example for delegate calls from here: https://github.com/hyeonLewis/ink/tree/master/examples/delegator
//Below code is for cross-contract interaction between accumulator, adder, subber and counter.
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod forward {
    use accumulator::AccumulatorRef;
    use adder::AdderRef;
    use counter::CounterRef;
    use ink_env::call::FromAccountId;
    use ink_prelude::vec::Vec;
    use ink_storage::traits::{PackedLayout, SpreadLayout};
    use subber::SubberRef;

    /// Specifies the state of the `forward` contract.
    ///
    /// In `Adder` state the `forward` contract will call to the `Adder` contract
    /// and in `Subber` state will call to the `Subber` contract.
    ///
    /// The initial state is `Adder`.
    #[derive(
        Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout,
    )]
    #[cfg_attr(
        feature = "std",
        derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub enum Which {
        Adder,
        Subber,
    }


    /// In this example, I use error for counter contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        TransactionFailed,
        AddAuthFailed,
        RemoveAuthFailed,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    /// Forward calls to an `adder` or `subber` contract to mutate
    /// a value in an `accumulator` contract.
    ///
    /// # Note
    ///
    /// In order to instantiate the `forward` smart contract we first
    /// have to manually put the code of the `accumulator`, `adder`
    /// , `subber` and "counter" smart contracts, receive their account id.
    ///
    /// The `AccumulatorRef`, `AdderRef`, `SubberRef` and "CounterRef" are smart contract
    /// reference types that have been automatically generated by ink!.
    #[ink(storage)]
    pub struct Forward {
        /// Says which of `adder` or `subber` is currently in use.
        which: Which,
        /// The `accumulator` smart contract.
        accumulator: AccumulatorRef,
        /// The `adder` smart contract.
        adder: AdderRef,
        /// The `subber` smart contract.
        subber: SubberRef,
        /// The 'counter' smart contract,
        counter: CounterRef,
    }

    impl Forward {
        /// Since we just want to call other contracts in Forward, we need account id for each contracts.
        /// It means that below contracts should be uploaded before this.
        #[ink(constructor)]
        pub fn new(
            accumulator_code: AccountId,
            adder_code: AccountId,
            subber_code: AccountId,
            counter_code: AccountId,
        ) -> Self {
            let accumulator = AccumulatorRef::from_account_id(accumulator_code);
            let adder = AdderRef::from_account_id(adder_code);
            let subber = SubberRef::from_account_id(subber_code);
            let counter = CounterRef::from_account_id(counter_code);
            Self {
                which: Which::Adder,
                accumulator,
                adder,
                subber,
                counter,
            }
        }

        /// Returns the `accumulator` value.
        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.accumulator.get()
        }

        /// Delegates the call to either `Adder` or `Subber`.
        #[ink(message)]
        pub fn change(&mut self, by: i32) {
            match self.which {
                Which::Adder => self.adder.inc(by),
                Which::Subber => self.subber.dec(by),
            }
        }

        /// Switches the contract.
        #[ink(message)]
        pub fn switch(&mut self) {
            match self.which {
                Which::Adder => {
                    self.which = Which::Subber;
                }
                Which::Subber => {
                    self.which = Which::Adder;
                }
            }
        }

        /// Returns the 'counter' value.
        #[ink(message)]
        pub fn get_count_value(&self) -> u64 {
            self.counter.get_count()
        }

        /// Returns the Vec<AccoundId> of 'counter'.
        #[ink(message)]
        pub fn get_auth_id(&self) -> Vec<AccountId> {
            self.counter.get_auth()
        }

        /// Execute transaction for adding value to count.
        #[ink(message)]
        pub fn execution(&mut self, value: u64) -> Result<()> {
            match self.counter.execute(value) {
                Ok(()) => Ok(()),
                Err(err) => Err(Error::TransactionFailed),
            }
        }

        /// Add auth.
        #[ink(message)]
        pub fn add_auth_tx(&mut self, auth: AccountId) -> Result<()> {
            match self.counter.add_auth(auth) {
                Ok(()) => Ok(()),
                Err(err) => Err(Error::AddAuthFailed),
            }
        }

        /// Remove auth.
        #[ink(message)]
        pub fn remove_auth_tx(&mut self, auth: AccountId) -> Result<()> {
            match self.counter.remove_auth(auth) {
                Ok(()) => Ok(()),
                Err(err) => Err(Error::RemoveAuthFailed),
            }
        }

        /// Add 1 to count.
        #[ink(message)]
        pub fn increment_tx(&mut self) {
            self.counter.increment()
        }

        /// Minus 1 to count.
        #[ink(message)]
        pub fn decrement_tx(&mut self) {
            self.counter.decrement()
        }
    }
}
