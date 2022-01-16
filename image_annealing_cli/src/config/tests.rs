mod parse_args {
    use super::super::{parse_args, Config};
    use image_annealing::ImageDimensions;
    use std::error::Error;

    #[test]
    #[should_panic(expected = "no arguments (not even the program name)")]
    fn empty_input() {
        let v: Vec<String> = Vec::new();
        let _ = parse_args(v);
    }

    #[test]
    fn no_config_file() {
        let v = vec![String::from("one")];
        test_utils::assert_error_contains(
            parse_args(v),
            "expected at least one argument for a configuration file's path",
        );
    }

    #[test]
    fn valid_config_file() -> Result<(), Box<dyn Error>> {
        let path =
            test_utils::make_test_data_path_string(&["config", "create_permutation", "valid.json"]);
        let v = vec![String::from("one"), path];
        let r = parse_args(v)?;
        assert_eq!(
            r,
            Config::CreatePermutation {
                image_dimensions: ImageDimensions::new(20, 25)?,
                permutation_output_path_no_extension: String::from("permutation_out"),
            }
        );
        Ok(())
    }

    #[test]
    fn additional_args() -> Result<(), Box<dyn Error>> {
        let path =
            test_utils::make_test_data_path_string(&["config", "create_permutation", "valid.json"]);
        let v = vec![String::from("one"), path, String::from("other_arg")];
        let r = parse_args(v)?;
        assert_eq!(
            r,
            Config::CreatePermutation {
                image_dimensions: ImageDimensions::new(20, 25)?,
                permutation_output_path_no_extension: String::from("permutation_out"),
            }
        );
        Ok(())
    }
}

mod parse_config_file {
    use super::super::{parse_config_file, Config};
    use crate::{CandidatePermutationPath, DisplacementGoalPath, ImagePath};
    use image_annealing::ImageDimensions;
    use std::error::Error;

    #[test]
    fn malformed_config_file() {
        let path = test_utils::make_test_data_path(&["config", "empty.json"]);
        test_utils::assert_error_contains(
            parse_config_file(path),
            "configuration file deserialization error",
        );
    }

    #[test]
    fn valid_create_permutation_config_file() -> Result<(), Box<dyn Error>> {
        let path = test_utils::make_test_data_path(&["config", "create_permutation", "valid.json"]);
        let r = parse_config_file(path)?;
        assert_eq!(
            r,
            Config::CreatePermutation {
                image_dimensions: ImageDimensions::new(20, 25)?,
                permutation_output_path_no_extension: String::from("permutation_out"),
            }
        );
        Ok(())
    }

    #[test]
    fn invalid_create_permutation_config_file() {
        let path = test_utils::make_test_data_path(&[
            "config",
            "create_permutation",
            "invalid_dimensions.json",
        ]);
        test_utils::assert_error_contains(parse_config_file(path), "width is zero");
    }

    #[test]
    fn valid_permute_config_file() -> Result<(), Box<dyn Error>> {
        let path = test_utils::make_test_data_path(&["config", "permute", "valid.json"]);
        let r = parse_config_file(path)?;
        assert_eq!(
            r,
            Config::Permute {
                candidate_permutation: CandidatePermutationPath(
                    test_utils::make_test_data_path_string(&[
                        "image",
                        "permutation",
                        "identity_permutation.png"
                    ])
                ),
                original_image: ImagePath(test_utils::make_test_data_path_string(&[
                    "image",
                    "image",
                    "stripes.png"
                ])),
                permuted_image_output_path_no_extension: String::from("permuted_image_out"),
            }
        );
        Ok(())
    }

    #[test]
    fn invalid_permute_config_file_permutation() {
        let path = test_utils::make_test_data_path(&[
            "config",
            "permute",
            "candidate_permutation_not_found.json",
        ]);
        test_utils::assert_error_contains(
            parse_config_file(path),
            "does not exist in the filesystem", // Note: do not put a platform-dependent path string here
        );
    }

    #[test]
    fn invalid_permute_config_file_image() {
        let path = test_utils::make_test_data_path(&[
            "config",
            "permute",
            "original_image_not_found.json",
        ]);
        test_utils::assert_error_contains(
            parse_config_file(path),
            "does not exist in the filesystem", // Note: do not put a platform-dependent path string here
        );
    }

    #[test]
    fn invalid_permute_config_file_dimensions() {
        let path =
            test_utils::make_test_data_path(&["config", "permute", "dimensions_mismatch.json"]);
        test_utils::assert_error_contains(
            parse_config_file(path),
            "mismatch in image dimensions, (width, height) = (200, 200) and (width, height) = (20, 25)",
        );
    }

    #[test]
    fn valid_swap_config_file() -> Result<(), Box<dyn Error>> {
        let path = test_utils::make_test_data_path(&["config", "swap", "valid.json"]);
        let r = parse_config_file(path)?;
        assert_eq!(
            r,
            Config::Swap {
                candidate_permutation: CandidatePermutationPath(
                    test_utils::make_test_data_path_string(&[
                        "image",
                        "permutation",
                        "identity_permutation.png"
                    ])
                ),
                displacement_goal: DisplacementGoalPath(test_utils::make_test_data_path_string(&[
                    "image",
                    "displacement_goal",
                    "identity_displacement_goal.png"
                ])),
                permutation_output_path_no_extension: String::from("permutation_out"),
            }
        );
        Ok(())
    }

    #[test]
    fn invalid_swap_config_file_permutation() {
        let path = test_utils::make_test_data_path(&[
            "config",
            "swap",
            "candidate_permutation_not_found.json",
        ]);
        test_utils::assert_error_contains(
            parse_config_file(path),
            "does not exist in the filesystem", // Note: do not put a platform-dependent path string here
        );
    }

    #[test]
    fn invalid_swap_config_file_displacement_goal() {
        let path = test_utils::make_test_data_path(&[
            "config",
            "swap",
            "displacement_goal_not_found.json",
        ]);
        test_utils::assert_error_contains(
            parse_config_file(path),
            "does not exist in the filesystem", // Note: do not put a platform-dependent path string here
        );
    }

    #[test]
    fn invalid_swap_config_file_dimensions() {
        let path = test_utils::make_test_data_path(&["config", "swap", "dimensions_mismatch.json"]);
        test_utils::assert_error_contains(
            parse_config_file(path),
            "mismatch in image dimensions, (width, height) = (21, 25) and (width, height) = (20, 25)",
        );
    }

    #[test]
    fn valid_validate_permutation_config_file() -> Result<(), Box<dyn Error>> {
        let path =
            test_utils::make_test_data_path(&["config", "validate_permutation", "valid.json"]);
        let r = parse_config_file(path)?;
        assert_eq!(
            r,
            Config::ValidatePermutation {
                candidate_permutation: CandidatePermutationPath(
                    test_utils::make_test_data_path_string(&[
                        "image",
                        "permutation",
                        "identity_permutation.png"
                    ])
                ),
            }
        );
        Ok(())
    }

    #[test]
    fn invalid_validate_permutation_config_file() {
        let path = test_utils::make_test_data_path(&[
            "config",
            "validate_permutation",
            "candidate_permutation_not_found.json",
        ]);
        test_utils::assert_error_contains(
            parse_config_file(path),
            "does not exist in the filesystem", // Note: do not put a platform-dependent path string here
        );
    }
}

mod check_input_path {
    use super::super::check_input_path;
    use std::error::Error;
    use std::path::Path;

    #[test]
    fn absent_file() {
        let path = test_utils::make_test_data_path(&["none.png"]);
        test_utils::assert_error_contains(
            check_input_path(path),
            "does not exist in the filesystem", // Note: do not put a platform-dependent path string here
        );
    }

    #[test]
    fn not_a_file() {
        let path = test_utils::make_test_data_path::<Vec<&Path>, &Path>(Vec::new());
        test_utils::assert_error_contains(check_input_path(path), "file");
    }

    #[test]
    fn valid_file() -> Result<(), Box<dyn Error>> {
        let path = test_utils::make_test_data_path(&["image", "image", "radial_gradient_rg.png"]);
        Ok(check_input_path(path)?)
    }
}

mod convert_path_separators {
    use super::super::convert_path_separators;
    use std::path::MAIN_SEPARATOR;

    #[test]
    fn windows_path() {
        let filepath = String::from("one\\two\\three\\..\\.\\end.txt");
        let expected = filepath.clone();
        let converted = convert_path_separators(filepath);
        if MAIN_SEPARATOR == '\\' {
            assert_eq!(converted, expected);
        } else {
            assert!(converted.find('\\').is_none());
            assert!(converted.find(MAIN_SEPARATOR).is_some());
        }
    }

    #[test]
    fn unix_path() {
        let filepath = String::from("one/two/three/.././end.txt");
        let expected = filepath.clone();
        let converted = convert_path_separators(filepath);
        if MAIN_SEPARATOR == '/' {
            assert_eq!(converted, expected);
        } else {
            assert!(converted.find('/').is_none());
            assert!(converted.find(MAIN_SEPARATOR).is_some());
        }
    }

    #[test]
    fn no_separators() {
        let filepath = String::from("end.txt");
        let expected = filepath.clone();
        let converted = convert_path_separators(filepath);
        assert_eq!(converted, expected);
    }
}
