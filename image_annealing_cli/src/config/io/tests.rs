mod parse_config_file {
    use super::super::super::{AlgorithmConfig, Config, OutputPermutationPath};
    use super::super::parse_config_file;
    use image_annealing::{compute, ImageDimensions};
    use image_annealing_cli_util::path::OutputFilePath;
    use std::borrow::Cow;
    use std::error::Error;

    #[test]
    fn missing_config_file() {
        let path = test_util::path::unverified_absolute_input_path("config/not_found.json");
        test_util::assert_error_contains(parse_config_file(path), "does not exist");
    }

    #[test]
    fn directory_not_file() {
        let path = test_util::path::absolute_input_directory("config");
        test_util::assert_error_contains(parse_config_file(path.0), "is not a file");
    }

    #[test]
    fn malformed_config_file() {
        let path = test_util::path::absolute_input_file("config/empty.json");
        test_util::assert_error_contains(
            parse_config_file(path.0),
            "error parsing the contents of",
        );
    }

    #[test]
    fn valid_create_permutation_config_file() -> Result<(), Box<dyn Error>> {
        let path =
            test_util::path::absolute_input_file("config/operation/create_permutation/valid.json");
        let r = parse_config_file(&path.0)?;
        assert_eq!(
            r,
            Config {
                algorithm: AlgorithmConfig::CreatePermutation {
                    permutation_output_path_no_extension: OutputPermutationPath(OutputFilePath(
                        Cow::Owned(path.0.parent().unwrap().join("permutation_out"))
                    )),
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
        let path = test_util::path::absolute_input_file(
            "config/operation/create_permutation/invalid.json",
        );
        test_util::assert_error_contains(parse_config_file(path.0), "width is zero");
    }
}
