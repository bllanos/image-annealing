mod config_try_from_unverified_config {
    mod create_displacement_goal {
        use super::super::super::{
            AlgorithmConfig, Config, UnverifiedConfig, UnverifiedCreateDisplacementGoalInputConfig,
            UnverifiedImageDimensionsConfig, UnverifiedOutputDisplacementGoalPath,
        };
        use image_annealing::{compute, ImageDimensions};
        use image_annealing_cli_util::path::{TryFromWithPathContext, TryIntoWithPathContext};
        use std::error::Error;

        #[test]
        fn valid() -> Result<(), Box<dyn Error>> {
            let base_path = test_util::path::base_output().0;
            let unverified_path =
                UnverifiedOutputDisplacementGoalPath(test_util::unique_relative_output_file!());
            let image_dimensions = ImageDimensions::try_new(20, 25)?;
            let unverified_config = UnverifiedConfig::CreateDisplacementGoal {
                input: UnverifiedCreateDisplacementGoalInputConfig::ImageDimensions(
                    UnverifiedImageDimensionsConfig {
                        width: image_dimensions.width(),
                        height: image_dimensions.height(),
                    },
                ),
                displacement_goal_output_path_no_extension: unverified_path.clone(),
            };
            let r: Config = unverified_config.try_into_with_path_context(&base_path)?;
            assert_eq!(
                r,
                Config {
                    algorithm: AlgorithmConfig::CreateDisplacementGoal {
                        input: Default::default(),
                        displacement_goal_output_path_no_extension: unverified_path
                            .try_into_with_path_context(&base_path)?,
                    },
                    dispatcher: compute::Config { image_dimensions }
                }
            );
            Ok(())
        }

        #[test]
        fn invalid_dimensions() {
            let base_path = test_util::path::base_output().0;
            let unverified_config = UnverifiedConfig::CreateDisplacementGoal {
                input: UnverifiedCreateDisplacementGoalInputConfig::ImageDimensions(
                    UnverifiedImageDimensionsConfig {
                        width: 0,
                        height: 25,
                    },
                ),
                displacement_goal_output_path_no_extension: UnverifiedOutputDisplacementGoalPath(
                    test_util::unique_relative_output_file!(),
                ),
            };
            let r = Config::try_from_with_path_context(unverified_config, &base_path);
            test_util::assert_error_contains(r, "width is zero");
        }
    }

    mod create_permutation {
        use super::super::super::{
            AlgorithmConfig, Config, UnverifiedConfig, UnverifiedImageDimensionsConfig,
            UnverifiedOutputPermutationPath,
        };
        use image_annealing::{compute, ImageDimensions};
        use image_annealing_cli_util::path::{TryFromWithPathContext, TryIntoWithPathContext};
        use std::error::Error;

        #[test]
        fn valid() -> Result<(), Box<dyn Error>> {
            let base_path = test_util::path::base_output().0;
            let unverified_path =
                UnverifiedOutputPermutationPath(test_util::unique_relative_output_file!());
            let unverified_config = UnverifiedConfig::CreatePermutation {
                image_dimensions: UnverifiedImageDimensionsConfig {
                    width: 20,
                    height: 25,
                },
                permutation_output_path_no_extension: unverified_path.clone(),
            };
            let r: Config = unverified_config.try_into_with_path_context(&base_path)?;
            assert_eq!(
                r,
                Config {
                    algorithm: AlgorithmConfig::CreatePermutation {
                        permutation_output_path_no_extension: unverified_path
                            .try_into_with_path_context(&base_path)?,
                    },
                    dispatcher: compute::Config {
                        image_dimensions: ImageDimensions::try_new(20, 25)?
                    }
                }
            );
            Ok(())
        }

        #[test]
        fn invalid_dimensions() {
            let base_path = test_util::path::base_output().0;
            let unverified_config = UnverifiedConfig::CreatePermutation {
                image_dimensions: UnverifiedImageDimensionsConfig {
                    width: 0,
                    height: 25,
                },
                permutation_output_path_no_extension: UnverifiedOutputPermutationPath(
                    test_util::unique_relative_output_file!(),
                ),
            };
            let r = Config::try_from_with_path_context(unverified_config, &base_path);
            test_util::assert_error_contains(r, "width is zero");
        }
    }

    mod permute {
        use super::super::super::{
            AlgorithmConfig, Config, InputLosslessImagePath, InputPermutationPath,
            UnverifiedConfig, UnverifiedInputLosslessImagePath, UnverifiedInputPermutationPath,
            UnverifiedOutputLosslessImagePath,
        };
        use image_annealing::compute;
        use image_annealing_cli_util::path::{TryFromWithPathContext, TryIntoWithPathContext};
        use std::error::Error;

        #[test]
        fn valid() -> Result<(), Box<dyn Error>> {
            let base_path = test_util::path::base_output().0;
            let unverified_candidate_permutation_path = UnverifiedInputPermutationPath(
                test_util::path::relative_input_file("image/permutation/identity_permutation.png"),
            );
            let unverified_original_image_path = UnverifiedInputLosslessImagePath::Rgba8(
                test_util::path::relative_input_file("image/image/stripes.png"),
            );
            let unverified_permuted_image_path =
                UnverifiedOutputLosslessImagePath::Rgba8(test_util::unique_relative_output_file!());
            let unverified_config = UnverifiedConfig::Permute {
                candidate_permutation: unverified_candidate_permutation_path.clone(),
                original_image: unverified_original_image_path.clone(),
                permuted_image_output_path_no_extension: unverified_permuted_image_path.clone(),
            };
            let r: Config = unverified_config.try_into_with_path_context(&base_path)?;
            let (candidate_permutation_path, image_dimensions) =
                InputPermutationPath::try_from_unverified_with_path_context(
                    unverified_candidate_permutation_path,
                    &base_path,
                )?;
            assert_eq!(
                r,
                Config {
                    algorithm: AlgorithmConfig::Permute {
                        candidate_permutation: candidate_permutation_path,
                        original_image:
                            InputLosslessImagePath::try_from_unverified_with_path_context(
                                unverified_original_image_path,
                                &base_path
                            )?
                            .0,
                        permuted_image_output_path_no_extension: unverified_permuted_image_path
                            .try_into_with_path_context(&base_path)?,
                    },
                    dispatcher: compute::Config { image_dimensions }
                }
            );
            Ok(())
        }

        #[test]
        fn invalid_permutation() {
            let base_path = test_util::path::base_output().0;
            let unverified_config = UnverifiedConfig::Permute {
                candidate_permutation: UnverifiedInputPermutationPath(
                    test_util::path::relative_input_file("image/permutation/not_found.png"),
                ),
                original_image: UnverifiedInputLosslessImagePath::Rgba8(
                    test_util::path::relative_input_file("image/image/stripes.png"),
                ),
                permuted_image_output_path_no_extension: UnverifiedOutputLosslessImagePath::Rgba8(
                    test_util::unique_relative_output_file!(),
                ),
            };
            let r = Config::try_from_with_path_context(unverified_config, &base_path);
            test_util::assert_error_contains(
                r,
                "does not exist", // Note: do not put a platform-dependent path string here
            );
        }

        #[test]
        fn invalid_image() {
            let base_path = test_util::path::base_output().0;
            let unverified_config = UnverifiedConfig::Permute {
                candidate_permutation: UnverifiedInputPermutationPath(
                    test_util::path::relative_input_file(
                        "image/permutation/identity_permutation.png",
                    ),
                ),
                original_image: UnverifiedInputLosslessImagePath::Rgba8(
                    test_util::path::relative_input_file("image/image/not_found.png"),
                ),
                permuted_image_output_path_no_extension: UnverifiedOutputLosslessImagePath::Rgba8(
                    test_util::unique_relative_output_file!(),
                ),
            };
            let r = Config::try_from_with_path_context(unverified_config, &base_path);
            test_util::assert_error_contains(
                r,
                "does not exist", // Note: do not put a platform-dependent path string here
            );
        }

        #[test]
        fn invalid_dimensions() {
            let base_path = test_util::path::base_output().0;
            let unverified_config = UnverifiedConfig::Permute {
                candidate_permutation: UnverifiedInputPermutationPath(
                    test_util::path::relative_input_file(
                        "image/permutation/identity_permutation.png",
                    ),
                ),
                original_image: UnverifiedInputLosslessImagePath::Rgba8(
                    test_util::path::relative_input_file("image/image/stripes_large.png"),
                ),
                permuted_image_output_path_no_extension: UnverifiedOutputLosslessImagePath::Rgba8(
                    test_util::unique_relative_output_file!(),
                ),
            };
            let r = Config::try_from_with_path_context(unverified_config, &base_path);
            test_util::assert_error_contains(
            r,
            "mismatch in image dimensions, (width, height) = (21, 25) and (width, height) = (20, 25)",
        );
        }
    }

    mod swap {
        use super::super::super::{
            AlgorithmConfig, Config, InputDisplacementGoalPath, InputPermutationPath,
            SwapParametersConfig, SwapPass, SwapStopConfig, SwapStopThreshold, UnverifiedConfig,
            UnverifiedInputDisplacementGoalPath, UnverifiedInputPermutationPath,
            UnverifiedOutputPermutationPath, UnverifiedSwapParametersConfig,
            UnverifiedSwapStopConfig, UnverifiedSwapStopThreshold,
        };
        use image_annealing::compute::{self, SwapPassSequence};
        use image_annealing_cli_util::path::{TryFromWithPathContext, TryIntoWithPathContext};
        use std::error::Error;

        const SWAP_ACCEPTANCE_THRESHOLD: f32 = 2.0;

        fn make_unverified_swap_parameters() -> UnverifiedSwapParametersConfig {
            UnverifiedSwapParametersConfig {
                stop: UnverifiedSwapStopConfig::Unbounded(
                    UnverifiedSwapStopThreshold::SwapsAccepted(1),
                ),
                swap_acceptance_threshold: SWAP_ACCEPTANCE_THRESHOLD,
                swap_pass_sequence: vec![SwapPass::Vertical, SwapPass::OffsetVertical],
                output_intermediate_permutations: false,
            }
        }

        fn make_swap_parameters() -> SwapParametersConfig {
            SwapParametersConfig {
                stop: SwapStopConfig::Unbounded(SwapStopThreshold::SwapsAccepted(1)),
                swap_acceptance_threshold: SWAP_ACCEPTANCE_THRESHOLD,
                swap_pass_sequence: SwapPassSequence::from_passes([
                    compute::SwapPass::Vertical,
                    compute::SwapPass::OffsetVertical,
                ])
                .unwrap(),
                output_intermediate_permutations: false,
            }
        }

        #[test]
        fn valid() -> Result<(), Box<dyn Error>> {
            let base_path = test_util::path::base_output().0;
            let unverified_candidate_permutation_path = UnverifiedInputPermutationPath(
                test_util::path::relative_input_file("image/permutation/identity_permutation.png"),
            );
            let unverified_displacement_goal_path =
                UnverifiedInputDisplacementGoalPath(test_util::path::relative_input_file(
                    "image/displacement_goal/identity_displacement_goal.png",
                ));
            let unverified_output_permutation_path =
                UnverifiedOutputPermutationPath(test_util::unique_relative_output_file!());

            let unverified_config = UnverifiedConfig::Swap {
                candidate_permutation: unverified_candidate_permutation_path.clone(),
                displacement_goal: unverified_displacement_goal_path.clone(),
                permutation_output_path_prefix: unverified_output_permutation_path.clone(),
                parameters: make_unverified_swap_parameters(),
            };
            let r: Config = unverified_config.try_into_with_path_context(&base_path)?;
            let (candidate_permutation_path, image_dimensions) =
                InputPermutationPath::try_from_unverified_with_path_context(
                    unverified_candidate_permutation_path,
                    &base_path,
                )?;
            assert_eq!(
                r,
                Config {
                    algorithm: AlgorithmConfig::Swap {
                        candidate_permutation: candidate_permutation_path,
                        displacement_goal:
                            InputDisplacementGoalPath::try_from_unverified_with_path_context(
                                unverified_displacement_goal_path,
                                &base_path
                            )?
                            .0,
                        permutation_output_path_prefix: unverified_output_permutation_path
                            .try_into_with_path_context(&base_path)?,
                        parameters: make_swap_parameters()
                    },
                    dispatcher: compute::Config { image_dimensions }
                }
            );
            Ok(())
        }

        #[test]
        fn invalid_permutation() {
            let base_path = test_util::path::base_output().0;
            let unverified_config = UnverifiedConfig::Swap {
                candidate_permutation: UnverifiedInputPermutationPath(
                    test_util::path::relative_input_file("image/permutation/not_found.png"),
                ),
                displacement_goal: UnverifiedInputDisplacementGoalPath(
                    test_util::path::relative_input_file(
                        "image/displacement_goal/identity_displacement_goal.png",
                    ),
                ),
                permutation_output_path_prefix: UnverifiedOutputPermutationPath(
                    test_util::unique_relative_output_file!(),
                ),
                parameters: make_unverified_swap_parameters(),
            };
            let r = Config::try_from_with_path_context(unverified_config, &base_path);
            test_util::assert_error_contains(
                r,
                "does not exist", // Note: do not put a platform-dependent path string here
            );
        }

        #[test]
        fn invalid_displacement_goal() {
            let base_path = test_util::path::base_output().0;
            let unverified_config = UnverifiedConfig::Swap {
                candidate_permutation: UnverifiedInputPermutationPath(
                    test_util::path::relative_input_file(
                        "image/permutation/identity_permutation.png",
                    ),
                ),
                displacement_goal: UnverifiedInputDisplacementGoalPath(
                    test_util::path::relative_input_file("image/displacement_goal/not_found.png"),
                ),
                permutation_output_path_prefix: UnverifiedOutputPermutationPath(
                    test_util::unique_relative_output_file!(),
                ),
                parameters: make_unverified_swap_parameters(),
            };
            let r = Config::try_from_with_path_context(unverified_config, &base_path);

            test_util::assert_error_contains(
                r,
                "does not exist", // Note: do not put a platform-dependent path string here
            );
        }

        #[test]
        fn invalid_dimensions() {
            let base_path = test_util::path::base_output().0;
            let unverified_config = UnverifiedConfig::Swap {
                candidate_permutation: UnverifiedInputPermutationPath(
                    test_util::path::relative_input_file(
                        "image/permutation/identity_permutation.png",
                    ),
                ),
                displacement_goal: UnverifiedInputDisplacementGoalPath(
                    test_util::path::relative_input_file(
                        "image/displacement_goal/identity_larger_displacement_goal.png",
                    ),
                ),
                permutation_output_path_prefix: UnverifiedOutputPermutationPath(
                    test_util::unique_relative_output_file!(),
                ),
                parameters: make_unverified_swap_parameters(),
            };
            let r = Config::try_from_with_path_context(unverified_config, &base_path);

            test_util::assert_error_contains(
            r,
            "mismatch in image dimensions, (width, height) = (20, 25) and (width, height) = (21, 25)",
        );
        }

        #[test]
        fn invalid_swap_parameters() {
            let base_path = test_util::path::base_output().0;
            let unverified_config = UnverifiedConfig::Swap {
                candidate_permutation: UnverifiedInputPermutationPath(
                    test_util::path::relative_input_file(
                        "image/permutation/identity_permutation.png",
                    ),
                ),
                displacement_goal: UnverifiedInputDisplacementGoalPath(
                    test_util::path::relative_input_file(
                        "image/displacement_goal/identity_displacement_goal.png",
                    ),
                ),
                permutation_output_path_prefix: UnverifiedOutputPermutationPath(
                    test_util::unique_relative_output_file!(),
                ),
                parameters: UnverifiedSwapParametersConfig {
                    stop: UnverifiedSwapStopConfig::Unbounded(
                        UnverifiedSwapStopThreshold::SwapAcceptanceFraction(2.0),
                    ),
                    ..make_unverified_swap_parameters()
                },
            };
            let r = Config::try_from_with_path_context(unverified_config, &base_path);

            test_util::assert_error_contains(r, "2 is not less than one");
        }
    }

    mod validate_permutation {
        use super::super::super::{
            AlgorithmConfig, Config, InputPermutationPath, UnverifiedConfig,
            UnverifiedInputPermutationPath,
        };
        use image_annealing::compute;
        use image_annealing_cli_util::path::{TryFromWithPathContext, TryIntoWithPathContext};
        use std::error::Error;

        #[test]
        fn valid() -> Result<(), Box<dyn Error>> {
            let base_path = test_util::path::base_output().0;
            let unverified_candidate_permutation_path = UnverifiedInputPermutationPath(
                test_util::path::relative_input_file("image/permutation/identity_permutation.png"),
            );
            let unverified_config = UnverifiedConfig::ValidatePermutation {
                candidate_permutation: unverified_candidate_permutation_path.clone(),
            };
            let r: Config = unverified_config.try_into_with_path_context(&base_path)?;
            let (candidate_permutation_path, image_dimensions) =
                InputPermutationPath::try_from_unverified_with_path_context(
                    unverified_candidate_permutation_path,
                    &base_path,
                )?;
            assert_eq!(
                r,
                Config {
                    algorithm: AlgorithmConfig::ValidatePermutation {
                        candidate_permutation: candidate_permutation_path
                    },
                    dispatcher: compute::Config { image_dimensions }
                }
            );
            Ok(())
        }

        #[test]
        fn invalid_permutation() {
            let base_path = test_util::path::base_output().0;
            let unverified_config = UnverifiedConfig::ValidatePermutation {
                candidate_permutation: UnverifiedInputPermutationPath(
                    test_util::path::relative_input_file("image/permutation/not_found.png"),
                ),
            };
            let r = Config::try_from_with_path_context(unverified_config, &base_path);
            test_util::assert_error_contains(
                r,
                "does not exist", // Note: do not put a platform-dependent path string here
            );
        }
    }
}
