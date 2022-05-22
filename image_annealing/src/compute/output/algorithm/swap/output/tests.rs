mod swap_ratio {
    use super::super::SwapRatio;
    use std::fmt;

    struct TestSwapRatio(usize, usize);

    impl SwapRatio for TestSwapRatio {
        fn total(&self) -> usize {
            self.0
        }

        fn accepted(&self) -> usize {
            self.1
        }
    }

    impl fmt::Display for TestSwapRatio {
        fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
            unimplemented!()
        }
    }

    #[test]
    fn zero_total() {
        let ratio = TestSwapRatio(0, 0);
        assert!(ratio.is_none_accepted());
        assert_eq!(ratio.accepted_fraction(), 0.0);
    }

    #[test]
    fn nonzero_total_zero_accepted() {
        let ratio = TestSwapRatio(2, 0);
        assert!(ratio.is_none_accepted());
        assert_eq!(ratio.accepted_fraction(), 0.0);
    }

    #[test]
    fn nonzero_total_some_accepted() {
        let ratio = TestSwapRatio(2, 1);
        assert!(!ratio.is_none_accepted());
        assert_eq!(ratio.accepted_fraction(), 0.5);
    }

    #[test]
    fn nonzero_total_all_accepted() {
        let ratio = TestSwapRatio(2, 2);
        assert!(!ratio.is_none_accepted());
        assert_eq!(ratio.accepted_fraction(), 1.0);
    }
}
