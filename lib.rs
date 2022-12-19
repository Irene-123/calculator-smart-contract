#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod calculator {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
  
    #[ink(storage)]
    pub struct Calculator {
        /// Stores a single `bool` value on the storage.
        a: u64,
        b: u64, 
        op: String 
    }

    impl Calculator {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(value1: u64, value2: u64, operation: String) -> Self {
            Self { a: value1, 
                   b: value2, 
                   op: operation
                }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
        Self::new(Default::default(), Default::default(), String::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        // #[ink(message)]
        // pub fn flip(&mut self) {
        //     self.value = !self.value;
        // }

        // /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self)-> u64{
            self.a
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let calculator = Calculator::default();
            assert_eq!(calculator.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut calculator = Calculator::new(5, 5, '+');
            // assert_eq!(calculator.get(), false);
            // calculator.flip();
            // assert_eq!(calculator.get(), true);
        }
    }
}
