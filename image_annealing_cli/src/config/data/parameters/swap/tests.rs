mod swap_stop_threshold_try_from_unverified_swap_stop_threshold {
    use super::super::super::super::number::NonnegativeProperFraction;
    use super::super::{SwapStopThreshold, UnverifiedSwapStopThreshold};
    use std::error::Error;

    #[test]
    fn accepted_count() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            SwapStopThreshold::try_from(UnverifiedSwapStopThreshold::SwapsAccepted(1))?,
            SwapStopThreshold::SwapsAccepted(1)
        );
        Ok(())
    }

    #[test]
    fn acceptance_fraction() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            SwapStopThreshold::try_from(UnverifiedSwapStopThreshold::SwapAcceptanceFraction(0.5))?,
            SwapStopThreshold::SwapAcceptanceFraction(NonnegativeProperFraction::new(0.5)?)
        );
        Ok(())
    }

    #[test]
    fn invalid_acceptance_fraction() {
        test_util::assert_error_contains(
            SwapStopThreshold::try_from(UnverifiedSwapStopThreshold::SwapAcceptanceFraction(-1.0)),
            "-1 is negative",
        );
    }
}

mod iteration_count {
    use super::super::{IterationCount, UnverifiedIterationCount};
    use std::error::Error;
    use std::num::NonZeroUsize;

    #[test]
    fn one() {
        assert_eq!(IterationCount(NonZeroUsize::new(1).unwrap()).get(), 1);
    }

    #[test]
    fn try_from() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            IterationCount::try_from(UnverifiedIterationCount(1))?.get(),
            1
        );
        Ok(())
    }

    #[test]
    fn try_from_zero() {
        test_util::assert_error_contains(
            IterationCount::try_from(UnverifiedIterationCount(0)),
            "iteration count cannot be zero",
        );
    }
}

mod swap_stop_config_try_from_unverified_swap_stop_config {
    mod bounded {
        use super::super::super::{
            IterationCount, SwapStopConfig, SwapStopThreshold, UnverifiedIterationCount,
            UnverifiedSwapStopConfig, UnverifiedSwapStopThreshold,
        };
        use std::error::Error;
        use std::num::NonZeroUsize;

        #[test]
        fn iteration_count_only() -> Result<(), Box<dyn Error>> {
            assert_eq!(
                SwapStopConfig::try_from(UnverifiedSwapStopConfig::Bounded {
                    iteration_count: UnverifiedIterationCount(1),
                    threshold: None
                })?,
                SwapStopConfig::Bounded {
                    iteration_count: IterationCount(NonZeroUsize::new(1).unwrap()),
                    threshold: None
                }
            );
            Ok(())
        }

        #[test]
        fn iteration_count_zero() {
            test_util::assert_error_contains(
                SwapStopConfig::try_from(UnverifiedSwapStopConfig::Bounded {
                    iteration_count: UnverifiedIterationCount(0),
                    threshold: None,
                }),
                "iteration count cannot be zero",
            );
        }

        #[test]
        fn iteration_count_and_threshold() -> Result<(), Box<dyn Error>> {
            assert_eq!(
                SwapStopConfig::try_from(UnverifiedSwapStopConfig::Bounded {
                    iteration_count: UnverifiedIterationCount(1),
                    threshold: Some(UnverifiedSwapStopThreshold::SwapsAccepted(0))
                })?,
                SwapStopConfig::Bounded {
                    iteration_count: IterationCount(NonZeroUsize::new(1).unwrap()),
                    threshold: Some(SwapStopThreshold::SwapsAccepted(0))
                }
            );
            Ok(())
        }

        #[test]
        fn threshold_one() {
            test_util::assert_error_contains(
                SwapStopConfig::try_from(UnverifiedSwapStopConfig::Bounded {
                    iteration_count: UnverifiedIterationCount(1),
                    threshold: Some(UnverifiedSwapStopThreshold::SwapAcceptanceFraction(1.0)),
                }),
                "1 is not less than one",
            );
        }
    }

    mod unbounded {
        use super::super::super::{
            SwapStopConfig, SwapStopThreshold, UnverifiedSwapStopConfig,
            UnverifiedSwapStopThreshold,
        };
        use std::error::Error;

        #[test]
        fn valid() -> Result<(), Box<dyn Error>> {
            assert_eq!(
                SwapStopConfig::try_from(UnverifiedSwapStopConfig::Unbounded(
                    UnverifiedSwapStopThreshold::SwapsAccepted(0)
                ))?,
                SwapStopConfig::Unbounded(SwapStopThreshold::SwapsAccepted(0))
            );
            Ok(())
        }

        #[test]
        fn threshold_one() {
            test_util::assert_error_contains(
                SwapStopConfig::try_from(UnverifiedSwapStopConfig::Unbounded(
                    UnverifiedSwapStopThreshold::SwapAcceptanceFraction(1.0),
                )),
                "1 is not less than one",
            );
        }
    }
}

mod swap_pass {
    mod from_and_into_swap_pass {
        use super::super::super::SwapPass;
        use image_annealing::compute::SwapPass as ImageAnnealingSwapPass;

        #[test]
        fn horizontal() {
            assert_eq!(
                SwapPass::from(ImageAnnealingSwapPass::Horizontal),
                SwapPass::Horizontal
            );
            assert_eq!(
                ImageAnnealingSwapPass::from(SwapPass::Horizontal),
                ImageAnnealingSwapPass::Horizontal
            );
        }

        #[test]
        fn vertical() {
            assert_eq!(
                SwapPass::from(ImageAnnealingSwapPass::Vertical),
                SwapPass::Vertical
            );
            assert_eq!(
                ImageAnnealingSwapPass::from(SwapPass::Vertical),
                ImageAnnealingSwapPass::Vertical
            );
        }

        #[test]
        fn offset_horizontal() {
            assert_eq!(
                SwapPass::from(ImageAnnealingSwapPass::OffsetHorizontal),
                SwapPass::OffsetHorizontal
            );
            assert_eq!(
                ImageAnnealingSwapPass::from(SwapPass::OffsetHorizontal),
                ImageAnnealingSwapPass::OffsetHorizontal
            );
        }

        #[test]
        fn offset_vertical() {
            assert_eq!(
                SwapPass::from(ImageAnnealingSwapPass::OffsetVertical),
                SwapPass::OffsetVertical
            );
            assert_eq!(
                ImageAnnealingSwapPass::from(SwapPass::OffsetVertical),
                ImageAnnealingSwapPass::OffsetVertical
            );
        }
    }
}

mod swap_parameters_config_try_from_unverified_swap_parameters_config {
    use super::super::{
        SwapParametersConfig, SwapPass, SwapStopConfig, SwapStopThreshold,
        UnverifiedSwapParametersConfig, UnverifiedSwapStopConfig, UnverifiedSwapStopThreshold,
    };
    use image_annealing::compute::{self, SwapPassSequence};
    use std::error::Error;

    #[test]
    fn valid() -> Result<(), Box<dyn Error>> {
        let swap_acceptance_threshold = 1.0;
        assert_eq!(
            SwapParametersConfig::try_from(UnverifiedSwapParametersConfig {
                stop: UnverifiedSwapStopConfig::Unbounded(
                    UnverifiedSwapStopThreshold::SwapsAccepted(0)
                ),
                swap_acceptance_threshold,
                swap_pass_sequence: vec![SwapPass::OffsetHorizontal, SwapPass::Vertical],
                output_intermediate_permutations: true,
            })?,
            SwapParametersConfig {
                stop: SwapStopConfig::Unbounded(SwapStopThreshold::SwapsAccepted(0)),
                swap_acceptance_threshold,
                swap_pass_sequence: SwapPassSequence::from_passes([
                    compute::SwapPass::OffsetHorizontal,
                    compute::SwapPass::Vertical
                ])?,
                output_intermediate_permutations: true,
            }
        );
        Ok(())
    }

    #[test]
    fn stop_threshold_one() {
        test_util::assert_error_contains(
            SwapParametersConfig::try_from(UnverifiedSwapParametersConfig {
                stop: UnverifiedSwapStopConfig::Unbounded(
                    UnverifiedSwapStopThreshold::SwapAcceptanceFraction(1.0),
                ),
                swap_acceptance_threshold: Default::default(),
                swap_pass_sequence: vec![SwapPass::OffsetHorizontal, SwapPass::Vertical],
                output_intermediate_permutations: true,
            }),
            "1 is not less than one",
        );
    }

    #[test]
    fn empty_sequence() {
        test_util::assert_error_contains(
            SwapParametersConfig::try_from(UnverifiedSwapParametersConfig {
                stop: UnverifiedSwapStopConfig::Unbounded(
                    UnverifiedSwapStopThreshold::SwapsAccepted(0),
                ),
                swap_acceptance_threshold: Default::default(),
                swap_pass_sequence: vec![],
                output_intermediate_permutations: true,
            }),
            "selection of swap passes is empty",
        );
    }

    #[test]
    fn duplicate_pass() {
        test_util::assert_error_contains(
            SwapParametersConfig::try_from(UnverifiedSwapParametersConfig {
                stop: UnverifiedSwapStopConfig::Unbounded(
                    UnverifiedSwapStopThreshold::SwapsAccepted(0),
                ),
                swap_acceptance_threshold: Default::default(),
                swap_pass_sequence: vec![
                    SwapPass::OffsetHorizontal,
                    SwapPass::Vertical,
                    SwapPass::OffsetHorizontal,
                ],
                output_intermediate_permutations: true,
            }),
            "attempt to select horizontal swaps, with offset pass multiple times",
        );
    }
}
