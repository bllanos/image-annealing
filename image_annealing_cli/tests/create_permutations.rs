use image_annealing::compute::format::{ImageFileReader, ImageFileWriter, VectorFieldImageBuffer};
use image_annealing::{compute, CandidatePermutation, ImageDimensions, ImageDimensionsHolder};
use image_annealing_cli::cli;
use image_annealing_cli::config::{AlgorithmConfig, Config, ImagePath, PermutationPath};
use std::error::Error;
use test_utils::permutation;

#[test]
fn create_permutation() -> Result<(), Box<dyn Error>> {
    let path = test_utils::make_test_output_path_string(["cli_create_permutation"]);
    let full_output_path = VectorFieldImageBuffer::make_filename(&path);
    assert!(!full_output_path.is_file());

    let dimensions = ImageDimensions::try_new(3, 4)?;
    let config = Config {
        algorithm: AlgorithmConfig::CreatePermutation {
            permutation_output_path_no_extension: PermutationPath::from_raw(path),
        },
        dispatcher: compute::Config {
            image_dimensions: dimensions,
        },
    };
    cli::run(config)?;

    let output_permutation = CandidatePermutation::load(&full_output_path)?;
    permutation::assert_is_identity(&output_permutation);
    assert_eq!(output_permutation.dimensions(), &dimensions);
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
            image_dimensions: ImageDimensions::try_new(3, 4)?,
        },
    };
    test_utils::assert_error_contains(cli::run(config), "No such file or directory");
    Ok(())
}
