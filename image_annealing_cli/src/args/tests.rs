mod options {
    use super::super::make_option_parser;

    #[test]
    fn check_options() {
        make_option_parser().check_invariants(true)
    }
}

mod parse_args {
    use super::super::parse_args;
    use crate::config::{AlgorithmConfig, Config, ImagePath, PermutationPath};
    use bpaf::ParseFailure;
    use image_annealing::{compute, ImageDimensions};

    #[test]
    #[should_panic(expected = "no arguments (not even the program name)")]
    fn empty_input() {
        let v: Vec<String> = Vec::new();
        let _ = parse_args(v);
    }

    #[test]
    fn no_config_file() {
        let v = vec![String::from("one")];
        let message = parse_args(v).unwrap_err().unwrap_stderr();
        test_util::assert_error_contains::<(), String>(
            Err(message),
            "pass --help for usage information",
        );
    }

    #[test]
    fn valid_config_file() -> Result<(), ParseFailure> {
        let path = test_util::make_test_data_path_string([
            "config",
            "operation",
            "create_permutation",
            "valid.json",
        ]);
        let v = vec![String::from("one"), String::from("-c"), path];
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
                    image_dimensions: ImageDimensions::try_new(20, 25).unwrap()
                }
            }
        );
        Ok(())
    }

    #[test]
    fn additional_args() {
        let path = test_util::make_test_data_path_string([
            "config",
            "operation",
            "create_permutation",
            "valid.json",
        ]);
        let v = vec![
            String::from("one"),
            String::from("-c"),
            path,
            String::from("other_arg"),
        ];
        let message = parse_args(v).unwrap_err().unwrap_stderr();
        test_util::assert_error_contains::<(), String>(
            Err(message),
            "No such command: `other_arg`",
        );
    }
}
