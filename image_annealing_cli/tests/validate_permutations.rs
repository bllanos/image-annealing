use image_annealing_cli::cli;
use image_annealing_cli::config::Config;
use std::error::Error;

#[test]
fn validate_permutation_valid() -> Result<(), Box<dyn Error>> {
    let config = Config::ValidatePermutationConfig {
        candidate_permutation_path: test_utils::make_test_data_path_string(&[
            "image",
            "permutation",
            "identity_permutation.png",
        ]),
    };
    cli::run(config)?;
    Ok(())
}

#[test]
fn validate_permutation_invalid() {
    let config = Config::ValidatePermutationConfig {
        candidate_permutation_path: test_utils::make_test_data_path_string(&[
            "image",
            "permutation",
            "invalid_permutation.png",
        ]),
    };
    cli::run(config)
        .expect_err("An error should be raised if the candidate permutation is invalid");
}
