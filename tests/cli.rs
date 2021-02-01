use image_annealing::cli;
use image_annealing::compute::format::{ImageFileWriter, PermutationImageBuffer};
use image_annealing::config::Config;
use image_annealing::image_utils::ImageDimensions;
use image_annealing::test_utils;
use std::error::Error;

#[test]
fn create_permutation() -> Result<(), Box<dyn Error>> {
    let path = test_utils::make_test_output_path_string(&["cli_create_permutation"]);
    let full_output_path = PermutationImageBuffer::make_filename(&path);
    if full_output_path.is_file() {
        panic!(format!(
            "Output permutation {} already exists in the filesystem.",
            full_output_path.display()
        ))
    }

    let config = Config::CreatePermutationConfig {
        image_dimensions: ImageDimensions::new(3, 4)?,
        permutation_output_path_no_extension: path,
    };
    cli::run(config)?;

    if !full_output_path.is_file() {
        panic!(format!(
            "Output permutation {} does not exist in the filesystem.",
            full_output_path.display()
        ))
    }
    std::fs::remove_file(full_output_path)?;

    Ok(())
}
