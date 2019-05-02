#![cfg_attr(not(any(test, feature = "std")), no_std)]

use ink_lang::contract;
use ink_core::storage;
use ink_core::env::{self, AccountId};

use parity_codec::{
    Decode,
    Encode,
};

#[derive(Encode, Decode)]
enum Event {
    OwnershipTransferred {
        from: Option<AccountId>,
        to: Option<AccountId>,
    },
}

fn transer_ownership_event (event: Event) {
    env::deposit_raw_event(&event.encode()[..])
}
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
           transer_ownership_event(Event::OwnershipTransferred {
               from: Some(env::caller()),
               to: Some(new_owner)
           });
       }

    
    }
}

// comment out all test to build 
#[cfg(all(test, feature = "test-env"))]
mod tests {
    use super::*;
    use std::convert::TryFrom;

      
    #[test]

    fn is_owner() {
        let _bob = AccountId::try_from([0x1; 32]).unwrap();

        let mut contract = Ownable::deploy_mock();
        assert_eq!(contract.is_owner(), true);
        env::test::set_caller(_bob);
        assert_eq!(contract.is_owner(), false);

    }
    
    #[test]

    fn get_owner() {
        let _alice = AccountId::try_from([0x0; 32]).unwrap();
        let _bob = AccountId::try_from([0x1; 32]).unwrap();


        let mut contract = Ownable::deploy_mock();
        assert_eq!(contract.get_owner(), _alice);
        assert_ne!(contract.get_owner(), _bob);


    }

    #[test]

    fn transfer_ownership() {
        let _alice = AccountId::try_from([0x0; 32]).unwrap();
        let _bob = AccountId::try_from([0x1; 32]).unwrap();

        let mut contract = Ownable::deploy_mock();
        contract.transfer_ownership(_bob);
        // will fail
        // contract.transfer_ownership(alice);        

    }


}
