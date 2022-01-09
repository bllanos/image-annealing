mod create_dispatcher {
    use super::super::create_dispatcher;
    use crate::config::Config;
    use image_annealing::ImageDimensions;
    use std::error::Error;

    #[test]
    fn create_permutation_config() -> Result<(), Box<dyn Error>> {
        create_dispatcher(&Config::CreatePermutation {
            image_dimensions: ImageDimensions::new(5, 6)?,
            permutation_output_path_no_extension: String::from("none"),
        })?;
        Ok(())
    }

    #[test]
    fn valid_permute_config() -> Result<(), Box<dyn Error>> {
        create_dispatcher(&Config::Permute {
            candidate_permutation_path: test_utils::make_test_data_path_string(&[
                "image",
                "permutation",
                "identity_permutation.png",
            ]),
            original_image_path: test_utils::make_test_data_path_string(&[
                "image",
                "image",
                "stripes.png",
            ]),
            permuted_image_output_path_no_extension: String::from("none"),
        })?;
        Ok(())
    }

    #[test]
    fn invalid_permute_config() -> Result<(), Box<dyn Error>> {
        test_utils::assert_error_contains(
            create_dispatcher(&Config::Permute {
                candidate_permutation_path: test_utils::make_test_data_path_string(&[
                    "image",
                    "permutation",
                    "not_found.png",
                ]),
                original_image_path: test_utils::make_test_data_path_string(&[
                    "image",
                    "image",
                    "not_found.png",
                ]),
                permuted_image_output_path_no_extension: String::from("none"),
            }),
            "No such file or directory",
        );
        Ok(())
    }

    #[test]
    fn valid_swap_config() -> Result<(), Box<dyn Error>> {
        create_dispatcher(&Config::Swap {
            candidate_permutation_path: test_utils::make_test_data_path_string(&[
                "image",
                "permutation",
                "identity_permutation.png",
            ]),
            displacement_goal_path: test_utils::make_test_data_path_string(&[
                "image",
                "displacement_goal",
                "identity_displacement_goal.png",
            ]),
            permutation_output_path_no_extension: String::from("none"),
        })?;
        Ok(())
    }

    #[test]
    fn invalid_swap_config() -> Result<(), Box<dyn Error>> {
        test_utils::assert_error_contains(
            create_dispatcher(&Config::Swap {
                candidate_permutation_path: test_utils::make_test_data_path_string(&[
                    "image",
                    "permutation",
                    "not_found.png",
                ]),
                displacement_goal_path: test_utils::make_test_data_path_string(&[
                    "image",
                    "displacement_goal",
                    "identity_displacement_goal.png",
                ]),
                permutation_output_path_no_extension: String::from("none"),
            }),
            "No such file or directory",
        );
        Ok(())
    }

    #[test]
    fn valid_validate_permutation_config() -> Result<(), Box<dyn Error>> {
        create_dispatcher(&Config::ValidatePermutation {
            candidate_permutation_path: test_utils::make_test_data_path_string(&[
                "image",
                "permutation",
                "identity_permutation.png",
            ]),
        })?;
        Ok(())
    }

    #[test]
    fn invalid_validate_permutation_config() -> Result<(), Box<dyn Error>> {
        test_utils::assert_error_contains(
            create_dispatcher(&Config::ValidatePermutation {
                candidate_permutation_path: test_utils::make_test_data_path_string(&[
                    "image",
                    "permutation",
                    "not_found.png",
                ]),
            }),
            "No such file or directory",
        );
        Ok(())
    }
}
