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

fn deposit_event (event: Event) {
    env::deposit_raw_event(&event.encode()[..])
}

contract! {
    struct Ownable {
        owner: storage::Value<Option<AccountId>>,
    }

    impl Deploy for Ownable {
        fn deploy(&mut self) {
            self.owner.set(Some(env.caller()));
            deposit_event(Event::OwnershipTransferred {
               from: None,
               to: Some(env::caller())
            });
        }
    }

    impl Ownable {
        pub (external) fn is_owner(&mut self) -> bool {
            if self.owner.is_some() {
                return env::caller() == self.owner.unwrap()
            }
            false
        }

        pub (external) fn get_owner(&mut self) -> Option<AccountId> {
            // Need to remove `--features generate-api-description` from build.sh
            // Known bug, will be fixed in the future
            *self.owner
        }

        pub (external) fn transfer_ownership(&mut self, new_owner: AccountId) -> bool {
            assert!(self.only_owner(env::caller()));
            self.owner.set(Some(new_owner));
            deposit_event(Event::OwnershipTransferred {
               from: Some(env::caller()),
               to: Some(new_owner)
            });
            true
        }

        pub (external) fn renounce_ownership(&mut self) -> bool {
            assert!(self.only_owner(env::caller()));
            self.owner.set(None);
            deposit_event(Event::OwnershipTransferred {
                from: Some(env::caller()),
                to: None
            });
            true
        }
    }

    impl Ownable {
        fn only_owner(&mut self, caller: AccountId) -> bool {
            if self.owner.is_some() {
                if caller == self.owner.unwrap() {
                    return true
                }
            }
            false
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
        let bob = AccountId::try_from([0x1; 32]).unwrap();

        let mut contract = Ownable::deploy_mock();
        assert_eq!(contract.is_owner(), true);
        env::test::set_caller(bob);
        assert_eq!(contract.is_owner(), false);

    }
    
    #[test]
    fn get_owner() {
        let alice = AccountId::try_from([0x0; 32]).unwrap();

        let mut contract = Ownable::deploy_mock();
        assert_eq!(contract.get_owner(), Some(alice));
    }

    #[test]
    fn transfer_ownership_works() {
        let bob = AccountId::try_from([0x1; 32]).unwrap();

        let mut contract = Ownable::deploy_mock();
        assert!(contract.transfer_ownership(bob));
        assert_eq!(contract.get_owner(), Some(bob));
    }

    #[test]
    #[should_panic]
    fn transfer_ownership_fails() {
        let alice = AccountId::try_from([0x0; 32]).unwrap();
        let bob = AccountId::try_from([0x1; 32]).unwrap();

        let mut contract = Ownable::deploy_mock();
        assert!(contract.transfer_ownership(bob));
        // This line should panic since non-owner cannot call this function.
        contract.transfer_ownership(alice);
    }

    #[test]

    fn renounce_ownership_works()  {
        let mut contract = Ownable::deploy_mock();
        assert!(contract.renounce_ownership());
        assert_eq!(contract.get_owner(), None);
    }
}
