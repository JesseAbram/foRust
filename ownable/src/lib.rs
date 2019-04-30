#![cfg_attr(not(any(test, feature = "std")), no_std)]

use ink_lang::contract;
use ink_core::storage;
use ink_core::memory::format;
use ink_core::env::{self, println, AccountId};



contract! {
    
    struct Ownable {
        owner: storage::Value<AccountId>,
    }

    impl Deploy for Ownable {
        
        fn deploy(&mut self) {
            self.owner.set(env.caller());
        }
    }

    impl Ownable {

    
        pub (external) fn is_owner(&mut self) -> bool {
            env::caller() == *self.owner 
         }

       pub (external) fn get_owner(&mut self) -> AccountId {
           *self.owner
       }

       fn only_owner(&mut self, caller: AccountId) -> bool {
           if caller == *self.owner {
               true
           } else {
               panic!("Must be called from owner account");
           }
       }

       pub (external) fn transfer_ownership(&mut self, new_owner: AccountId){
           self.only_owner(env::caller());
           self.owner.set(new_owner); 
       }
    
    }
}

#[cfg(all(test, feature = "test-env"))]
mod tests {
    use super::*;
    use std::convert::TryFrom;

      
    #[test]

    fn is_owner() {
        let bob = AccountId::try_from([0x1; 32]).unwrap();

        let mut contract = Ownable::deploy_mock();
        assert_eq!(contract.is_owner(), true);
        env::test::set_caller(bob);
        assert_eq!(contract.is_owner(), false);

    }
    
    #[test]

    fn get_owner() {
        let alice = AccountId::try_from([0x0; 32]).unwrap();
        let bob = AccountId::try_from([0x1; 32]).unwrap();


        let mut contract = Ownable::deploy_mock();
        assert_eq!(contract.get_owner(), alice);
        assert_ne!(contract.get_owner(), bob);


    }

    #[test]

    fn transfer_ownership() {
        let alice = AccountId::try_from([0x0; 32]).unwrap();
        let bob = AccountId::try_from([0x1; 32]).unwrap();

        let mut contract = Ownable::deploy_mock();
        contract.transfer_ownership(bob);
        // will fail
        // contract.transfer_ownership(alice);        
      

    }


}
