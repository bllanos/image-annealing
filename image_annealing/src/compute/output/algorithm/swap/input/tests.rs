mod swap_parameters {
    use super::super::{SwapParameters, SwapPassSelection};

    mod new {
        use super::super::super::{SwapParameters, SwapPassSelection};
        use std::error::Error;

        #[test]
        fn new() -> Result<(), Box<dyn Error>> {
            let selection =
                SwapPassSelection::OFFSET_HORIZONTAL | SwapPassSelection::OFFSET_VERTICAL;
            let count_swap = true;
            let parameters = SwapParameters::new(selection, count_swap)?;
            assert_eq!(parameters.selection(), selection);
            assert_eq!(parameters.count_swap(), count_swap);
            Ok(())
        }

        #[test]
        fn empty_swap_pass_selection() {
            test_utils::assert_error_contains(
                SwapParameters::new(SwapPassSelection::empty(), true),
                "selection of swap passes is empty",
            );
        }
    }

    mod from_selection {
        use super::super::super::{SwapParameters, SwapPassSelection};
        use std::error::Error;

        #[test]
        fn from_selection() -> Result<(), Box<dyn Error>> {
            let selection =
                SwapPassSelection::OFFSET_HORIZONTAL | SwapPassSelection::OFFSET_VERTICAL;
            let parameters = SwapParameters::from_selection(selection)?;
            assert_eq!(parameters.selection(), selection);
            assert!(!parameters.count_swap());
            Ok(())
        }

        #[test]
        fn empty_swap_pass_selection() {
            test_utils::assert_error_contains(
                SwapParameters::from_selection(SwapPassSelection::empty()),
                "selection of swap passes is empty",
            );
        }
    }

    #[test]
    fn default() {
        assert_eq!(
            <SwapParameters as Default>::default(),
            SwapParameters {
                selection: SwapPassSelection::all(),
                count_swap: false,
            }
        );
    }
}
