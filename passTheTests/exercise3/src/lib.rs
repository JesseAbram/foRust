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
