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
    #[should_panic(expected = "number of accepted swaps, NaN, is not finite")]
    fn nan_accepted() {
        SwapRatio::new(0, CountSwapOutputDataElement::NAN);
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

mod count_swap_operation_output_pass {
    use super::super::super::super::super::link::swap::SwapPass;
    use super::super::{CountSwapOperationOutputPass, SwapRatio};

    #[test]
    fn some_accepted() {
        let pass = SwapPass::Horizontal;
        let ratio = SwapRatio::new(2, 1.0);
        let output_pass = CountSwapOperationOutputPass {
            pass,
            swap_ratio: ratio,
        };
        assert_eq!(output_pass.pass(), pass);
        assert_eq!(output_pass.is_none_accepted(), ratio.is_none_accepted());
        assert_eq!(output_pass.accepted_fraction(), ratio.accepted_fraction());
        assert_eq!(output_pass.total(), ratio.total());
        assert_eq!(output_pass.accepted(), ratio.accepted());
        assert_eq!(
            output_pass.to_string(),
            format!("pass: {}, result: {}", pass, ratio)
        );
    }
}

mod count_swap_operation_output {
    use super::super::super::super::super::link::swap::{
        CountSwapOutput, SwapPass, SwapPassSelection,
    };
    use super::super::{CountSwapOperationOutput, CountSwapOperationOutputPass, SwapRatio};
    use crate::ImageDimensions;
    use image_annealing_shaders::constant;
    use std::convert::TryInto;
    use std::error::Error;

    #[test]
    fn zero_total() -> Result<(), Box<dyn Error>> {
        let counts = [0.0_f32; constant::count_swap::N_CHANNEL];
        let bytes: Vec<u8> = counts
            .iter()
            .flat_map(|&count| count.to_ne_bytes())
            .collect();
        let count_swap_output = CountSwapOutput::from_ne_bytes(bytes.as_slice().try_into()?);
        let selection = SwapPassSelection::all();
        let output = CountSwapOperationOutput::new(
            &count_swap_output,
            selection,
            &ImageDimensions::new(1, 1)?,
        );
        let passes = output.passes();
        assert_eq!(passes.len(), constant::count_swap::N_CHANNEL);
        passes
            .iter()
            .zip(selection.iter())
            .for_each(|(output_pass, pass)| {
                assert_eq!(output_pass.pass(), *pass);
                assert!(output_pass.is_none_accepted());
                assert_eq!(output_pass.accepted_fraction(), 0.0);
                assert_eq!(output_pass.total(), 0);
                assert_eq!(output_pass.accepted(), 0);
            });
        assert!(output.is_none_accepted());
        assert_eq!(output.accepted_fraction(), 0.0);
        assert_eq!(output.total(), 0);
        assert_eq!(output.accepted(), 0);
        assert_eq!(
            output.to_string(),
            "all passes: 0 / 0 (0.00%) swaps accepted
\tpass: horizontal swaps, no offset, result: 0 / 0 (0.00%) swaps accepted
\tpass: vertical swaps, no offset, result: 0 / 0 (0.00%) swaps accepted
\tpass: horizontal swaps, with offset, result: 0 / 0 (0.00%) swaps accepted
\tpass: vertical swaps, with offset, result: 0 / 0 (0.00%) swaps accepted
"
        );
        Ok(())
    }

    #[test]
    fn nonzero_total() -> Result<(), Box<dyn Error>> {
        let counts = [1.0_f32, 2.0_f32, 4.0_f32, 8.0_f32];
        let bytes: Vec<u8> = counts
            .iter()
            .flat_map(|&count| count.to_ne_bytes())
            .collect();
        let count_swap_output = CountSwapOutput::from_ne_bytes(bytes.as_slice().try_into()?);
        let pass1 = SwapPass::Horizontal;
        let pass2 = SwapPass::OffsetVertical;
        let selection = SwapPassSelection::from(pass1) | SwapPassSelection::from(pass2);
        let output = CountSwapOperationOutput::new(
            &count_swap_output,
            selection,
            &ImageDimensions::new(2, 9)?,
        );
        let passes = output.passes();
        assert_eq!(passes.len(), 2);
        passes.iter().eq([
            CountSwapOperationOutputPass {
                pass: pass1,
                swap_ratio: SwapRatio::new(9, 1.0_f32),
            },
            CountSwapOperationOutputPass {
                pass: pass2,
                swap_ratio: SwapRatio::new(8, 8.0_f32),
            },
        ]
        .iter());
        assert!(!output.is_none_accepted());
        assert_eq!(output.accepted_fraction(), 9.0_f64 / 17.0_f64);
        assert_eq!(output.total(), 17);
        assert_eq!(output.accepted(), 9);
        assert_eq!(
            output.to_string(),
            "all passes: 9 / 17 (52.94%) swaps accepted
\tpass: horizontal swaps, no offset, result: 1 / 9 (11.11%) swaps accepted
\tpass: vertical swaps, with offset, result: 8 / 8 (100.00%) swaps accepted
"
        );
        Ok(())
    }

    #[test]
    #[should_panic(
        expected = "number of accepted swaps, 1, is greater than the total number of swaps, 0"
    )]
    fn zero_total_nonzero_accepted() {
        let counts = [1.0_f32; constant::count_swap::N_CHANNEL];
        let bytes: Vec<u8> = counts
            .iter()
            .flat_map(|&count| count.to_ne_bytes())
            .collect();
        let count_swap_output =
            CountSwapOutput::from_ne_bytes(bytes.as_slice().try_into().unwrap());
        let selection = SwapPassSelection::all();
        CountSwapOperationOutput::new(
            &count_swap_output,
            selection,
            &ImageDimensions::new(1, 1).unwrap(),
        );
    }

    #[test]
    #[should_panic(
        expected = "number of accepted swaps, 3, is greater than the total number of swaps, 2"
    )]
    fn accepted_greater_than_total() {
        let counts = [3.0_f32; constant::count_swap::N_CHANNEL];
        let bytes: Vec<u8> = counts
            .iter()
            .flat_map(|&count| count.to_ne_bytes())
            .collect();
        let count_swap_output =
            CountSwapOutput::from_ne_bytes(bytes.as_slice().try_into().unwrap());
        let selection = SwapPassSelection::all();
        CountSwapOperationOutput::new(
            &count_swap_output,
            selection,
            &ImageDimensions::new(2, 2).unwrap(),
        );
    }
}
