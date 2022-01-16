use image_annealing_cli::cli;
use image_annealing_cli::config::Config;
use image_annealing_cli::CandidatePermutationPath;
use std::error::Error;

#[test]
fn validate_permutation_valid() -> Result<(), Box<dyn Error>> {
    let config = Config::ValidatePermutation {
        candidate_permutation: CandidatePermutationPath(test_utils::make_test_data_path_string(&[
            "image",
            "permutation",
            "identity_permutation.png",
        ])),
    };
    cli::run(config)?;
    Ok(())
}

#[test]
fn validate_permutation_invalid() {
    let config = Config::ValidatePermutation {
        candidate_permutation: CandidatePermutationPath(test_utils::make_test_data_path_string(&[
            "image",
            "permutation",
            "invalid_permutation.png",
        ])),
    };
    test_utils::assert_error_contains(cli::run(config), "out of bounds mapping (x, y, delta_x, delta_y) = (3, 10, 257, 511) for an image of dimensions (width, height) = (20, 25)");
}
