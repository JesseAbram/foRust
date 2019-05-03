#![cfg_attr(not(any(test, feature = "std")), no_std)]

use ink_core::{
    storage,
    env::AccountId,
};
use ink_lang::contract;


contract! {
  
    struct Exercise2 {
    }

    impl Deploy for Exercise2 {
        fn deploy(&mut self) {

        }
    }

    impl Exercise2 {
        pub(external) fn set(&mut self, key: AccountId, value: u64) {

        }

        pub(external) fn get(&self, key: AccountId) -> u64 {
            
        }
    }
    impl Exercise2 {
        fn get_map(&self, of: &AccountId) -> u64 {
            
        }
    }

}

#[cfg(all(test, feature = "test-env"))]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn get_number() {
        let alice = AccountId::try_from([0x0; 32]).unwrap();
        let bob = AccountId::try_from([0x1; 32]).unwrap();

        let mut contract = Exercise2::deploy_mock();
        contract.set(alice, 3);
        assert_eq!(contract.get(alice), 3);
        assert_eq!(contract.get(bob), 0);

    }


}