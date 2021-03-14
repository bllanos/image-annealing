mod create_dispatcher {
    use super::super::create_dispatcher;
    use crate::config::Config;
    use image_annealing::image_utils::ImageDimensions;
    use std::error::Error;

    #[test]
    fn create_permutation_config() -> Result<(), Box<dyn Error>> {
        create_dispatcher(&Config::CreatePermutationConfig {
            image_dimensions: ImageDimensions::new(5, 6)?,
            permutation_output_path_no_extension: String::from("none"),
        })?;
        Ok(())
    }

    #[test]
    fn valid_validate_permutation_config() -> Result<(), Box<dyn Error>> {
        create_dispatcher(&Config::ValidatePermutationConfig {
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
        let r = create_dispatcher(&Config::ValidatePermutationConfig {
            candidate_permutation_path: test_utils::make_test_data_path_string(&[
                "image",
                "permutation",
                "not_found.png",
            ]),
        });
        match r {
            Ok(_) => panic!("A non-existant candidate permutation should trigger an error"),
            Err(_) => Ok(()),
        }
    }
}
