use image_annealing::compute;
use image_annealing::compute::format::ImageFormat;
use image_annealing_cli::cli;
use image_annealing_cli::config::{AlgorithmConfig, Config, PermutationPath};
use std::error::Error;

#[test]
fn validate_permutation_valid() -> Result<(), Box<dyn Error>> {
    let (candidate_permutation_path, image_dimensions) =
        PermutationPath::from_input_path(test_utils::make_test_data_path_string(&[
            "image",
            "permutation",
            "identity_permutation.png",
        ]))?;
    let config = Config {
        algorithm: AlgorithmConfig::ValidatePermutation {
            candidate_permutation: candidate_permutation_path,
        },
        dispatcher: compute::Config { image_dimensions },
    };
    cli::run(config)?;
    Ok(())
}

#[test]
fn validate_permutation_invalid() -> Result<(), Box<dyn Error>> {
    let (candidate_permutation_path, image_dimensions) =
        PermutationPath::from_input_path(test_utils::make_test_data_path_string(&[
            "image",
            "permutation",
            "invalid_permutation.png",
        ]))?;
    let config = Config {
        algorithm: AlgorithmConfig::ValidatePermutation {
            candidate_permutation: candidate_permutation_path,
        },
        dispatcher: compute::Config { image_dimensions },
    };
    test_utils::assert_error_contains(cli::run(config), "out of bounds mapping (x, y, delta_x, delta_y) = (3, 10, 257, 511) for an image of dimensions (width, height) = (20, 25)");
    Ok(())
}

#[test]
fn invalid_permutation_format() -> Result<(), Box<dyn Error>> {
    let (candidate_permutation_path, image_dimensions) =
        PermutationPath::from_input_path(test_utils::make_test_data_path_string(&[
            "image", "image", "red.png",
        ]))?;
    let config = Config {
        algorithm: AlgorithmConfig::ValidatePermutation {
            candidate_permutation: candidate_permutation_path,
        },
        dispatcher: compute::Config { image_dimensions },
    };
    test_utils::assert_error_contains(
        cli::run(config),
        &format!("not the expected format of {}", ImageFormat::Rgba8),
    );
    Ok(())
}
