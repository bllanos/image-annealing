mod create_displacement_goal_input_config {

    mod from_unverified_config {
        use super::super::super::{
            CreateDisplacementGoalInputConfig, InputDisplacementGoalPath, InputLosslessImagePath,
            InputPermutationPath, UnverifiedCreateDisplacementGoalInputConfig,
            UnverifiedCreateDisplacementGoalInputDataConfig, UnverifiedImageDimensionsConfig,
            UnverifiedInputDisplacementGoalPath, UnverifiedInputLosslessImagePath,
            UnverifiedInputPermutationPath,
        };
        use image_annealing::ImageDimensions;
        use std::error::Error;

        #[test]
        fn valid_dimensions() -> Result<(), Box<dyn Error>> {
            let image_dimensions = ImageDimensions::try_new(20, 25)?;
            let unverified_config = UnverifiedCreateDisplacementGoalInputConfig::ImageDimensions(
                UnverifiedImageDimensionsConfig {
                    width: image_dimensions.width(),
                    height: image_dimensions.height(),
                },
            );
            let expected_config: CreateDisplacementGoalInputConfig = Default::default();
            assert_eq!(
                CreateDisplacementGoalInputConfig::try_from_unverified_with_path_context(
                    unverified_config,
                    test_util::path::base_input().0
                )?,
                (expected_config, image_dimensions)
            );
            Ok(())
        }

        #[test]
        fn valid_input() -> Result<(), Box<dyn Error>> {
            let unverified_displacement_goal_path =
                UnverifiedInputDisplacementGoalPath(test_util::path::relative_input_file(
                    "image/displacement_goal/identity_displacement_goal.png",
                ));
            let unverified_candidate_permutation_path = UnverifiedInputPermutationPath(
                test_util::path::relative_input_file("image/permutation/identity_permutation.png"),
            );
            let unverified_lossless_image_path = UnverifiedInputLosslessImagePath::Rgba8(
                test_util::path::relative_input_file("image/image/stripes.png"),
            );
            let unverified_config = UnverifiedCreateDisplacementGoalInputConfig::Input(
                UnverifiedCreateDisplacementGoalInputDataConfig {
                    displacement_goal: Some(unverified_displacement_goal_path.clone()),
                    candidate_permutation: Some(unverified_candidate_permutation_path.clone()),
                    image: Some(unverified_lossless_image_path.clone()),
                },
            );
            let (displacement_goal_path, image_dimensions) =
                InputDisplacementGoalPath::try_from_unverified_with_path_context(
                    unverified_displacement_goal_path,
                    test_util::path::base_input().0,
                )?;
            let (candidate_permutation_path, _) =
                InputPermutationPath::try_from_unverified_with_path_context(
                    unverified_candidate_permutation_path,
                    test_util::path::base_input().0,
                )?;
            let (image_path, _) = InputLosslessImagePath::try_from_unverified_with_path_context(
                unverified_lossless_image_path,
                test_util::path::base_input().0,
            )?;
            let expected_config = CreateDisplacementGoalInputConfig {
                displacement_goal: Some(displacement_goal_path),
                candidate_permutation: Some(candidate_permutation_path),
                image: Some(image_path),
            };
            assert_eq!(
                CreateDisplacementGoalInputConfig::try_from_unverified_with_path_context(
                    unverified_config,
                    test_util::path::base_input().0,
                )?,
                (expected_config, image_dimensions)
            );
            Ok(())
        }

        #[test]
        fn invalid_dimensions() {
            let unverified_config = UnverifiedCreateDisplacementGoalInputConfig::ImageDimensions(
                UnverifiedImageDimensionsConfig {
                    width: 0,
                    height: 25,
                },
            );
            test_util::assert_error_contains(
                CreateDisplacementGoalInputConfig::try_from_unverified_with_path_context(
                    unverified_config,
                    test_util::path::base_input().0,
                ),
                "width is zero",
            );
        }

        #[test]
        fn no_input() {
            let unverified_config =
                UnverifiedCreateDisplacementGoalInputConfig::Input(Default::default());
            test_util::assert_error_contains(
                CreateDisplacementGoalInputConfig::try_from_unverified_with_path_context(
                    unverified_config,
                    test_util::path::base_input().0,),
                "at least one input must be provided when specifying input data as opposed to image dimensions",
            );
        }

        #[test]
        fn invalid_displacement_goal() {
            let unverified_config = UnverifiedCreateDisplacementGoalInputConfig::Input(
                UnverifiedCreateDisplacementGoalInputDataConfig {
                    displacement_goal: Some(UnverifiedInputDisplacementGoalPath(
                        test_util::path::relative_input_file(
                            "image/displacement_goal/not_found.png",
                        ),
                    )),
                    ..Default::default()
                },
            );
            test_util::assert_error_contains(
                CreateDisplacementGoalInputConfig::try_from_unverified_with_path_context(
                    unverified_config,
                    test_util::path::base_input().0,
                ),
                "does not exist", // Note: do not put a platform-dependent path string here
            );
        }

        #[test]
        fn invalid_permutation() {
            let unverified_config = UnverifiedCreateDisplacementGoalInputConfig::Input(
                UnverifiedCreateDisplacementGoalInputDataConfig {
                    candidate_permutation: Some(UnverifiedInputPermutationPath(
                        test_util::path::relative_input_file("image/permutation/not_found.png"),
                    )),
                    ..Default::default()
                },
            );
            test_util::assert_error_contains(
                CreateDisplacementGoalInputConfig::try_from_unverified_with_path_context(
                    unverified_config,
                    test_util::path::base_input().0,
                ),
                "does not exist", // Note: do not put a platform-dependent path string here
            );
        }

        #[test]
        fn invalid_image() {
            let unverified_config = UnverifiedCreateDisplacementGoalInputConfig::Input(
                UnverifiedCreateDisplacementGoalInputDataConfig {
                    image: Some(UnverifiedInputLosslessImagePath::Rgba8(
                        test_util::path::relative_input_file("image/image/not_found.png"),
                    )),
                    ..Default::default()
                },
            );
            test_util::assert_error_contains(
                CreateDisplacementGoalInputConfig::try_from_unverified_with_path_context(
                    unverified_config,
                    test_util::path::base_input().0,
                ),
                "does not exist", // Note: do not put a platform-dependent path string here
            );
        }

        #[test]
        fn dimensions_mismatch() {
            let unverified_config = UnverifiedCreateDisplacementGoalInputConfig::Input(
                UnverifiedCreateDisplacementGoalInputDataConfig {
                    displacement_goal: Some(UnverifiedInputDisplacementGoalPath(
                        test_util::path::relative_input_file(
                            "image/displacement_goal/identity_larger_displacement_goal.png",
                        ),
                    )),
                    candidate_permutation: Some(UnverifiedInputPermutationPath(
                        test_util::path::relative_input_file(
                            "image/permutation/identity_permutation.png",
                        ),
                    )),
                    ..Default::default()
                },
            );
            test_util::assert_error_contains(
                CreateDisplacementGoalInputConfig::try_from_unverified_with_path_context(
                    unverified_config,
                    test_util::path::base_input().0,),
            "mismatch in image dimensions, (width, height) = (21, 25) and (width, height) = (20, 25)",
        );
        }
    }
}
