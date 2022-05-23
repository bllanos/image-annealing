mod parse_config_file {
    use super::super::super::{AlgorithmConfig, Config, ImagePath, PermutationPath};
    use super::super::parse_config_file;
    use image_annealing::{compute, ImageDimensions};
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
            Config {
                algorithm: AlgorithmConfig::CreatePermutation {
                    permutation_output_path_no_extension: PermutationPath::from_raw_clone(
                        "permutation_out"
                    ),
                },
                dispatcher: compute::Config {
                    image_dimensions: ImageDimensions::new(20, 25)?
                }
            }
        );
        Ok(())
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
