mod swap_parameters {
    use super::super::{SwapParameters, SwapPassSelection};

    mod new {
        use super::super::super::{SwapParameters, SwapPassSelection};
        use std::error::Error;

        #[test]
        fn new() -> Result<(), Box<dyn Error>> {
            let selection =
                SwapPassSelection::OFFSET_HORIZONTAL | SwapPassSelection::OFFSET_VERTICAL;
            let swap_acceptance_threshold = 3.0;
            let count_swap = true;
            let parameters = SwapParameters::new(selection, swap_acceptance_threshold, count_swap)?;
            assert_eq!(parameters.selection(), selection);
            assert_eq!(
                parameters.swap_acceptance_threshold(),
                swap_acceptance_threshold
            );
            assert_eq!(parameters.count_swap(), count_swap);
            Ok(())
        }

        #[test]
        fn empty_swap_pass_selection() {
            test_utils::assert_error_contains(
                SwapParameters::new(SwapPassSelection::empty(), 3.0, true),
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
            assert_eq!(parameters.swap_acceptance_threshold(), Default::default());
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

    mod from_selection_and_threshold {
        use super::super::super::{SwapParameters, SwapPassSelection};
        use std::error::Error;

        #[test]
        fn from_selection_and_threshold() -> Result<(), Box<dyn Error>> {
            let selection =
                SwapPassSelection::OFFSET_HORIZONTAL | SwapPassSelection::OFFSET_VERTICAL;
            let swap_acceptance_threshold = 3.0;
            let parameters =
                SwapParameters::from_selection_and_threshold(selection, swap_acceptance_threshold)?;
            assert_eq!(parameters.selection(), selection);
            assert_eq!(
                parameters.swap_acceptance_threshold(),
                swap_acceptance_threshold
            );
            assert!(!parameters.count_swap());
            Ok(())
        }

        #[test]
        fn empty_swap_pass_selection() {
            test_utils::assert_error_contains(
                SwapParameters::from_selection_and_threshold(SwapPassSelection::empty(), 3.0),
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
                swap_acceptance_threshold: Default::default(),
                count_swap: Default::default(),
            }
        );
    }
}
