mod swap_ratio {
    use super::super::super::super::super::link::swap::CountSwapOutputDataElement;
    use super::super::SwapRatio;

    #[test]
    fn zero_total() {
        let ratio = SwapRatio::new(0, 0.0);
        assert!(ratio.is_none_accepted());
        assert_eq!(ratio.accepted_fraction(), 0.0);
        assert_eq!(ratio.total(), 0);
        assert_eq!(ratio.accepted(), 0);
        assert_eq!(ratio.to_string(), "0 / 0 (0.00%) swaps accepted");
    }

    #[test]
    #[should_panic(
        expected = "number of accepted swaps, 1, is greater than the total number of swaps, 0"
    )]
    fn zero_total_nonzero_accepted() {
        SwapRatio::new(0, 1.0);
    }

    #[test]
    #[should_panic(expected = "number of accepted swaps, inf, is not finite")]
    fn infinite_accepted() {
        SwapRatio::new(0, CountSwapOutputDataElement::INFINITY);
    }

    #[test]
    #[should_panic(expected = "number of accepted swaps, -1, is negative")]
    fn negative_accepted() {
        SwapRatio::new(0, -1.0);
    }

    #[test]
    #[should_panic(expected = "number of accepted swaps, 1.5, is not an integer")]
    fn fractional_accepted() {
        SwapRatio::new(0, 1.5);
    }

    #[test]
    #[should_panic(
        expected = "number of accepted swaps, 2, is greater than the total number of swaps, 1"
    )]
    fn accepted_greater_than_total() {
        SwapRatio::new(1, 2.0);
    }

    #[test]
    fn nonzero_total_zero_accepted() {
        let ratio = SwapRatio::new(2, 0.0);
        assert!(ratio.is_none_accepted());
        assert_eq!(ratio.accepted_fraction(), 0.0);
        assert_eq!(ratio.total(), 2);
        assert_eq!(ratio.accepted(), 0);
        assert_eq!(ratio.to_string(), "0 / 2 (0.00%) swaps accepted");
    }

    #[test]
    fn nonzero_total_some_accepted() {
        let ratio = SwapRatio::new(2, 1.0);
        assert!(!ratio.is_none_accepted());
        assert_eq!(ratio.accepted_fraction(), 0.5);
        assert_eq!(ratio.total(), 2);
        assert_eq!(ratio.accepted(), 1);
        assert_eq!(ratio.to_string(), "1 / 2 (50.00%) swaps accepted");
    }

    #[test]
    fn nonzero_total_all_accepted() {
        let ratio = SwapRatio::new(2, 2.0);
        assert!(!ratio.is_none_accepted());
        assert_eq!(ratio.accepted_fraction(), 1.0);
        assert_eq!(ratio.total(), 2);
        assert_eq!(ratio.accepted(), 2);
        assert_eq!(ratio.to_string(), "2 / 2 (100.00%) swaps accepted");
    }
}
