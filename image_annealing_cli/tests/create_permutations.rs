use image_annealing::compute::format::{ImageFileWriter, VectorFieldImageBuffer};
use image_annealing::{compute, ImageDimensions};
use image_annealing_cli::cli;
use image_annealing_cli::config::{AlgorithmConfig, Config, ImagePath, PermutationPath};
use std::error::Error;

#[test]
fn create_permutation() -> Result<(), Box<dyn Error>> {
    let path = test_utils::make_test_output_path_string(["cli_create_permutation"]);
    let full_output_path = VectorFieldImageBuffer::make_filename(&path);
    assert!(!full_output_path.is_file());

    let config = Config {
        algorithm: AlgorithmConfig::CreatePermutation {
            permutation_output_path_no_extension: PermutationPath::from_raw(path),
        },
        dispatcher: compute::Config {
            image_dimensions: ImageDimensions::new(3, 4)?,
        },
    };
    cli::run(config)?;

    assert!(full_output_path.is_file());
    std::fs::remove_file(full_output_path)?;

    Ok(())
}

#[test]
fn save_missing_directory() -> Result<(), Box<dyn Error>> {
    let path = test_utils::make_test_output_path_string(["not_found", "cannot_create"]);

    let config = Config {
        algorithm: AlgorithmConfig::CreatePermutation {
            permutation_output_path_no_extension: PermutationPath::from_raw(path),
        },
        dispatcher: compute::Config {
            image_dimensions: ImageDimensions::new(3, 4)?,
        },
    };
    test_utils::assert_error_contains(cli::run(config), "No such file or directory");
    Ok(())
}
