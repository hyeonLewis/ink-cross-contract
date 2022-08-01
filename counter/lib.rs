#![cfg_attr(not(feature = "std"), no_std)]

pub use self::counter::{Counter, CounterRef};

use ink_lang as ink;

#[ink::contract]
mod counter {

    use ink_prelude::vec::Vec;

    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Counter {
        count: u64,
        auth: Vec<AccountId>, //We could use Mapping<,> of course.
        init: bool,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        AlreadyInitialized,
        NotInitialized,
        AlreadyRegistered,
        AlreadyRemoved,
        BecomeZeroAuth,
        ValueIsOver10,
        CallerNotAuth,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    #[ink(event)] //Event emitted when transaction occurs
    pub struct Transaction {
        value: u64,
    }

    impl Counter {
        #[ink(constructor)]
        pub fn new(init_count: u64) -> Self {
            Self {
                count: init_count,
                auth: Vec::new(),
                init: false,
            }
        }

        #[ink(message)]
        pub fn init(&mut self, init_count: u64, _auth: AccountId) -> Result<()> {
            if self.init {
                return Err(Error::AlreadyInitialized);
            }

            self.count = init_count;
            self.auth.push(_auth);
            self.init = true;
            Ok(())
        }

        #[ink(message)] //Check the caller is in auth
        pub fn only_auth(&self) {
            let from = Self::env().caller();

            if !self.init {
                panic!("Not initialized");
            }

            if !self.auth.contains(&from) {
                panic!("Not authorized");
            }
        }

        #[ink(message)] //Execute our transaction
        pub fn execute(&mut self, input: u64) -> Result<()> {
            self.only_auth();

            if input > 10 {
                return Err(Error::ValueIsOver10);
            }

            self.count += input;

            Self::env().emit_event(Transaction { value: input });

            Ok(())
        }

        #[ink(message)] //Add auth
        pub fn add_auth(&mut self, new_auth: AccountId) -> Result<()> {
            self.only_auth();

            if self.auth.contains(&new_auth) {
                return Err(Error::AlreadyRegistered);
            }

            self.auth.push(new_auth);

            Ok(())
        }

        #[ink(message)] //Remove auth from Vec<AccountId>
        pub fn remove_auth(&mut self, _auth: AccountId) -> Result<()> {
            self.only_auth();

            if self.auth.len() == 1 {
                return Err(Error::BecomeZeroAuth);
            }

            if !self.auth.contains(&_auth) {
                return Err(Error::AlreadyRemoved);
            }

            self.auth.retain(|&x| x != _auth);

            Ok(())
        }

        #[ink(message)]
        pub fn increment(&mut self) {
            self.only_auth();
            self.count += 1;
        }

        #[ink(message)]
        pub fn decrement(&mut self) {
            self.only_auth();
            self.count -= 1;
        }

        #[ink(message)]
        pub fn reset(&mut self) {
            self.only_auth();
            self.count = 0;
        }

        #[ink(message)] //Since below 2 functions are "view" function, we don't need to check whether caller is in auth or not.
        pub fn get_count(&self) -> u64 {
            self.count
        }

        #[ink(message)]
        pub fn get_auth(&self) -> Vec<AccountId> {
            self.auth.clone()
        }
    }
}
