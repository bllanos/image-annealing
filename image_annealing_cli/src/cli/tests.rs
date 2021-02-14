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
}
