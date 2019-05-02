#[cfg(all(test, feature = "test-env"))]
mod tests {
    use super::*;

    #[test]
    fn incrementer_works() {
        let mut contract = Incrementer::deploy_mock(5);
        assert_eq!(contract.get(), 5);
    }

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