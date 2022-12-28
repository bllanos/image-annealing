mod parse_config_file {
    use super::super::super::{AlgorithmConfig, Config, ImagePath, PermutationPath};
    use super::super::parse_config_file;
    use image_annealing::{compute, ImageDimensions};
    use std::error::Error;

    #[test]
    fn missing_config_file() {
        let path = test_util::make_test_data_path(["config", "not_found.json"]);
        test_util::assert_error_contains(parse_config_file(path), "does not exist");
    }

    #[test]
    fn directory_not_file() {
        let path = test_util::make_test_data_path(["config"]);
        test_util::assert_error_contains(parse_config_file(path), "is not a file");
    }

    #[test]
    fn malformed_config_file() {
        let path = test_util::make_test_data_path(["config", "empty.json"]);
        test_util::assert_error_contains(
            parse_config_file(path),
            "configuration file deserialization error",
        );
    }

    #[test]
    fn valid_create_permutation_config_file() -> Result<(), Box<dyn Error>> {
        let path = test_util::make_test_data_path([
            "config",
            "operation",
            "create_permutation",
            "valid.json",
        ]);
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
                    image_dimensions: ImageDimensions::try_new(20, 25)?
                }
            }
        );
        Ok(())
    }

    #[test]
    fn invalid_create_permutation_config_file() {
        let path = test_util::make_test_data_path([
            "config",
            "operation",
            "create_permutation",
            "invalid.json",
        ]);
        test_util::assert_error_contains(parse_config_file(path), "width is zero");
    }
}
