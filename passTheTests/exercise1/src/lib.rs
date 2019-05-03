#![cfg_attr(not(any(test, feature = "std")), no_std)]

use ink_lang::contract;
use ink_core::storage;

contract! {
    struct Incrementer {
    }

    impl Deploy for Incrementer {
        fn deploy(&mut self, init_value: u64) {
        }
    }

    impl Incrementer {

        pub (external) fn set(&mut self, amount: u64) {
        }
        pub(external) fn get(&self) -> u64 {
        }
    }
}

#[cfg(all(test, feature = "test-env"))]
mod tests {
    use super::*;


    #[test]
    fn set_works() {
        let mut contract = Incrementer::deploy_mock(3);
        assert_eq!(contract.get(), 3);
        contract.set(4);
        assert_eq!(contract.get(), 7);
        contract.set(0);
        assert_eq!(contract.get(), 7)
    }
}

