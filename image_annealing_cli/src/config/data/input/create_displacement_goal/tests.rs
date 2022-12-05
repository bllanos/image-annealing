mod create_displacement_goal_input_config {

    mod from_config {
        use super::super::super::{
            CreateDisplacementGoalInputConfig, DisplacementGoalPath, ImagePath, LosslessImagePath,
            PermutationPath, UnverifiedCreateDisplacementGoalInputConfig,
            UnverifiedCreateDisplacementGoalInputDataConfig, UnverifiedImageDimensionsConfig,
            UnverifiedLosslessImagePath,
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
                CreateDisplacementGoalInputConfig::from_config(unverified_config)?,
                (expected_config, image_dimensions)
            );
            Ok(())
        }

        #[test]
        fn valid_input() -> Result<(), Box<dyn Error>> {
            let unverified_config = UnverifiedCreateDisplacementGoalInputConfig::Input(
                UnverifiedCreateDisplacementGoalInputDataConfig {
                    displacement_goal: Some(String::from(
                        "../test_data/image/displacement_goal/identity_displacement_goal.png",
                    )),
                    candidate_permutation: Some(String::from(
                        "../test_data/image/permutation/identity_permutation.png",
                    )),
                    image: Some(UnverifiedLosslessImagePath::Rgba8(String::from(
                        "../test_data/image/image/stripes.png",
                    ))),
                },
            );
            let (displacement_goal_path, image_dimensions) =
                DisplacementGoalPath::from_input_path(test_util::make_test_data_path_string([
                    "image",
                    "displacement_goal",
                    "identity_displacement_goal.png",
                ]))?;
            let (candidate_permutation_path, _) =
                PermutationPath::from_input_path(test_util::make_test_data_path_string([
                    "image",
                    "permutation",
                    "identity_permutation.png",
                ]))?;
            let image_path = LosslessImagePath::Rgba8(test_util::make_test_data_path_string([
                "image",
                "image",
                "stripes.png",
            ]));
            let expected_config = CreateDisplacementGoalInputConfig {
                displacement_goal: Some(displacement_goal_path),
                candidate_permutation: Some(candidate_permutation_path),
                image: Some(image_path),
            };
            assert_eq!(
                CreateDisplacementGoalInputConfig::from_config(unverified_config)?,
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
                CreateDisplacementGoalInputConfig::from_config(unverified_config),
                "width is zero",
            );
        }

        #[test]
        fn no_input() {
            let unverified_config =
                UnverifiedCreateDisplacementGoalInputConfig::Input(Default::default());
            test_util::assert_error_contains(
                CreateDisplacementGoalInputConfig::from_config(unverified_config),
                "at least one input must be provided when specifying input data as opposed to image dimensions",
            );
        }

        #[test]
        fn invalid_displacement_goal() {
            let unverified_config = UnverifiedCreateDisplacementGoalInputConfig::Input(
                UnverifiedCreateDisplacementGoalInputDataConfig {
                    displacement_goal: Some(String::from(
                        "../test_data/image/displacement_goal/not_found.png",
                    )),
                    ..Default::default()
                },
            );
            test_util::assert_error_contains(
                CreateDisplacementGoalInputConfig::from_config(unverified_config),
                "does not exist in the filesystem", // Note: do not put a platform-dependent path string here
            );
        }

        #[test]
        fn invalid_permutation() {
            let unverified_config = UnverifiedCreateDisplacementGoalInputConfig::Input(
                UnverifiedCreateDisplacementGoalInputDataConfig {
                    candidate_permutation: Some(String::from(
                        "../test_data/image/permutation/not_found.png",
                    )),
                    ..Default::default()
                },
            );
            test_util::assert_error_contains(
                CreateDisplacementGoalInputConfig::from_config(unverified_config),
                "does not exist in the filesystem", // Note: do not put a platform-dependent path string here
            );
        }

        #[test]
        fn invalid_image() {
            let unverified_config = UnverifiedCreateDisplacementGoalInputConfig::Input(
                UnverifiedCreateDisplacementGoalInputDataConfig {
                    image: Some(UnverifiedLosslessImagePath::Rgba8(String::from(
                        "../test_data/image/image/not_found.png",
                    ))),
                    ..Default::default()
                },
            );
            test_util::assert_error_contains(
                CreateDisplacementGoalInputConfig::from_config(unverified_config),
                "does not exist in the filesystem", // Note: do not put a platform-dependent path string here
            );
        }

        #[test]
        fn dimensions_mismatch() {
            let unverified_config = UnverifiedCreateDisplacementGoalInputConfig::Input(
                UnverifiedCreateDisplacementGoalInputDataConfig {
                    displacement_goal: Some(String::from(
                        "../test_data/image/displacement_goal/identity_larger_displacement_goal.png",
                    )),
                    candidate_permutation: Some(String::from(
                        "../test_data/image/permutation/identity_permutation.png",
                    )),
                    ..Default::default()
                },
            );
            test_util::assert_error_contains(
                CreateDisplacementGoalInputConfig::from_config(unverified_config),
            "mismatch in image dimensions, (width, height) = (21, 25) and (width, height) = (20, 25)",
        );
        }
    }
}
