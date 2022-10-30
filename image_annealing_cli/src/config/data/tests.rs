mod config_try_from_unverified_config {
    mod create_permutation {
        use super::super::super::{
            AlgorithmConfig, Config, ImagePath, PermutationPath, UnverifiedConfig,
        };
        use image_annealing::{compute, ImageDimensions};
        use std::error::Error;

        #[test]
        fn valid() -> Result<(), Box<dyn Error>> {
            let unverified_config = UnverifiedConfig::CreatePermutation {
                image_width: 20,
                image_height: 25,
                permutation_output_path_no_extension: String::from("permutation_out"),
            };
            let r: Config = unverified_config.try_into()?;
            assert_eq!(
                r,
                Config {
                    algorithm: AlgorithmConfig::CreatePermutation {
                        permutation_output_path_no_extension: PermutationPath::from_raw_clone(
                            "permutation_out"
                        ),
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
            let unverified_config = UnverifiedConfig::CreatePermutation {
                image_width: 0,
                image_height: 25,
                permutation_output_path_no_extension: String::from("permutation_out"),
            };
            let r = <Config as TryFrom<UnverifiedConfig>>::try_from(unverified_config);
            test_utils::assert_error_contains(r, "width is zero");
        }
    }

    mod permute {
        use super::super::super::{
            AlgorithmConfig, Config, ImagePath, LosslessImagePath, PermutationPath,
            UnverifiedConfig, UnverifiedLosslessImagePath,
        };
        use image_annealing::compute;
        use std::error::Error;

        #[test]
        fn valid() -> Result<(), Box<dyn Error>> {
            let unverified_config = UnverifiedConfig::Permute {
                candidate_permutation: String::from(
                    "../test_data/image/permutation/identity_permutation.png",
                ),
                original_image: UnverifiedLosslessImagePath::Rgba8(String::from(
                    "../test_data/image/image/stripes.png",
                )),
                permuted_image_output_path_no_extension: UnverifiedLosslessImagePath::Rgba8(
                    String::from("permuted_image_out"),
                ),
            };
            let r: Config = unverified_config.try_into()?;
            let (candidate_permutation_path, image_dimensions) =
                PermutationPath::from_input_path(test_utils::make_test_data_path_string([
                    "image",
                    "permutation",
                    "identity_permutation.png",
                ]))?;
            assert_eq!(
                r,
                Config {
                    algorithm: AlgorithmConfig::Permute {
                        candidate_permutation: candidate_permutation_path,
                        original_image: LosslessImagePath::Rgba8(
                            test_utils::make_test_data_path_string([
                                "image",
                                "image",
                                "stripes.png"
                            ])
                        ),
                        permuted_image_output_path_no_extension: LosslessImagePath::Rgba8(
                            String::from("permuted_image_out")
                        ),
                    },
                    dispatcher: compute::Config { image_dimensions }
                }
            );
            Ok(())
        }

        #[test]
        fn invalid_permutation() {
            let unverified_config = UnverifiedConfig::Permute {
                candidate_permutation: String::from("../test_data/image/permutation/not_found.png"),
                original_image: UnverifiedLosslessImagePath::Rgba8(String::from(
                    "../test_data/image/image/stripes.png",
                )),
                permuted_image_output_path_no_extension: UnverifiedLosslessImagePath::Rgba8(
                    String::from("permuted_image_out"),
                ),
            };
            let r = <Config as TryFrom<UnverifiedConfig>>::try_from(unverified_config);
            test_utils::assert_error_contains(
                r,
                "does not exist in the filesystem", // Note: do not put a platform-dependent path string here
            );
        }

        #[test]
        fn invalid_image() {
            let unverified_config = UnverifiedConfig::Permute {
                candidate_permutation: String::from(
                    "../test_data/image/permutation/identity_permutation.png",
                ),
                original_image: UnverifiedLosslessImagePath::Rgba8(String::from(
                    "../test_data/image/image/not_found.png",
                )),
                permuted_image_output_path_no_extension: UnverifiedLosslessImagePath::Rgba8(
                    String::from("permuted_image_out"),
                ),
            };
            let r = <Config as TryFrom<UnverifiedConfig>>::try_from(unverified_config);
            test_utils::assert_error_contains(
                r,
                "does not exist in the filesystem", // Note: do not put a platform-dependent path string here
            );
        }

        #[test]
        fn invalid_dimensions() {
            let unverified_config = UnverifiedConfig::Permute {
                candidate_permutation: String::from(
                    "../test_data/image/permutation/identity_permutation.png",
                ),
                original_image: UnverifiedLosslessImagePath::Rgba8(String::from(
                    "../test_data/image/image/stripes_large.png",
                )),
                permuted_image_output_path_no_extension: UnverifiedLosslessImagePath::Rgba8(
                    String::from("permuted_image_out"),
                ),
            };
            let r = <Config as TryFrom<UnverifiedConfig>>::try_from(unverified_config);
            test_utils::assert_error_contains(
            r,
            "mismatch in image dimensions, (width, height) = (21, 25) and (width, height) = (20, 25)",
        );
        }
    }

    mod swap {
        use super::super::super::{
            AlgorithmConfig, Config, DisplacementGoalPath, ImagePath, PermutationPath,
            SwapParametersConfig, SwapPass, SwapStopConfig, SwapStopThreshold, UnverifiedConfig,
            UnverifiedSwapParametersConfig, UnverifiedSwapStopConfig, UnverifiedSwapStopThreshold,
        };
        use image_annealing::compute::{self, SwapPassSequence};
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
            let unverified_config = UnverifiedConfig::Swap {
                candidate_permutation: String::from(
                    "../test_data/image/permutation/identity_permutation.png",
                ),
                displacement_goal: String::from(
                    "../test_data/image/displacement_goal/identity_displacement_goal.png",
                ),
                permutation_output_path_prefix: String::from("permutation_out"),
                parameters: make_unverified_swap_parameters(),
            };
            let r: Config = unverified_config.try_into()?;
            let (candidate_permutation_path, image_dimensions) =
                PermutationPath::from_input_path(test_utils::make_test_data_path_string([
                    "image",
                    "permutation",
                    "identity_permutation.png",
                ]))?;
            assert_eq!(
                r,
                Config {
                    algorithm: AlgorithmConfig::Swap {
                        candidate_permutation: candidate_permutation_path,
                        displacement_goal: DisplacementGoalPath::from_raw(
                            test_utils::make_test_data_path_string([
                                "image",
                                "displacement_goal",
                                "identity_displacement_goal.png"
                            ])
                        ),
                        permutation_output_path_prefix: PermutationPath::from_raw_clone(
                            "permutation_out"
                        ),
                        parameters: make_swap_parameters()
                    },
                    dispatcher: compute::Config { image_dimensions }
                }
            );
            Ok(())
        }

        #[test]
        fn invalid_permutation() {
            let unverified_config = UnverifiedConfig::Swap {
                candidate_permutation: String::from("../test_data/image/permutation/not_found.png"),
                displacement_goal: String::from(
                    "../test_data/image/displacement_goal/identity_displacement_goal.png",
                ),
                permutation_output_path_prefix: String::from("permutation_out"),
                parameters: make_unverified_swap_parameters(),
            };
            let r = <Config as TryFrom<UnverifiedConfig>>::try_from(unverified_config);
            test_utils::assert_error_contains(
                r,
                "does not exist in the filesystem", // Note: do not put a platform-dependent path string here
            );
        }

        #[test]
        fn invalid_displacement_goal() {
            let unverified_config = UnverifiedConfig::Swap {
                candidate_permutation: String::from(
                    "../test_data/image/permutation/identity_permutation.png",
                ),
                displacement_goal: String::from(
                    "../test_data/image/displacement_goal/not_found.png",
                ),
                permutation_output_path_prefix: String::from("permutation_out"),
                parameters: make_unverified_swap_parameters(),
            };
            let r = <Config as TryFrom<UnverifiedConfig>>::try_from(unverified_config);
            test_utils::assert_error_contains(
                r,
                "does not exist in the filesystem", // Note: do not put a platform-dependent path string here
            );
        }

        #[test]
        fn invalid_dimensions() {
            let unverified_config = UnverifiedConfig::Swap {
                candidate_permutation: String::from(
                    "../test_data/image/permutation/identity_permutation.png",
                ),
                displacement_goal: String::from(
                    "../test_data/image/displacement_goal/identity_larger_displacement_goal.png",
                ),
                permutation_output_path_prefix: String::from("permutation_out"),
                parameters: make_unverified_swap_parameters(),
            };
            let r = <Config as TryFrom<UnverifiedConfig>>::try_from(unverified_config);
            test_utils::assert_error_contains(
            r,
            "mismatch in image dimensions, (width, height) = (20, 25) and (width, height) = (21, 25)",
        );
        }

        #[test]
        fn invalid_swap_parameters() {
            let unverified_config = UnverifiedConfig::Swap {
                candidate_permutation: String::from(
                    "../test_data/image/permutation/identity_permutation.png",
                ),
                displacement_goal: String::from(
                    "../test_data/image/displacement_goal/identity_displacement_goal.png",
                ),
                permutation_output_path_prefix: String::from("permutation_out"),
                parameters: UnverifiedSwapParametersConfig {
                    stop: UnverifiedSwapStopConfig::Unbounded(
                        UnverifiedSwapStopThreshold::SwapAcceptanceFraction(2.0),
                    ),
                    ..make_unverified_swap_parameters()
                },
            };
            let r = <Config as TryFrom<UnverifiedConfig>>::try_from(unverified_config);
            test_utils::assert_error_contains(r, "2 is not less than one");
        }
    }

    mod validate_permutation {
        use super::super::super::{
            AlgorithmConfig, Config, ImagePath, PermutationPath, UnverifiedConfig,
        };
        use image_annealing::compute;
        use std::error::Error;

        #[test]
        fn valid() -> Result<(), Box<dyn Error>> {
            let unverified_config = UnverifiedConfig::ValidatePermutation {
                candidate_permutation: String::from(
                    "../test_data/image/permutation/identity_permutation.png",
                ),
            };
            let r: Config = unverified_config.try_into()?;
            let (candidate_permutation_path, image_dimensions) =
                PermutationPath::from_input_path(test_utils::make_test_data_path_string([
                    "image",
                    "permutation",
                    "identity_permutation.png",
                ]))?;
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
            let unverified_config = UnverifiedConfig::ValidatePermutation {
                candidate_permutation: String::from("../test_data/image/permutation/not_found.png"),
            };
            let r = <Config as TryFrom<UnverifiedConfig>>::try_from(unverified_config);
            test_utils::assert_error_contains(
                r,
                "does not exist in the filesystem", // Note: do not put a platform-dependent path string here
            );
        }
    }
}
