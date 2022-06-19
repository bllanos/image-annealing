mod run_swap {
    use crate::config::{IterationCount, SwapParametersConfig, SwapStopConfig, SwapStopThreshold};
    use image_annealing::compute::{
        Algorithm, CreatePermutationAlgorithm, CreatePermutationInput, CreatePermutationParameters,
        Dispatcher, OutputStatus, PermuteAlgorithm, PermuteInput, PermuteParameters, SwapAlgorithm,
        SwapFullOutput, SwapInput, SwapParameters, SwapPartialOutput, SwapPass, SwapPassSequence,
        SwapPassSequenceSwapRatio, SwapPassSwapRatio, SwapRatio, ValidatePermutationAlgorithm,
        ValidatePermutationInput, ValidatePermutationParameters,
    };
    use image_annealing::image_utils::validation;
    use image_annealing::{
        CandidatePermutation, DisplacementGoal, ImageDimensions, ValidatedPermutation,
    };
    use std::error::Error;
    use std::fmt;
    use std::num::NonZeroUsize;

    #[derive(Clone)]
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
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "{} / {} ({:.2}%) swaps accepted",
                self.accepted(),
                self.total(),
                self.accepted_fraction() * 100.0
            )
        }
    }

    impl SwapPassSequenceSwapRatio for TestSwapRatio {
        fn passes<'a, 'b>(&'a self) -> Box<dyn Iterator<Item = &'a dyn SwapPassSwapRatio> + 'b>
        where
            'a: 'b,
        {
            unreachable!()
        }
    }

    #[derive(Clone)]
    struct RunSwapInput {
        candidate_permutation: Option<CandidatePermutation>,
        displacement_goal: Option<DisplacementGoal>,
        parameters: SwapParametersConfig,
    }

    struct SwapDispatcher {
        run_swap_input: RunSwapInput,
        output_swap_counts: Vec<TestSwapRatio>,
        output_permutations: <Vec<ValidatedPermutation> as IntoIterator>::IntoIter,
        swap_round_index: usize,
        step_index: usize,
        remaining_passes: Option<std::iter::Peekable<<SwapPassSequence as IntoIterator>::IntoIter>>,
        previous_pass: Option<SwapPass>,
        partial_output_call_count: usize,
    }

    impl SwapDispatcher {
        const FINAL_STEP_INDEX: usize = 4;

        pub fn new(
            run_swap_input: RunSwapInput,
            output_swap_counts: Vec<TestSwapRatio>,
            output_permutations: Vec<ValidatedPermutation>,
        ) -> Self {
            // TODO replace `1` with a value that depends on `run_swap_input.parameters`
            assert_eq!(output_permutations.len(), 1);
            let instance = Self {
                run_swap_input,
                output_swap_counts,
                output_permutations: output_permutations.into_iter(),
                swap_round_index: 0,
                step_index: 0,
                remaining_passes: None,
                previous_pass: None,
                partial_output_call_count: 0,
            };
            if instance.expected_count_swap_flag() {
                assert_eq!(
                    instance.output_swap_counts.len(),
                    instance.expected_number_of_rounds()
                );
            } else {
                assert!(instance.output_swap_counts.is_empty());
            }
            instance
        }

        fn expected_count_swap_flag(&self) -> bool {
            !matches!(
                self.run_swap_input.parameters.stop,
                SwapStopConfig::Bounded {
                    threshold: None,
                    ..
                }
            )
        }

        fn expected_number_of_rounds(&self) -> usize {
            match self.run_swap_input.parameters.stop {
                SwapStopConfig::Bounded {
                    iteration_count,
                    threshold: threshold_option,
                } => match threshold_option {
                    None => iteration_count.get(),
                    Some(threshold) => std::cmp::min(
                        self.output_swap_counts
                            .iter()
                            .take_while(|swap_count| match threshold {
                                SwapStopThreshold::SwapsAccepted(accepted) => {
                                    swap_count.accepted() > accepted
                                }
                                SwapStopThreshold::SwapAcceptanceFraction(fraction) => {
                                    swap_count.accepted_fraction() > fraction.get()
                                }
                            })
                            .count()
                            + 1,
                        iteration_count.get(),
                    ),
                },
                SwapStopConfig::Unbounded(threshold) => {
                    self.output_swap_counts
                        .iter()
                        .take_while(|swap_count| match threshold {
                            SwapStopThreshold::SwapsAccepted(accepted) => {
                                swap_count.accepted() > accepted
                            }
                            SwapStopThreshold::SwapAcceptanceFraction(fraction) => {
                                swap_count.accepted_fraction() > fraction.get()
                            }
                        })
                        .count()
                        + 1
                }
            }
        }
    }

    impl Dispatcher for SwapDispatcher {
        fn create_permutation(
            self: Box<Self>,
            _input: CreatePermutationInput,
            _parameters: &CreatePermutationParameters,
        ) -> Box<CreatePermutationAlgorithm> {
            unreachable!()
        }

        fn permute(
            self: Box<Self>,
            _input: PermuteInput,
            _parameters: &PermuteParameters,
        ) -> Box<PermuteAlgorithm> {
            unreachable!()
        }

        fn swap(
            mut self: Box<Self>,
            input: SwapInput,
            parameters: &SwapParameters,
        ) -> Box<SwapAlgorithm> {
            assert!(self.swap_round_index < self.expected_number_of_rounds());
            assert_eq!(
                parameters,
                &SwapParameters {
                    sequence: self.run_swap_input.parameters.swap_pass_sequence,
                    swap_acceptance_threshold: self
                        .run_swap_input
                        .parameters
                        .swap_acceptance_threshold,
                    count_swap: self.expected_count_swap_flag()
                }
            );
            if self.swap_round_index == 0 {
                assert_eq!(
                    input.candidate_permutation,
                    self.run_swap_input.candidate_permutation,
                );
                assert_eq!(
                    input.displacement_goal,
                    self.run_swap_input.displacement_goal
                );
            } else {
                assert_eq!(
                    input,
                    SwapInput {
                        candidate_permutation: None,
                        displacement_goal: None
                    }
                );
            }
            self.remaining_passes = Some(
                self.run_swap_input
                    .parameters
                    .swap_pass_sequence
                    .into_iter()
                    .peekable(),
            );
            self.previous_pass = None;
            self.swap_round_index += 1;
            self
        }

        fn validate_permutation(
            self: Box<Self>,
            _input: ValidatePermutationInput,
            _parameters: &ValidatePermutationParameters,
        ) -> Box<ValidatePermutationAlgorithm> {
            unreachable!()
        }
    }

    impl Algorithm<SwapPartialOutput, SwapFullOutput> for SwapDispatcher {
        fn step(&mut self) -> Result<OutputStatus, Box<dyn Error>> {
            let status = match self.step_index {
                0 => OutputStatus::NoNewOutput,
                1 => {
                    if self.expected_count_swap_flag() {
                        OutputStatus::NoNewOutput
                    } else {
                        OutputStatus::NewPartialOutput
                    }
                }
                2 => {
                    if self.expected_count_swap_flag() {
                        OutputStatus::NoNewOutput
                    } else {
                        OutputStatus::NewPartialAndFullOutput
                    }
                }
                3 => {
                    let (pass_option, is_last_pass) = self
                        .remaining_passes
                        .as_mut()
                        .map(|iter| (iter.next(), iter.peek().is_none()))
                        .unwrap();
                    match pass_option {
                        Some(_) => {
                            self.previous_pass = pass_option;
                            if self.expected_count_swap_flag() {
                                OutputStatus::NewFullOutput
                            } else {
                                if is_last_pass {
                                    self.step_index += 1;
                                    OutputStatus::FinalFullOutput
                                } else {
                                    OutputStatus::NewFullOutput
                                }
                            }
                        }
                        None => {
                            self.step_index += 1;
                            if self.expected_count_swap_flag() {
                                OutputStatus::FinalPartialOutput
                            } else {
                                unreachable!()
                            }
                        }
                    }
                }
                _ => unreachable!(),
            };
            if self.step_index < 3 {
                self.step_index += 1;
            }
            Ok(status)
        }

        fn partial_output(&mut self) -> Option<SwapPartialOutput> {
            if self.expected_count_swap_flag() && self.step_index == Self::FINAL_STEP_INDEX {
                self.partial_output_call_count += 1;
                Some(SwapPartialOutput {
                    counts: Box::new(self.output_swap_counts[self.swap_round_index - 1].clone()),
                })
            } else {
                unreachable!()
            }
        }

        fn full_output(&mut self) -> Option<SwapFullOutput> {
            let expected_rounds = self.expected_number_of_rounds();
            if self.swap_round_index == expected_rounds && self.step_index == Self::FINAL_STEP_INDEX
            {
                if self.expected_count_swap_flag() {
                    // Assert that client code accessed swap count values every round,
                    // even if only to print them to the user.
                    assert_eq!(self.partial_output_call_count, expected_rounds);
                }

                Some(SwapFullOutput {
                    input: None,
                    output_permutation: self.output_permutations.next().unwrap(),
                    pass: self.previous_pass.unwrap(),
                })
            } else {
                unreachable!()
            }
        }

        fn return_to_dispatcher(mut self: Box<Self>) -> Box<dyn Dispatcher> {
            self.step_index = 0;
            self
        }
    }

    fn test_run_swap_with_parameters(
        parameters: SwapParametersConfig,
        swap_ratios: Vec<TestSwapRatio>,
    ) -> Result<(), Box<dyn Error>> {
        // TODO replace `1` with a value that depends on `run_swap()` parameters
        let dimensions = ImageDimensions::new(1, 1)?;
        let permutation = test_utils::permutation::identity_with_dimensions(
            dimensions.width(),
            dimensions.height(),
        )
        .permutation;
        let candidate_permutation = CandidatePermutation::new(permutation.clone()).unwrap();
        // TODO replace `1` with a value that depends on `run_swap()` parameters
        let validated_permutations = (0..1)
            .map(|i| unsafe {
                validation::vector_field_into_validated_permutation_unchecked(
                    test_utils::permutation::line_with_first_texel_moved(dimensions.width(), i)
                        .permutation,
                )
            })
            .collect::<Vec<_>>();
        let run_swap_input = RunSwapInput {
            candidate_permutation: Some(candidate_permutation),
            displacement_goal: Some(
                DisplacementGoal::from_raw_candidate_permutation(permutation).unwrap(),
            ),
            parameters,
        };

        let dispatcher = Box::new(SwapDispatcher::new(
            run_swap_input.clone(),
            swap_ratios,
            validated_permutations.clone(),
        ));
        assert_eq!(
            super::super::run_swap(
                dispatcher,
                run_swap_input.candidate_permutation,
                run_swap_input.displacement_goal,
                &run_swap_input.parameters
            )
            .collect::<Result<Vec<_>, _>>()?,
            validated_permutations
        );
        Ok(())
    }

    fn make_base_swap_parameters_config() -> SwapParametersConfig {
        SwapParametersConfig {
            stop: SwapStopConfig::Bounded {
                iteration_count: IterationCount(NonZeroUsize::new(1).unwrap()),
                threshold: None,
            },
            swap_acceptance_threshold: 2.0,
            swap_pass_sequence: SwapPassSequence::from_passes([
                SwapPass::OffsetVertical,
                SwapPass::Horizontal,
            ])
            .unwrap(),
        }
    }

    mod one_swap_rounds {
        mod bounded {
            use super::super::TestSwapRatio;
            use crate::config::{
                IterationCount, SwapParametersConfig, SwapStopConfig, SwapStopThreshold,
            };
            use std::error::Error;
            use std::num::NonZeroUsize;

            const ITERATION_COUNT_LIMIT: usize = 1;

            #[test]
            fn iteration_count_only() -> Result<(), Box<dyn Error>> {
                let parameters = SwapParametersConfig {
                    stop: SwapStopConfig::Bounded {
                        iteration_count: IterationCount(
                            NonZeroUsize::new(ITERATION_COUNT_LIMIT).unwrap(),
                        ),
                        threshold: None,
                    },
                    ..super::super::make_base_swap_parameters_config()
                };
                let swap_ratios = Vec::new();
                super::super::test_run_swap_with_parameters(parameters, swap_ratios)
            }

            #[test]
            fn iteration_count_first() -> Result<(), Box<dyn Error>> {
                let parameters = SwapParametersConfig {
                    stop: SwapStopConfig::Bounded {
                        iteration_count: IterationCount(
                            NonZeroUsize::new(ITERATION_COUNT_LIMIT).unwrap(),
                        ),
                        threshold: Some(SwapStopThreshold::SwapsAccepted(0)),
                    },
                    ..super::super::make_base_swap_parameters_config()
                };
                let swap_ratios = vec![TestSwapRatio(1, 1)];
                super::super::test_run_swap_with_parameters(parameters, swap_ratios)
            }

            mod threshold_first {
                use super::super::super::TestSwapRatio;
                use crate::config::{
                    IterationCount, NonnegativeProperFraction, SwapParametersConfig,
                    SwapStopConfig, SwapStopThreshold,
                };
                use std::error::Error;
                use std::num::NonZeroUsize;

                const ITERATION_COUNT_LIMIT_ADD_ONE: usize = 2;

                #[test]
                fn none_accepted() -> Result<(), Box<dyn Error>> {
                    let parameters = SwapParametersConfig {
                        stop: SwapStopConfig::Bounded {
                            iteration_count: IterationCount(
                                NonZeroUsize::new(ITERATION_COUNT_LIMIT_ADD_ONE).unwrap(),
                            ),
                            threshold: Some(SwapStopThreshold::SwapsAccepted(0)),
                        },
                        ..super::super::super::make_base_swap_parameters_config()
                    };
                    let swap_ratios = vec![TestSwapRatio(0, 0)];
                    super::super::super::test_run_swap_with_parameters(parameters, swap_ratios)
                }

                #[test]
                fn some_accepted() -> Result<(), Box<dyn Error>> {
                    let parameters = SwapParametersConfig {
                        stop: SwapStopConfig::Bounded {
                            iteration_count: IterationCount(
                                NonZeroUsize::new(ITERATION_COUNT_LIMIT_ADD_ONE).unwrap(),
                            ),
                            threshold: Some(SwapStopThreshold::SwapsAccepted(1)),
                        },
                        ..super::super::super::make_base_swap_parameters_config()
                    };
                    let swap_ratios = vec![TestSwapRatio(2, 1)];
                    super::super::super::test_run_swap_with_parameters(parameters, swap_ratios)
                }

                #[test]
                fn some_accepted_fraction() -> Result<(), Box<dyn Error>> {
                    let parameters = SwapParametersConfig {
                        stop: SwapStopConfig::Bounded {
                            iteration_count: IterationCount(
                                NonZeroUsize::new(ITERATION_COUNT_LIMIT_ADD_ONE).unwrap(),
                            ),
                            threshold: Some(SwapStopThreshold::SwapAcceptanceFraction(
                                NonnegativeProperFraction::new(0.5)?,
                            )),
                        },
                        ..super::super::super::make_base_swap_parameters_config()
                    };
                    let swap_ratios = vec![TestSwapRatio(2, 1)];
                    super::super::super::test_run_swap_with_parameters(parameters, swap_ratios)
                }

                #[test]
                fn all_accepted() -> Result<(), Box<dyn Error>> {
                    let parameters = SwapParametersConfig {
                        stop: SwapStopConfig::Bounded {
                            iteration_count: IterationCount(
                                NonZeroUsize::new(ITERATION_COUNT_LIMIT_ADD_ONE).unwrap(),
                            ),
                            threshold: Some(SwapStopThreshold::SwapsAccepted(2)),
                        },
                        ..super::super::super::make_base_swap_parameters_config()
                    };
                    let swap_ratios = vec![TestSwapRatio(2, 2)];
                    super::super::super::test_run_swap_with_parameters(parameters, swap_ratios)
                }
            }
        }

        mod unbounded {
            use super::super::TestSwapRatio;
            use crate::config::{
                NonnegativeProperFraction, SwapParametersConfig, SwapStopConfig, SwapStopThreshold,
            };
            use std::error::Error;

            #[test]
            fn none_accepted() -> Result<(), Box<dyn Error>> {
                let parameters = SwapParametersConfig {
                    stop: SwapStopConfig::Unbounded(SwapStopThreshold::SwapsAccepted(0)),
                    ..super::super::make_base_swap_parameters_config()
                };
                let swap_ratios = vec![TestSwapRatio(0, 0)];
                super::super::test_run_swap_with_parameters(parameters, swap_ratios)
            }

            #[test]
            fn some_accepted() -> Result<(), Box<dyn Error>> {
                let parameters = SwapParametersConfig {
                    stop: SwapStopConfig::Unbounded(SwapStopThreshold::SwapsAccepted(1)),
                    ..super::super::make_base_swap_parameters_config()
                };
                let swap_ratios = vec![TestSwapRatio(2, 1)];
                super::super::test_run_swap_with_parameters(parameters, swap_ratios)
            }

            #[test]
            fn some_accepted_fraction() -> Result<(), Box<dyn Error>> {
                let parameters = SwapParametersConfig {
                    stop: SwapStopConfig::Unbounded(SwapStopThreshold::SwapAcceptanceFraction(
                        NonnegativeProperFraction::new(0.5)?,
                    )),
                    ..super::super::make_base_swap_parameters_config()
                };
                let swap_ratios = vec![TestSwapRatio(2, 1)];
                super::super::test_run_swap_with_parameters(parameters, swap_ratios)
            }

            #[test]
            fn all_accepted() -> Result<(), Box<dyn Error>> {
                let parameters = SwapParametersConfig {
                    stop: SwapStopConfig::Unbounded(SwapStopThreshold::SwapsAccepted(2)),
                    ..super::super::make_base_swap_parameters_config()
                };
                let swap_ratios = vec![TestSwapRatio(2, 2)];
                super::super::test_run_swap_with_parameters(parameters, swap_ratios)
            }
        }
    }

    mod two_swap_rounds {
        mod bounded {
            use super::super::TestSwapRatio;
            use crate::config::{
                IterationCount, SwapParametersConfig, SwapStopConfig, SwapStopThreshold,
            };
            use std::error::Error;
            use std::num::NonZeroUsize;

            const ITERATION_COUNT_LIMIT: usize = 2;

            #[test]
            fn iteration_count_only() -> Result<(), Box<dyn Error>> {
                let parameters = SwapParametersConfig {
                    stop: SwapStopConfig::Bounded {
                        iteration_count: IterationCount(
                            NonZeroUsize::new(ITERATION_COUNT_LIMIT).unwrap(),
                        ),
                        threshold: None,
                    },
                    ..super::super::make_base_swap_parameters_config()
                };
                let swap_ratios = Vec::new();
                super::super::test_run_swap_with_parameters(parameters, swap_ratios)
            }

            #[test]
            fn iteration_count_first() -> Result<(), Box<dyn Error>> {
                let parameters = SwapParametersConfig {
                    stop: SwapStopConfig::Bounded {
                        iteration_count: IterationCount(
                            NonZeroUsize::new(ITERATION_COUNT_LIMIT).unwrap(),
                        ),
                        threshold: Some(SwapStopThreshold::SwapsAccepted(0)),
                    },
                    ..super::super::make_base_swap_parameters_config()
                };
                let swap_ratios = vec![TestSwapRatio(1, 1); 2];
                super::super::test_run_swap_with_parameters(parameters, swap_ratios)
            }

            mod threshold_first {
                use super::super::super::TestSwapRatio;
                use crate::config::{
                    IterationCount, NonnegativeProperFraction, SwapParametersConfig,
                    SwapStopConfig, SwapStopThreshold,
                };
                use std::error::Error;
                use std::num::NonZeroUsize;

                const ITERATION_COUNT_LIMIT_ADD_ONE: usize = 3;

                #[test]
                fn none_accepted() -> Result<(), Box<dyn Error>> {
                    let parameters = SwapParametersConfig {
                        stop: SwapStopConfig::Bounded {
                            iteration_count: IterationCount(
                                NonZeroUsize::new(ITERATION_COUNT_LIMIT_ADD_ONE).unwrap(),
                            ),
                            threshold: Some(SwapStopThreshold::SwapsAccepted(0)),
                        },
                        ..super::super::super::make_base_swap_parameters_config()
                    };
                    let swap_ratios = vec![TestSwapRatio(2, 1), TestSwapRatio(2, 0)];
                    super::super::super::test_run_swap_with_parameters(parameters, swap_ratios)
                }

                #[test]
                fn some_accepted() -> Result<(), Box<dyn Error>> {
                    let parameters = SwapParametersConfig {
                        stop: SwapStopConfig::Bounded {
                            iteration_count: IterationCount(
                                NonZeroUsize::new(ITERATION_COUNT_LIMIT_ADD_ONE).unwrap(),
                            ),
                            threshold: Some(SwapStopThreshold::SwapsAccepted(1)),
                        },
                        ..super::super::super::make_base_swap_parameters_config()
                    };
                    let swap_ratios = vec![TestSwapRatio(2, 2), TestSwapRatio(2, 1)];
                    super::super::super::test_run_swap_with_parameters(parameters, swap_ratios)
                }

                #[test]
                fn some_accepted_fraction() -> Result<(), Box<dyn Error>> {
                    let parameters = SwapParametersConfig {
                        stop: SwapStopConfig::Bounded {
                            iteration_count: IterationCount(
                                NonZeroUsize::new(ITERATION_COUNT_LIMIT_ADD_ONE).unwrap(),
                            ),
                            threshold: Some(SwapStopThreshold::SwapAcceptanceFraction(
                                NonnegativeProperFraction::new(0.5)?,
                            )),
                        },
                        ..super::super::super::make_base_swap_parameters_config()
                    };
                    let swap_ratios = vec![TestSwapRatio(2, 2), TestSwapRatio(2, 1)];
                    super::super::super::test_run_swap_with_parameters(parameters, swap_ratios)
                }
            }
        }

        mod unbounded {
            use super::super::TestSwapRatio;
            use crate::config::{
                NonnegativeProperFraction, SwapParametersConfig, SwapStopConfig, SwapStopThreshold,
            };
            use std::error::Error;

            #[test]
            fn none_accepted() -> Result<(), Box<dyn Error>> {
                let parameters = SwapParametersConfig {
                    stop: SwapStopConfig::Unbounded(SwapStopThreshold::SwapsAccepted(0)),
                    ..super::super::make_base_swap_parameters_config()
                };
                let swap_ratios = vec![TestSwapRatio(2, 1), TestSwapRatio(2, 0)];
                super::super::test_run_swap_with_parameters(parameters, swap_ratios)
            }

            #[test]
            fn some_accepted() -> Result<(), Box<dyn Error>> {
                let parameters = SwapParametersConfig {
                    stop: SwapStopConfig::Unbounded(SwapStopThreshold::SwapsAccepted(1)),
                    ..super::super::make_base_swap_parameters_config()
                };
                let swap_ratios = vec![TestSwapRatio(2, 2), TestSwapRatio(2, 1)];
                super::super::test_run_swap_with_parameters(parameters, swap_ratios)
            }

            #[test]
            fn some_accepted_fraction() -> Result<(), Box<dyn Error>> {
                let parameters = SwapParametersConfig {
                    stop: SwapStopConfig::Unbounded(SwapStopThreshold::SwapAcceptanceFraction(
                        NonnegativeProperFraction::new(0.5)?,
                    )),
                    ..super::super::make_base_swap_parameters_config()
                };
                let swap_ratios = vec![TestSwapRatio(2, 2), TestSwapRatio(2, 1)];
                super::super::test_run_swap_with_parameters(parameters, swap_ratios)
            }
        }
    }

    mod three_swap_rounds {
        mod bounded {
            use super::super::TestSwapRatio;
            use crate::config::{
                IterationCount, SwapParametersConfig, SwapStopConfig, SwapStopThreshold,
            };
            use std::error::Error;
            use std::num::NonZeroUsize;

            const ITERATION_COUNT_LIMIT: usize = 3;

            #[test]
            fn iteration_count_only() -> Result<(), Box<dyn Error>> {
                let parameters = SwapParametersConfig {
                    stop: SwapStopConfig::Bounded {
                        iteration_count: IterationCount(
                            NonZeroUsize::new(ITERATION_COUNT_LIMIT).unwrap(),
                        ),
                        threshold: None,
                    },
                    ..super::super::make_base_swap_parameters_config()
                };
                let swap_ratios = Vec::new();
                super::super::test_run_swap_with_parameters(parameters, swap_ratios)
            }

            #[test]
            fn iteration_count_first() -> Result<(), Box<dyn Error>> {
                let parameters = SwapParametersConfig {
                    stop: SwapStopConfig::Bounded {
                        iteration_count: IterationCount(
                            NonZeroUsize::new(ITERATION_COUNT_LIMIT).unwrap(),
                        ),
                        threshold: Some(SwapStopThreshold::SwapsAccepted(0)),
                    },
                    ..super::super::make_base_swap_parameters_config()
                };
                let swap_ratios = vec![TestSwapRatio(1, 1); 3];
                super::super::test_run_swap_with_parameters(parameters, swap_ratios)
            }

            mod threshold_first {
                use super::super::super::TestSwapRatio;
                use crate::config::{
                    IterationCount, NonnegativeProperFraction, SwapParametersConfig,
                    SwapStopConfig, SwapStopThreshold,
                };
                use std::error::Error;
                use std::num::NonZeroUsize;

                const ITERATION_COUNT_LIMIT_ADD_ONE: usize = 4;

                #[test]
                fn none_accepted() -> Result<(), Box<dyn Error>> {
                    let parameters = SwapParametersConfig {
                        stop: SwapStopConfig::Bounded {
                            iteration_count: IterationCount(
                                NonZeroUsize::new(ITERATION_COUNT_LIMIT_ADD_ONE).unwrap(),
                            ),
                            threshold: Some(SwapStopThreshold::SwapsAccepted(0)),
                        },
                        ..super::super::super::make_base_swap_parameters_config()
                    };
                    let swap_ratios = vec![
                        TestSwapRatio(2, 1),
                        TestSwapRatio(2, 1),
                        TestSwapRatio(2, 0),
                    ];
                    super::super::super::test_run_swap_with_parameters(parameters, swap_ratios)
                }

                #[test]
                fn some_accepted() -> Result<(), Box<dyn Error>> {
                    let parameters = SwapParametersConfig {
                        stop: SwapStopConfig::Bounded {
                            iteration_count: IterationCount(
                                NonZeroUsize::new(ITERATION_COUNT_LIMIT_ADD_ONE).unwrap(),
                            ),
                            threshold: Some(SwapStopThreshold::SwapsAccepted(1)),
                        },
                        ..super::super::super::make_base_swap_parameters_config()
                    };
                    let swap_ratios = vec![
                        TestSwapRatio(2, 2),
                        TestSwapRatio(2, 2),
                        TestSwapRatio(2, 1),
                    ];
                    super::super::super::test_run_swap_with_parameters(parameters, swap_ratios)
                }

                #[test]
                fn some_accepted_fraction() -> Result<(), Box<dyn Error>> {
                    let parameters = SwapParametersConfig {
                        stop: SwapStopConfig::Bounded {
                            iteration_count: IterationCount(
                                NonZeroUsize::new(ITERATION_COUNT_LIMIT_ADD_ONE).unwrap(),
                            ),
                            threshold: Some(SwapStopThreshold::SwapAcceptanceFraction(
                                NonnegativeProperFraction::new(0.5)?,
                            )),
                        },
                        ..super::super::super::make_base_swap_parameters_config()
                    };
                    let swap_ratios = vec![
                        TestSwapRatio(2, 2),
                        TestSwapRatio(2, 2),
                        TestSwapRatio(2, 1),
                    ];
                    super::super::super::test_run_swap_with_parameters(parameters, swap_ratios)
                }
            }
        }

        mod unbounded {
            use super::super::TestSwapRatio;
            use crate::config::{
                NonnegativeProperFraction, SwapParametersConfig, SwapStopConfig, SwapStopThreshold,
            };
            use std::error::Error;

            #[test]
            fn none_accepted() -> Result<(), Box<dyn Error>> {
                let parameters = SwapParametersConfig {
                    stop: SwapStopConfig::Unbounded(SwapStopThreshold::SwapsAccepted(0)),
                    ..super::super::make_base_swap_parameters_config()
                };
                let swap_ratios = vec![
                    TestSwapRatio(2, 1),
                    TestSwapRatio(2, 1),
                    TestSwapRatio(2, 0),
                ];
                super::super::test_run_swap_with_parameters(parameters, swap_ratios)
            }

            #[test]
            fn some_accepted() -> Result<(), Box<dyn Error>> {
                let parameters = SwapParametersConfig {
                    stop: SwapStopConfig::Unbounded(SwapStopThreshold::SwapsAccepted(1)),
                    ..super::super::make_base_swap_parameters_config()
                };
                let swap_ratios = vec![
                    TestSwapRatio(2, 2),
                    TestSwapRatio(2, 2),
                    TestSwapRatio(2, 1),
                ];
                super::super::test_run_swap_with_parameters(parameters, swap_ratios)
            }

            #[test]
            fn some_accepted_fraction() -> Result<(), Box<dyn Error>> {
                let parameters = SwapParametersConfig {
                    stop: SwapStopConfig::Unbounded(SwapStopThreshold::SwapAcceptanceFraction(
                        NonnegativeProperFraction::new(0.5)?,
                    )),
                    ..super::super::make_base_swap_parameters_config()
                };
                let swap_ratios = vec![
                    TestSwapRatio(2, 2),
                    TestSwapRatio(2, 2),
                    TestSwapRatio(2, 1),
                ];
                super::super::test_run_swap_with_parameters(parameters, swap_ratios)
            }
        }
    }
}
