mod parse_args {
    use super::super::{parse_args, AlgorithmConfig, Config, ImagePath, PermutationPath};
    use image_annealing::{compute, ImageDimensions};
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

    #[test]
    fn additional_args() -> Result<(), Box<dyn Error>> {
        let path =
            test_utils::make_test_data_path_string(&["config", "create_permutation", "valid.json"]);
        let v = vec![String::from("one"), path, String::from("other_arg")];
        let r = parse_args(v)?;
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
