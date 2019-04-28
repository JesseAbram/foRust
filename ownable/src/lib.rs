#![cfg_attr(not(any(test, feature = "std")), no_std)]

use ink_core::{
    env::println,
    memory::format,
    storage,
};
use ink_lang::contract;
use ink_core::env::AccountId;


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

    /** 
     * @return true if `msg.sender` is the owner of the contract.
     */       
    // fn is_owner(&mut self) -> bool {
        
    //     }

    /**
     * @dev Allows the current owner to relinquish control of the contract.
     * It will not be possible to call the functions with the `onlyOwner`
     * modifier anymore.
     * @notice Renouncing ownership will leave the contract without an owner,
     * thereby removing any functionality that is only available to the owner.
     */
    //  fn renounce_ownership(&mut self) {

    //  }

         
    /**
     * @dev Allows the current owner to transfer control of the contract to a newOwner.
     * @param newOwner The address to transfer ownership to.
     */


    /**
     * @dev Transfers control of the contract to a newOwner.
     * @param newOwner The address to transfer ownership to.
     */

      
    }
}

#[cfg(all(test, feature = "test-env"))]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    
    }
}
