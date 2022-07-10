mod run_swap {
    use super::super::iter::TaggedPermutation;
    use crate::config::{SwapParametersConfig, SwapStopConfig, SwapStopThreshold};
    use image_annealing::compute::{
        Algorithm, CreatePermutationAlgorithm, CreatePermutationInput, CreatePermutationParameters,
        Dispatcher, OutputStatus, PermuteAlgorithm, PermuteInput, PermuteParameters, SwapAlgorithm,
        SwapFullOutput, SwapInput, SwapParameters, SwapPartialOutput, SwapPass, SwapPassSequence,
        SwapPassSequenceSwapRatio, SwapPassSwapRatio, SwapRatio, ValidatePermutationAlgorithm,
        ValidatePermutationInput, ValidatePermutationParameters,
    };
    use image_annealing::image_utils::validation;
    use image_annealing::{CandidatePermutation, DisplacementGoal, ValidatedPermutation};
    use std::error::Error;
    use std::fmt;

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

    fn expected_number_of_rounds(
        stop_config: &SwapStopConfig,
        swap_counts: &Vec<TestSwapRatio>,
    ) -> usize {
        match stop_config {
            SwapStopConfig::Bounded {
                iteration_count,
                threshold: threshold_option,
            } => match threshold_option {
                None => iteration_count.get(),
                Some(threshold) => std::cmp::min(
                    swap_counts
                        .iter()
                        .take_while(|swap_count| match threshold {
                            SwapStopThreshold::SwapsAccepted(accepted) => {
                                swap_count.accepted() > *accepted
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
                swap_counts
                    .iter()
                    .take_while(|swap_count| match threshold {
                        SwapStopThreshold::SwapsAccepted(accepted) => {
                            swap_count.accepted() > *accepted
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

    fn expected_number_of_passes(
        parameters: &SwapParametersConfig,
        swap_counts: &Vec<TestSwapRatio>,
    ) -> usize {
        expected_number_of_rounds(&parameters.stop, swap_counts)
            * parameters.swap_pass_sequence.iter().count()
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
    }

    impl SwapDispatcher {
        const FINAL_STEP_INDEX: usize = 3;

        pub fn new(
            run_swap_input: RunSwapInput,
            output_swap_counts: Vec<TestSwapRatio>,
            output_permutations: Vec<ValidatedPermutation>,
        ) -> Self {
            assert_eq!(
                output_permutations.len(),
                if run_swap_input.parameters.output_intermediate_permutations {
                    expected_number_of_passes(&run_swap_input.parameters, &output_swap_counts)
                } else {
                    1
                }
            );
            let instance = Self {
                run_swap_input,
                output_swap_counts,
                output_permutations: output_permutations.into_iter(),
                swap_round_index: 0,
                step_index: 0,
                remaining_passes: None,
                previous_pass: None,
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
            expected_number_of_rounds(
                &self.run_swap_input.parameters.stop,
                &self.output_swap_counts,
            )
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
            self.swap_round_index += 1;
            self.step_index = 0;
            self.previous_pass = None;
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
                    let (pass_option, is_last_pass) = self
                        .remaining_passes
                        .as_mut()
                        .map(|iter| (iter.next(), iter.peek().is_none()))
                        .unwrap();
                    match pass_option {
                        Some(_) => {
                            self.previous_pass = pass_option;
                            if is_last_pass {
                                if self.expected_count_swap_flag() {
                                    OutputStatus::NewFullOutput
                                } else {
                                    self.step_index += 1;
                                    OutputStatus::FinalFullOutput
                                }
                            } else {
                                OutputStatus::NewFullOutput
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
            if self.step_index < 2 {
                self.step_index += 1;
            }
            Ok(status)
        }

        fn partial_output(&mut self) -> Option<SwapPartialOutput> {
            if self.expected_count_swap_flag() && self.step_index == Self::FINAL_STEP_INDEX {
                Some(SwapPartialOutput {
                    counts: Box::new(self.output_swap_counts[self.swap_round_index - 1].clone()),
                })
            } else {
                unreachable!()
            }
        }

        fn full_output(&mut self) -> Option<SwapFullOutput> {
            if !(self
                .run_swap_input
                .parameters
                .output_intermediate_permutations
                || (self.swap_round_index == self.expected_number_of_rounds()
                    && self.step_index == Self::FINAL_STEP_INDEX))
            {
                unreachable!()
            }
            Some(SwapFullOutput {
                input: None,
                output_permutation: self.output_permutations.next().unwrap(),
                pass: self.previous_pass.unwrap(),
            })
        }

        fn return_to_dispatcher(self: Box<Self>) -> Box<dyn Dispatcher> {
            self
        }
    }

    fn test_run_swap_with_parameters(
        stop: SwapStopConfig,
        swap_ratios: Vec<TestSwapRatio>,
    ) -> Result<(), Box<dyn Error>> {
        for swap_pass_sequence in [
            SwapPassSequence::from(SwapPass::Vertical),
            SwapPassSequence::from_passes([SwapPass::OffsetVertical, SwapPass::Horizontal])?,
            SwapPassSequence::all(),
        ] {
            for output_intermediate_permutations in [false, true] {
                let parameters = SwapParametersConfig {
                    stop: stop.clone(),
                    swap_acceptance_threshold: 2.0,
                    swap_pass_sequence,
                    output_intermediate_permutations,
                };

                let number_of_output_permutations = if output_intermediate_permutations {
                    expected_number_of_passes(&parameters, &swap_ratios)
                } else {
                    1
                };
                let width = number_of_output_permutations + 3;
                let validated_permutations = (3..width)
                    .map(|first_pixel_shift| unsafe {
                        validation::vector_field_into_validated_permutation_unchecked(
                            test_utils::permutation::line_with_first_texel_moved(
                                width,
                                first_pixel_shift,
                            )
                            .permutation,
                        )
                    })
                    .collect::<Vec<_>>();
                let run_swap_input = RunSwapInput {
                    candidate_permutation: Some(
                        CandidatePermutation::new(
                            test_utils::permutation::line_with_first_texel_moved(width, 1)
                                .permutation,
                        )
                        .unwrap(),
                    ),
                    displacement_goal: Some(
                        DisplacementGoal::from_raw_candidate_permutation(
                            test_utils::permutation::line_with_first_texel_moved(width, 2)
                                .permutation,
                        )
                        .unwrap(),
                    ),
                    parameters,
                };

                let dispatcher = Box::new(SwapDispatcher::new(
                    run_swap_input.clone(),
                    swap_ratios.clone(),
                    validated_permutations.clone(),
                ));

                let passes_per_round = run_swap_input.parameters.swap_pass_sequence.iter().count();
                assert_eq!(
                    super::super::run_swap(
                        dispatcher,
                        run_swap_input.candidate_permutation,
                        run_swap_input.displacement_goal,
                        &run_swap_input.parameters
                    )
                    .collect::<Result<Vec<_>, _>>()?,
                    validated_permutations
                        .into_iter()
                        .enumerate()
                        .zip(
                            run_swap_input
                                .parameters
                                .swap_pass_sequence
                                .into_iter()
                                .cycle()
                        )
                        .map(|((i, permutation), pass)| {
                            if output_intermediate_permutations {
                                TaggedPermutation {
                                    permutation,
                                    round_index: i / passes_per_round,
                                    pass_index: i % passes_per_round,
                                    pass,
                                }
                            } else {
                                TaggedPermutation {
                                    permutation,
                                    round_index: expected_number_of_rounds(
                                        &run_swap_input.parameters.stop,
                                        &swap_ratios,
                                    ) - 1,
                                    pass_index: passes_per_round - 1,
                                    pass: *run_swap_input
                                        .parameters
                                        .swap_pass_sequence
                                        .iter()
                                        .last()
                                        .unwrap(),
                                }
                            }
                        })
                        .collect::<Vec<_>>()
                );
            }
        }
        Ok(())
    }

    mod one_swap_rounds {
        mod bounded {
            use super::super::TestSwapRatio;
            use crate::config::{IterationCount, SwapStopConfig, SwapStopThreshold};
            use std::error::Error;
            use std::num::NonZeroUsize;

            const ITERATION_COUNT_LIMIT: usize = 1;

            #[test]
            fn iteration_count_only() -> Result<(), Box<dyn Error>> {
                let swap_ratios = Vec::new();
                super::super::test_run_swap_with_parameters(
                    SwapStopConfig::Bounded {
                        iteration_count: IterationCount(
                            NonZeroUsize::new(ITERATION_COUNT_LIMIT).unwrap(),
                        ),
                        threshold: None,
                    },
                    swap_ratios,
                )
            }

            #[test]
            fn iteration_count_first() -> Result<(), Box<dyn Error>> {
                let swap_ratios = vec![TestSwapRatio(1, 1)];
                super::super::test_run_swap_with_parameters(
                    SwapStopConfig::Bounded {
                        iteration_count: IterationCount(
                            NonZeroUsize::new(ITERATION_COUNT_LIMIT).unwrap(),
                        ),
                        threshold: Some(SwapStopThreshold::SwapsAccepted(0)),
                    },
                    swap_ratios,
                )
            }

            mod threshold_first {
                use super::super::super::TestSwapRatio;
                use crate::config::{
                    IterationCount, NonnegativeProperFraction, SwapStopConfig, SwapStopThreshold,
                };
                use std::error::Error;
                use std::num::NonZeroUsize;

                const ITERATION_COUNT_LIMIT_ADD_ONE: usize = 2;

                #[test]
                fn none_accepted() -> Result<(), Box<dyn Error>> {
                    let swap_ratios = vec![TestSwapRatio(0, 0)];
                    super::super::super::test_run_swap_with_parameters(
                        SwapStopConfig::Bounded {
                            iteration_count: IterationCount(
                                NonZeroUsize::new(ITERATION_COUNT_LIMIT_ADD_ONE).unwrap(),
                            ),
                            threshold: Some(SwapStopThreshold::SwapsAccepted(0)),
                        },
                        swap_ratios,
                    )
                }

                #[test]
                fn some_accepted() -> Result<(), Box<dyn Error>> {
                    let swap_ratios = vec![TestSwapRatio(2, 1)];
                    super::super::super::test_run_swap_with_parameters(
                        SwapStopConfig::Bounded {
                            iteration_count: IterationCount(
                                NonZeroUsize::new(ITERATION_COUNT_LIMIT_ADD_ONE).unwrap(),
                            ),
                            threshold: Some(SwapStopThreshold::SwapsAccepted(1)),
                        },
                        swap_ratios,
                    )
                }

                #[test]
                fn some_accepted_fraction() -> Result<(), Box<dyn Error>> {
                    let swap_ratios = vec![TestSwapRatio(2, 1)];
                    super::super::super::test_run_swap_with_parameters(
                        SwapStopConfig::Bounded {
                            iteration_count: IterationCount(
                                NonZeroUsize::new(ITERATION_COUNT_LIMIT_ADD_ONE).unwrap(),
                            ),
                            threshold: Some(SwapStopThreshold::SwapAcceptanceFraction(
                                NonnegativeProperFraction::new(0.5)?,
                            )),
                        },
                        swap_ratios,
                    )
                }

                #[test]
                fn all_accepted() -> Result<(), Box<dyn Error>> {
                    let swap_ratios = vec![TestSwapRatio(2, 2)];
                    super::super::super::test_run_swap_with_parameters(
                        SwapStopConfig::Bounded {
                            iteration_count: IterationCount(
                                NonZeroUsize::new(ITERATION_COUNT_LIMIT_ADD_ONE).unwrap(),
                            ),
                            threshold: Some(SwapStopThreshold::SwapsAccepted(2)),
                        },
                        swap_ratios,
                    )
                }
            }
        }

        mod unbounded {
            use super::super::TestSwapRatio;
            use crate::config::{NonnegativeProperFraction, SwapStopConfig, SwapStopThreshold};
            use std::error::Error;

            #[test]
            fn none_accepted() -> Result<(), Box<dyn Error>> {
                let swap_ratios = vec![TestSwapRatio(0, 0)];
                super::super::test_run_swap_with_parameters(
                    SwapStopConfig::Unbounded(SwapStopThreshold::SwapsAccepted(0)),
                    swap_ratios,
                )
            }

            #[test]
            fn some_accepted() -> Result<(), Box<dyn Error>> {
                let swap_ratios = vec![TestSwapRatio(2, 1)];
                super::super::test_run_swap_with_parameters(
                    SwapStopConfig::Unbounded(SwapStopThreshold::SwapsAccepted(1)),
                    swap_ratios,
                )
            }

            #[test]
            fn some_accepted_fraction() -> Result<(), Box<dyn Error>> {
                let swap_ratios = vec![TestSwapRatio(2, 1)];
                super::super::test_run_swap_with_parameters(
                    SwapStopConfig::Unbounded(SwapStopThreshold::SwapAcceptanceFraction(
                        NonnegativeProperFraction::new(0.5)?,
                    )),
                    swap_ratios,
                )
            }

            #[test]
            fn all_accepted() -> Result<(), Box<dyn Error>> {
                let swap_ratios = vec![TestSwapRatio(2, 2)];
                super::super::test_run_swap_with_parameters(
                    SwapStopConfig::Unbounded(SwapStopThreshold::SwapsAccepted(2)),
                    swap_ratios,
                )
            }
        }
    }

    mod two_swap_rounds {
        mod bounded {
            use super::super::TestSwapRatio;
            use crate::config::{IterationCount, SwapStopConfig, SwapStopThreshold};
            use std::error::Error;
            use std::num::NonZeroUsize;

            const ITERATION_COUNT_LIMIT: usize = 2;

            #[test]
            fn iteration_count_only() -> Result<(), Box<dyn Error>> {
                let swap_ratios = Vec::new();
                super::super::test_run_swap_with_parameters(
                    SwapStopConfig::Bounded {
                        iteration_count: IterationCount(
                            NonZeroUsize::new(ITERATION_COUNT_LIMIT).unwrap(),
                        ),
                        threshold: None,
                    },
                    swap_ratios,
                )
            }

            #[test]
            fn iteration_count_first() -> Result<(), Box<dyn Error>> {
                let swap_ratios = vec![TestSwapRatio(1, 1); 2];
                super::super::test_run_swap_with_parameters(
                    SwapStopConfig::Bounded {
                        iteration_count: IterationCount(
                            NonZeroUsize::new(ITERATION_COUNT_LIMIT).unwrap(),
                        ),
                        threshold: Some(SwapStopThreshold::SwapsAccepted(0)),
                    },
                    swap_ratios,
                )
            }

            mod threshold_first {
                use super::super::super::TestSwapRatio;
                use crate::config::{
                    IterationCount, NonnegativeProperFraction, SwapStopConfig, SwapStopThreshold,
                };
                use std::error::Error;
                use std::num::NonZeroUsize;

                const ITERATION_COUNT_LIMIT_ADD_ONE: usize = 3;

                #[test]
                fn none_accepted() -> Result<(), Box<dyn Error>> {
                    let swap_ratios = vec![TestSwapRatio(2, 1), TestSwapRatio(2, 0)];
                    super::super::super::test_run_swap_with_parameters(
                        SwapStopConfig::Bounded {
                            iteration_count: IterationCount(
                                NonZeroUsize::new(ITERATION_COUNT_LIMIT_ADD_ONE).unwrap(),
                            ),
                            threshold: Some(SwapStopThreshold::SwapsAccepted(0)),
                        },
                        swap_ratios,
                    )
                }

                #[test]
                fn some_accepted() -> Result<(), Box<dyn Error>> {
                    let swap_ratios = vec![TestSwapRatio(2, 2), TestSwapRatio(2, 1)];
                    super::super::super::test_run_swap_with_parameters(
                        SwapStopConfig::Bounded {
                            iteration_count: IterationCount(
                                NonZeroUsize::new(ITERATION_COUNT_LIMIT_ADD_ONE).unwrap(),
                            ),
                            threshold: Some(SwapStopThreshold::SwapsAccepted(1)),
                        },
                        swap_ratios,
                    )
                }

                #[test]
                fn some_accepted_fraction() -> Result<(), Box<dyn Error>> {
                    let swap_ratios = vec![TestSwapRatio(2, 2), TestSwapRatio(2, 1)];
                    super::super::super::test_run_swap_with_parameters(
                        SwapStopConfig::Bounded {
                            iteration_count: IterationCount(
                                NonZeroUsize::new(ITERATION_COUNT_LIMIT_ADD_ONE).unwrap(),
                            ),
                            threshold: Some(SwapStopThreshold::SwapAcceptanceFraction(
                                NonnegativeProperFraction::new(0.5)?,
                            )),
                        },
                        swap_ratios,
                    )
                }
            }
        }

        mod unbounded {
            use super::super::TestSwapRatio;
            use crate::config::{NonnegativeProperFraction, SwapStopConfig, SwapStopThreshold};
            use std::error::Error;

            #[test]
            fn none_accepted() -> Result<(), Box<dyn Error>> {
                let swap_ratios = vec![TestSwapRatio(2, 1), TestSwapRatio(2, 0)];
                super::super::test_run_swap_with_parameters(
                    SwapStopConfig::Unbounded(SwapStopThreshold::SwapsAccepted(0)),
                    swap_ratios,
                )
            }

            #[test]
            fn some_accepted() -> Result<(), Box<dyn Error>> {
                let swap_ratios = vec![TestSwapRatio(2, 2), TestSwapRatio(2, 1)];
                super::super::test_run_swap_with_parameters(
                    SwapStopConfig::Unbounded(SwapStopThreshold::SwapsAccepted(1)),
                    swap_ratios,
                )
            }

            #[test]
            fn some_accepted_fraction() -> Result<(), Box<dyn Error>> {
                let swap_ratios = vec![TestSwapRatio(2, 2), TestSwapRatio(2, 1)];
                super::super::test_run_swap_with_parameters(
                    SwapStopConfig::Unbounded(SwapStopThreshold::SwapAcceptanceFraction(
                        NonnegativeProperFraction::new(0.5)?,
                    )),
                    swap_ratios,
                )
            }
        }
    }

    mod three_swap_rounds {
        mod bounded {
            use super::super::TestSwapRatio;
            use crate::config::{IterationCount, SwapStopConfig, SwapStopThreshold};
            use std::error::Error;
            use std::num::NonZeroUsize;

            const ITERATION_COUNT_LIMIT: usize = 3;

            #[test]
            fn iteration_count_only() -> Result<(), Box<dyn Error>> {
                let swap_ratios = Vec::new();
                super::super::test_run_swap_with_parameters(
                    SwapStopConfig::Bounded {
                        iteration_count: IterationCount(
                            NonZeroUsize::new(ITERATION_COUNT_LIMIT).unwrap(),
                        ),
                        threshold: None,
                    },
                    swap_ratios,
                )
            }

            #[test]
            fn iteration_count_first() -> Result<(), Box<dyn Error>> {
                let swap_ratios = vec![TestSwapRatio(1, 1); 3];
                super::super::test_run_swap_with_parameters(
                    SwapStopConfig::Bounded {
                        iteration_count: IterationCount(
                            NonZeroUsize::new(ITERATION_COUNT_LIMIT).unwrap(),
                        ),
                        threshold: Some(SwapStopThreshold::SwapsAccepted(0)),
                    },
                    swap_ratios,
                )
            }

            mod threshold_first {
                use super::super::super::TestSwapRatio;
                use crate::config::{
                    IterationCount, NonnegativeProperFraction, SwapStopConfig, SwapStopThreshold,
                };
                use std::error::Error;
                use std::num::NonZeroUsize;

                const ITERATION_COUNT_LIMIT_ADD_ONE: usize = 4;

                #[test]
                fn none_accepted() -> Result<(), Box<dyn Error>> {
                    let swap_ratios = vec![
                        TestSwapRatio(2, 1),
                        TestSwapRatio(2, 1),
                        TestSwapRatio(2, 0),
                    ];
                    super::super::super::test_run_swap_with_parameters(
                        SwapStopConfig::Bounded {
                            iteration_count: IterationCount(
                                NonZeroUsize::new(ITERATION_COUNT_LIMIT_ADD_ONE).unwrap(),
                            ),
                            threshold: Some(SwapStopThreshold::SwapsAccepted(0)),
                        },
                        swap_ratios,
                    )
                }

                #[test]
                fn some_accepted() -> Result<(), Box<dyn Error>> {
                    let swap_ratios = vec![
                        TestSwapRatio(2, 2),
                        TestSwapRatio(2, 2),
                        TestSwapRatio(2, 1),
                    ];
                    super::super::super::test_run_swap_with_parameters(
                        SwapStopConfig::Bounded {
                            iteration_count: IterationCount(
                                NonZeroUsize::new(ITERATION_COUNT_LIMIT_ADD_ONE).unwrap(),
                            ),
                            threshold: Some(SwapStopThreshold::SwapsAccepted(1)),
                        },
                        swap_ratios,
                    )
                }

                #[test]
                fn some_accepted_fraction() -> Result<(), Box<dyn Error>> {
                    let swap_ratios = vec![
                        TestSwapRatio(2, 2),
                        TestSwapRatio(2, 2),
                        TestSwapRatio(2, 1),
                    ];
                    super::super::super::test_run_swap_with_parameters(
                        SwapStopConfig::Bounded {
                            iteration_count: IterationCount(
                                NonZeroUsize::new(ITERATION_COUNT_LIMIT_ADD_ONE).unwrap(),
                            ),
                            threshold: Some(SwapStopThreshold::SwapAcceptanceFraction(
                                NonnegativeProperFraction::new(0.5)?,
                            )),
                        },
                        swap_ratios,
                    )
                }
            }
        }

        mod unbounded {
            use super::super::TestSwapRatio;
            use crate::config::{NonnegativeProperFraction, SwapStopConfig, SwapStopThreshold};
            use std::error::Error;

            #[test]
            fn none_accepted() -> Result<(), Box<dyn Error>> {
                let swap_ratios = vec![
                    TestSwapRatio(2, 1),
                    TestSwapRatio(2, 1),
                    TestSwapRatio(2, 0),
                ];
                super::super::test_run_swap_with_parameters(
                    SwapStopConfig::Unbounded(SwapStopThreshold::SwapsAccepted(0)),
                    swap_ratios,
                )
            }

            #[test]
            fn some_accepted() -> Result<(), Box<dyn Error>> {
                let swap_ratios = vec![
                    TestSwapRatio(2, 2),
                    TestSwapRatio(2, 2),
                    TestSwapRatio(2, 1),
                ];
                super::super::test_run_swap_with_parameters(
                    SwapStopConfig::Unbounded(SwapStopThreshold::SwapsAccepted(1)),
                    swap_ratios,
                )
            }

            #[test]
            fn some_accepted_fraction() -> Result<(), Box<dyn Error>> {
                let swap_ratios = vec![
                    TestSwapRatio(2, 2),
                    TestSwapRatio(2, 2),
                    TestSwapRatio(2, 1),
                ];
                super::super::test_run_swap_with_parameters(
                    SwapStopConfig::Unbounded(SwapStopThreshold::SwapAcceptanceFraction(
                        NonnegativeProperFraction::new(0.5)?,
                    )),
                    swap_ratios,
                )
            }
        }
    }
}
