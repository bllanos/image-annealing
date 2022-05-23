mod run_swap {
    use crate::config::{SwapParametersConfig, SwapStopConfig, SwapStopThreshold};
    use image_annealing::compute::{
        Algorithm, CreatePermutationAlgorithm, CreatePermutationInput, CreatePermutationParameters,
        Dispatcher, OutputStatus, PermuteAlgorithm, PermuteInput, PermuteParameters,
        SwapFullOutput, SwapInput, SwapParameters, SwapPartialOutput, SwapPassSelection,
        SwapPassSelectionSwapRatio, SwapPassSwapRatio, SwapRatio, ValidatePermutationAlgorithm,
        ValidatePermutationInput, ValidatePermutationParameters,
    };
    use image_annealing::image_utils::validation;
    use image_annealing::{CandidatePermutation, DisplacementGoal, ValidatedPermutation};
    use std::error::Error;
    use std::fmt;
    use test_utils::permutation::DimensionsAndPermutation;

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

    impl SwapPassSelectionSwapRatio for TestSwapRatio {
        fn passes<'a, 'b>(&'a self) -> Box<dyn Iterator<Item = &'a dyn SwapPassSwapRatio> + 'b>
        where
            'a: 'b,
        {
            unimplemented!()
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
        swap_round_index: usize,
        step_index: usize,
        partial_output_call_count: usize,
    }

    impl SwapDispatcher {
        const FINAL_STEP_INDEX: usize = 5;

        pub fn new(run_swap_input: RunSwapInput, output_swap_counts: Vec<TestSwapRatio>) -> Self {
            let instance = Self {
                run_swap_input,
                output_swap_counts,
                swap_round_index: 0,
                step_index: 0,
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
            unimplemented!()
        }

        fn permute(
            self: Box<Self>,
            _input: PermuteInput,
            _parameters: &PermuteParameters,
        ) -> Box<PermuteAlgorithm> {
            unimplemented!()
        }

        fn swap(
            mut self: Box<Self>,
            input: SwapInput,
            parameters: &SwapParameters,
        ) -> Box<dyn Algorithm<SwapPartialOutput, SwapFullOutput>> {
            assert!(self.swap_round_index < self.expected_number_of_rounds());
            assert_eq!(
                parameters,
                // TODO: Use SwapPassSelection from self.run_swap_input.parameters
                &SwapParameters::new(
                    SwapPassSelection::HORIZONTAL,
                    self.expected_count_swap_flag()
                )
                .unwrap()
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
            self.swap_round_index += 1;
            self
        }

        fn validate_permutation(
            self: Box<Self>,
            _input: ValidatePermutationInput,
            _parameters: &ValidatePermutationParameters,
        ) -> Box<ValidatePermutationAlgorithm> {
            unimplemented!()
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
                2 => OutputStatus::NewFullOutput,
                3 => {
                    if self.expected_count_swap_flag() {
                        OutputStatus::NoNewOutput
                    } else {
                        OutputStatus::NewPartialAndFullOutput
                    }
                }
                4 => {
                    if self.expected_count_swap_flag() {
                        OutputStatus::FinalPartialAndFullOutput
                    } else {
                        OutputStatus::FinalFullOutput
                    }
                }
                _ => unreachable!(),
            };
            self.step_index += 1;
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
                let output_permutation_buffer = self
                    .run_swap_input
                    .candidate_permutation
                    .take()
                    .unwrap()
                    .into_inner();
                Some(SwapFullOutput {
                    input_permutation: None,
                    input_displacement_goal: None,
                    output_permutation: unsafe {
                        validation::vector_field_into_validated_permutation_unchecked(
                            output_permutation_buffer,
                        )
                    },
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

    fn make_test_data(parameters: SwapParametersConfig) -> (RunSwapInput, ValidatedPermutation) {
        let DimensionsAndPermutation { permutation, .. } = test_utils::permutation::non_identity();
        let candidate_permutation = CandidatePermutation::new(permutation.clone()).unwrap();
        let validated_permutation = unsafe {
            validation::vector_field_into_validated_permutation_unchecked(permutation.clone())
        };
        (
            RunSwapInput {
                candidate_permutation: Some(candidate_permutation),
                displacement_goal: Some(
                    DisplacementGoal::from_raw_candidate_permutation(permutation).unwrap(),
                ),
                parameters,
            },
            validated_permutation,
        )
    }

    mod one_swap_rounds {
        mod bounded {
            use super::super::super::super::run_swap;
            use super::super::{SwapDispatcher, TestSwapRatio};
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
                };
                let swap_ratios = Vec::new();
                let (run_swap_input, validated_permutation) =
                    super::super::make_test_data(parameters.clone());
                let dispatcher = Box::new(SwapDispatcher::new(run_swap_input.clone(), swap_ratios));
                assert_eq!(
                    run_swap(
                        dispatcher,
                        run_swap_input.candidate_permutation,
                        run_swap_input.displacement_goal,
                        &run_swap_input.parameters
                    )?,
                    validated_permutation
                );
                Ok(())
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
                };
                let swap_ratios = vec![TestSwapRatio(1, 1)];
                let (run_swap_input, validated_permutation) =
                    super::super::make_test_data(parameters.clone());
                let dispatcher = Box::new(SwapDispatcher::new(run_swap_input.clone(), swap_ratios));
                assert_eq!(
                    run_swap(
                        dispatcher,
                        run_swap_input.candidate_permutation,
                        run_swap_input.displacement_goal,
                        &run_swap_input.parameters
                    )?,
                    validated_permutation
                );
                Ok(())
            }

            mod threshold_first {
                use super::super::super::super::super::run_swap;
                use super::super::super::{SwapDispatcher, TestSwapRatio};
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
                    };
                    let swap_ratios = vec![TestSwapRatio(0, 0)];
                    let (run_swap_input, validated_permutation) =
                        super::super::super::make_test_data(parameters.clone());
                    let dispatcher =
                        Box::new(SwapDispatcher::new(run_swap_input.clone(), swap_ratios));
                    assert_eq!(
                        run_swap(
                            dispatcher,
                            run_swap_input.candidate_permutation,
                            run_swap_input.displacement_goal,
                            &run_swap_input.parameters
                        )?,
                        validated_permutation
                    );
                    Ok(())
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
                    };
                    let swap_ratios = vec![TestSwapRatio(2, 1)];
                    let (run_swap_input, validated_permutation) =
                        super::super::super::make_test_data(parameters.clone());
                    let dispatcher =
                        Box::new(SwapDispatcher::new(run_swap_input.clone(), swap_ratios));
                    assert_eq!(
                        run_swap(
                            dispatcher,
                            run_swap_input.candidate_permutation,
                            run_swap_input.displacement_goal,
                            &run_swap_input.parameters
                        )?,
                        validated_permutation
                    );
                    Ok(())
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
                    };
                    let swap_ratios = vec![TestSwapRatio(2, 1)];
                    let (run_swap_input, validated_permutation) =
                        super::super::super::make_test_data(parameters.clone());
                    let dispatcher =
                        Box::new(SwapDispatcher::new(run_swap_input.clone(), swap_ratios));
                    assert_eq!(
                        run_swap(
                            dispatcher,
                            run_swap_input.candidate_permutation,
                            run_swap_input.displacement_goal,
                            &run_swap_input.parameters
                        )?,
                        validated_permutation
                    );
                    Ok(())
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
                    };
                    let swap_ratios = vec![TestSwapRatio(2, 2)];
                    let (run_swap_input, validated_permutation) =
                        super::super::super::make_test_data(parameters.clone());
                    let dispatcher =
                        Box::new(SwapDispatcher::new(run_swap_input.clone(), swap_ratios));
                    assert_eq!(
                        run_swap(
                            dispatcher,
                            run_swap_input.candidate_permutation,
                            run_swap_input.displacement_goal,
                            &run_swap_input.parameters
                        )?,
                        validated_permutation
                    );
                    Ok(())
                }
            }
        }

        mod unbounded {
            use super::super::super::super::run_swap;
            use super::super::{SwapDispatcher, TestSwapRatio};
            use crate::config::{
                NonnegativeProperFraction, SwapParametersConfig, SwapStopConfig, SwapStopThreshold,
            };
            use std::error::Error;

            #[test]
            fn none_accepted() -> Result<(), Box<dyn Error>> {
                let parameters = SwapParametersConfig {
                    stop: SwapStopConfig::Unbounded(SwapStopThreshold::SwapsAccepted(0)),
                };
                let swap_ratios = vec![TestSwapRatio(0, 0)];
                let (run_swap_input, validated_permutation) =
                    super::super::make_test_data(parameters.clone());
                let dispatcher = Box::new(SwapDispatcher::new(run_swap_input.clone(), swap_ratios));
                assert_eq!(
                    run_swap(
                        dispatcher,
                        run_swap_input.candidate_permutation,
                        run_swap_input.displacement_goal,
                        &run_swap_input.parameters
                    )?,
                    validated_permutation
                );
                Ok(())
            }

            #[test]
            fn some_accepted() -> Result<(), Box<dyn Error>> {
                let parameters = SwapParametersConfig {
                    stop: SwapStopConfig::Unbounded(SwapStopThreshold::SwapsAccepted(1)),
                };
                let swap_ratios = vec![TestSwapRatio(2, 1)];
                let (run_swap_input, validated_permutation) =
                    super::super::make_test_data(parameters.clone());
                let dispatcher = Box::new(SwapDispatcher::new(run_swap_input.clone(), swap_ratios));
                assert_eq!(
                    run_swap(
                        dispatcher,
                        run_swap_input.candidate_permutation,
                        run_swap_input.displacement_goal,
                        &run_swap_input.parameters
                    )?,
                    validated_permutation
                );
                Ok(())
            }

            #[test]
            fn some_accepted_fraction() -> Result<(), Box<dyn Error>> {
                let parameters = SwapParametersConfig {
                    stop: SwapStopConfig::Unbounded(SwapStopThreshold::SwapAcceptanceFraction(
                        NonnegativeProperFraction::new(0.5)?,
                    )),
                };
                let swap_ratios = vec![TestSwapRatio(2, 1)];
                let (run_swap_input, validated_permutation) =
                    super::super::make_test_data(parameters.clone());
                let dispatcher = Box::new(SwapDispatcher::new(run_swap_input.clone(), swap_ratios));
                assert_eq!(
                    run_swap(
                        dispatcher,
                        run_swap_input.candidate_permutation,
                        run_swap_input.displacement_goal,
                        &run_swap_input.parameters
                    )?,
                    validated_permutation
                );
                Ok(())
            }

            #[test]
            fn all_accepted() -> Result<(), Box<dyn Error>> {
                let parameters = SwapParametersConfig {
                    stop: SwapStopConfig::Unbounded(SwapStopThreshold::SwapsAccepted(2)),
                };
                let swap_ratios = vec![TestSwapRatio(2, 2)];
                let (run_swap_input, validated_permutation) =
                    super::super::make_test_data(parameters.clone());
                let dispatcher = Box::new(SwapDispatcher::new(run_swap_input.clone(), swap_ratios));
                assert_eq!(
                    run_swap(
                        dispatcher,
                        run_swap_input.candidate_permutation,
                        run_swap_input.displacement_goal,
                        &run_swap_input.parameters
                    )?,
                    validated_permutation
                );
                Ok(())
            }
        }
    }

    mod two_swap_rounds {
        mod bounded {
            use super::super::super::super::run_swap;
            use super::super::{SwapDispatcher, TestSwapRatio};
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
                };
                let swap_ratios = Vec::new();
                let (run_swap_input, validated_permutation) =
                    super::super::make_test_data(parameters.clone());
                let dispatcher = Box::new(SwapDispatcher::new(run_swap_input.clone(), swap_ratios));
                assert_eq!(
                    run_swap(
                        dispatcher,
                        run_swap_input.candidate_permutation,
                        run_swap_input.displacement_goal,
                        &run_swap_input.parameters
                    )?,
                    validated_permutation
                );
                Ok(())
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
                };
                let swap_ratios = vec![TestSwapRatio(1, 1); 2];
                let (run_swap_input, validated_permutation) =
                    super::super::make_test_data(parameters.clone());
                let dispatcher = Box::new(SwapDispatcher::new(run_swap_input.clone(), swap_ratios));
                assert_eq!(
                    run_swap(
                        dispatcher,
                        run_swap_input.candidate_permutation,
                        run_swap_input.displacement_goal,
                        &run_swap_input.parameters
                    )?,
                    validated_permutation
                );
                Ok(())
            }

            mod threshold_first {
                use super::super::super::super::super::run_swap;
                use super::super::super::{SwapDispatcher, TestSwapRatio};
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
                    };
                    let swap_ratios = vec![TestSwapRatio(2, 1), TestSwapRatio(2, 0)];
                    let (run_swap_input, validated_permutation) =
                        super::super::super::make_test_data(parameters.clone());
                    let dispatcher =
                        Box::new(SwapDispatcher::new(run_swap_input.clone(), swap_ratios));
                    assert_eq!(
                        run_swap(
                            dispatcher,
                            run_swap_input.candidate_permutation,
                            run_swap_input.displacement_goal,
                            &run_swap_input.parameters
                        )?,
                        validated_permutation
                    );
                    Ok(())
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
                    };
                    let swap_ratios = vec![TestSwapRatio(2, 2), TestSwapRatio(2, 1)];
                    let (run_swap_input, validated_permutation) =
                        super::super::super::make_test_data(parameters.clone());
                    let dispatcher =
                        Box::new(SwapDispatcher::new(run_swap_input.clone(), swap_ratios));
                    assert_eq!(
                        run_swap(
                            dispatcher,
                            run_swap_input.candidate_permutation,
                            run_swap_input.displacement_goal,
                            &run_swap_input.parameters
                        )?,
                        validated_permutation
                    );
                    Ok(())
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
                    };
                    let swap_ratios = vec![TestSwapRatio(2, 2), TestSwapRatio(2, 1)];
                    let (run_swap_input, validated_permutation) =
                        super::super::super::make_test_data(parameters.clone());
                    let dispatcher =
                        Box::new(SwapDispatcher::new(run_swap_input.clone(), swap_ratios));
                    assert_eq!(
                        run_swap(
                            dispatcher,
                            run_swap_input.candidate_permutation,
                            run_swap_input.displacement_goal,
                            &run_swap_input.parameters
                        )?,
                        validated_permutation
                    );
                    Ok(())
                }
            }
        }

        mod unbounded {
            use super::super::super::super::run_swap;
            use super::super::{SwapDispatcher, TestSwapRatio};
            use crate::config::{
                NonnegativeProperFraction, SwapParametersConfig, SwapStopConfig, SwapStopThreshold,
            };
            use std::error::Error;

            #[test]
            fn none_accepted() -> Result<(), Box<dyn Error>> {
                let parameters = SwapParametersConfig {
                    stop: SwapStopConfig::Unbounded(SwapStopThreshold::SwapsAccepted(0)),
                };
                let swap_ratios = vec![TestSwapRatio(2, 1), TestSwapRatio(2, 0)];
                let (run_swap_input, validated_permutation) =
                    super::super::make_test_data(parameters.clone());
                let dispatcher = Box::new(SwapDispatcher::new(run_swap_input.clone(), swap_ratios));
                assert_eq!(
                    run_swap(
                        dispatcher,
                        run_swap_input.candidate_permutation,
                        run_swap_input.displacement_goal,
                        &run_swap_input.parameters
                    )?,
                    validated_permutation
                );
                Ok(())
            }

            #[test]
            fn some_accepted() -> Result<(), Box<dyn Error>> {
                let parameters = SwapParametersConfig {
                    stop: SwapStopConfig::Unbounded(SwapStopThreshold::SwapsAccepted(1)),
                };
                let swap_ratios = vec![TestSwapRatio(2, 2), TestSwapRatio(2, 1)];
                let (run_swap_input, validated_permutation) =
                    super::super::make_test_data(parameters.clone());
                let dispatcher = Box::new(SwapDispatcher::new(run_swap_input.clone(), swap_ratios));
                assert_eq!(
                    run_swap(
                        dispatcher,
                        run_swap_input.candidate_permutation,
                        run_swap_input.displacement_goal,
                        &run_swap_input.parameters
                    )?,
                    validated_permutation
                );
                Ok(())
            }

            #[test]
            fn some_accepted_fraction() -> Result<(), Box<dyn Error>> {
                let parameters = SwapParametersConfig {
                    stop: SwapStopConfig::Unbounded(SwapStopThreshold::SwapAcceptanceFraction(
                        NonnegativeProperFraction::new(0.5)?,
                    )),
                };
                let swap_ratios = vec![TestSwapRatio(2, 2), TestSwapRatio(2, 1)];
                let (run_swap_input, validated_permutation) =
                    super::super::make_test_data(parameters.clone());
                let dispatcher = Box::new(SwapDispatcher::new(run_swap_input.clone(), swap_ratios));
                assert_eq!(
                    run_swap(
                        dispatcher,
                        run_swap_input.candidate_permutation,
                        run_swap_input.displacement_goal,
                        &run_swap_input.parameters
                    )?,
                    validated_permutation
                );
                Ok(())
            }
        }
    }

    mod three_swap_rounds {
        mod bounded {
            use super::super::super::super::run_swap;
            use super::super::{SwapDispatcher, TestSwapRatio};
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
                };
                let swap_ratios = Vec::new();
                let (run_swap_input, validated_permutation) =
                    super::super::make_test_data(parameters.clone());
                let dispatcher = Box::new(SwapDispatcher::new(run_swap_input.clone(), swap_ratios));
                assert_eq!(
                    run_swap(
                        dispatcher,
                        run_swap_input.candidate_permutation,
                        run_swap_input.displacement_goal,
                        &run_swap_input.parameters
                    )?,
                    validated_permutation
                );
                Ok(())
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
                };
                let swap_ratios = vec![TestSwapRatio(1, 1); 3];
                let (run_swap_input, validated_permutation) =
                    super::super::make_test_data(parameters.clone());
                let dispatcher = Box::new(SwapDispatcher::new(run_swap_input.clone(), swap_ratios));
                assert_eq!(
                    run_swap(
                        dispatcher,
                        run_swap_input.candidate_permutation,
                        run_swap_input.displacement_goal,
                        &run_swap_input.parameters
                    )?,
                    validated_permutation
                );
                Ok(())
            }

            mod threshold_first {
                use super::super::super::super::super::run_swap;
                use super::super::super::{SwapDispatcher, TestSwapRatio};
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
                    };
                    let swap_ratios = vec![
                        TestSwapRatio(2, 1),
                        TestSwapRatio(2, 1),
                        TestSwapRatio(2, 0),
                    ];
                    let (run_swap_input, validated_permutation) =
                        super::super::super::make_test_data(parameters.clone());
                    let dispatcher =
                        Box::new(SwapDispatcher::new(run_swap_input.clone(), swap_ratios));
                    assert_eq!(
                        run_swap(
                            dispatcher,
                            run_swap_input.candidate_permutation,
                            run_swap_input.displacement_goal,
                            &run_swap_input.parameters
                        )?,
                        validated_permutation
                    );
                    Ok(())
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
                    };
                    let swap_ratios = vec![
                        TestSwapRatio(2, 2),
                        TestSwapRatio(2, 2),
                        TestSwapRatio(2, 1),
                    ];
                    let (run_swap_input, validated_permutation) =
                        super::super::super::make_test_data(parameters.clone());
                    let dispatcher =
                        Box::new(SwapDispatcher::new(run_swap_input.clone(), swap_ratios));
                    assert_eq!(
                        run_swap(
                            dispatcher,
                            run_swap_input.candidate_permutation,
                            run_swap_input.displacement_goal,
                            &run_swap_input.parameters
                        )?,
                        validated_permutation
                    );
                    Ok(())
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
                    };
                    let swap_ratios = vec![
                        TestSwapRatio(2, 2),
                        TestSwapRatio(2, 2),
                        TestSwapRatio(2, 1),
                    ];
                    let (run_swap_input, validated_permutation) =
                        super::super::super::make_test_data(parameters.clone());
                    let dispatcher =
                        Box::new(SwapDispatcher::new(run_swap_input.clone(), swap_ratios));
                    assert_eq!(
                        run_swap(
                            dispatcher,
                            run_swap_input.candidate_permutation,
                            run_swap_input.displacement_goal,
                            &run_swap_input.parameters
                        )?,
                        validated_permutation
                    );
                    Ok(())
                }
            }
        }

        mod unbounded {
            use super::super::super::super::run_swap;
            use super::super::{SwapDispatcher, TestSwapRatio};
            use crate::config::{
                NonnegativeProperFraction, SwapParametersConfig, SwapStopConfig, SwapStopThreshold,
            };
            use std::error::Error;

            #[test]
            fn none_accepted() -> Result<(), Box<dyn Error>> {
                let parameters = SwapParametersConfig {
                    stop: SwapStopConfig::Unbounded(SwapStopThreshold::SwapsAccepted(0)),
                };
                let swap_ratios = vec![
                    TestSwapRatio(2, 1),
                    TestSwapRatio(2, 1),
                    TestSwapRatio(2, 0),
                ];
                let (run_swap_input, validated_permutation) =
                    super::super::make_test_data(parameters.clone());
                let dispatcher = Box::new(SwapDispatcher::new(run_swap_input.clone(), swap_ratios));
                assert_eq!(
                    run_swap(
                        dispatcher,
                        run_swap_input.candidate_permutation,
                        run_swap_input.displacement_goal,
                        &run_swap_input.parameters
                    )?,
                    validated_permutation
                );
                Ok(())
            }

            #[test]
            fn some_accepted() -> Result<(), Box<dyn Error>> {
                let parameters = SwapParametersConfig {
                    stop: SwapStopConfig::Unbounded(SwapStopThreshold::SwapsAccepted(1)),
                };
                let swap_ratios = vec![
                    TestSwapRatio(2, 2),
                    TestSwapRatio(2, 2),
                    TestSwapRatio(2, 1),
                ];
                let (run_swap_input, validated_permutation) =
                    super::super::make_test_data(parameters.clone());
                let dispatcher = Box::new(SwapDispatcher::new(run_swap_input.clone(), swap_ratios));
                assert_eq!(
                    run_swap(
                        dispatcher,
                        run_swap_input.candidate_permutation,
                        run_swap_input.displacement_goal,
                        &run_swap_input.parameters
                    )?,
                    validated_permutation
                );
                Ok(())
            }

            #[test]
            fn some_accepted_fraction() -> Result<(), Box<dyn Error>> {
                let parameters = SwapParametersConfig {
                    stop: SwapStopConfig::Unbounded(SwapStopThreshold::SwapAcceptanceFraction(
                        NonnegativeProperFraction::new(0.5)?,
                    )),
                };
                let swap_ratios = vec![
                    TestSwapRatio(2, 2),
                    TestSwapRatio(2, 2),
                    TestSwapRatio(2, 1),
                ];
                let (run_swap_input, validated_permutation) =
                    super::super::make_test_data(parameters.clone());
                let dispatcher = Box::new(SwapDispatcher::new(run_swap_input.clone(), swap_ratios));
                assert_eq!(
                    run_swap(
                        dispatcher,
                        run_swap_input.candidate_permutation,
                        run_swap_input.displacement_goal,
                        &run_swap_input.parameters
                    )?,
                    validated_permutation
                );
                Ok(())
            }
        }
    }
}
