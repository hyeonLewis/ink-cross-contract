#![cfg_attr(not(feature = "std"), no_std)]

pub use self::subber::{Subber, SubberRef};

use ink_lang as ink;

#[ink::contract]
mod subber {
    use accumulator::AccumulatorRef;
    use ink_env::call::FromAccountId;

    /// Decreases the underlying `accumulator` value.
    #[ink(storage)]
    pub struct Subber {
        /// The `accumulator` to store the value.
        accumulator: AccumulatorRef,
    }

    impl Subber {
        /// Creates a new `subber` from the given `accumulator`.
        #[ink(constructor)]
        pub fn new(accumulator_code: AccountId) -> Self {
            let accumulator = AccumulatorRef::from_account_id(accumulator_code);
            Self { accumulator }
        }

        /// Decreases the `accumulator` value by some amount.
        #[ink(message)]
        pub fn dec(&mut self, by: i32) {
            self.accumulator.inc(-by)
        }
    }
}
