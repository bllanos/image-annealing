use image_annealing::compute;
use image_annealing::compute::format::{ImageFileWriter, ImageFormat, VectorFieldImageBuffer};
use image_annealing_cli::cli;
use image_annealing_cli::config::{
    AlgorithmConfig, Config, DisplacementGoalPath, ImagePath, PermutationPath,
    SwapParametersConfig, SwapStopConfig, SwapStopThreshold,
};
use std::error::Error;

#[test]
fn swap_valid() -> Result<(), Box<dyn Error>> {
    let path = test_utils::make_test_output_path_string(["cli_swap"]);
    let full_output_path = VectorFieldImageBuffer::make_filename(&path);
    assert!(!full_output_path.is_file());

    let (candidate_permutation_path, image_dimensions) =
        PermutationPath::from_input_path(test_utils::make_test_data_path_string([
            "image",
            "permutation",
            "identity_permutation.png",
        ]))?;
    let config = Config {
        algorithm: AlgorithmConfig::Swap {
            candidate_permutation: candidate_permutation_path,
            displacement_goal: DisplacementGoalPath::from_raw(
                test_utils::make_test_data_path_string([
                    "image",
                    "displacement_goal",
                    "identity_displacement_goal.png",
                ]),
            ),
            permutation_output_path_no_extension: PermutationPath::from_raw(path),
            parameters: SwapParametersConfig {
                stop: SwapStopConfig::Unbounded(SwapStopThreshold::SwapsAccepted(0)),
                swap_acceptance_threshold: Default::default(),
            },
        },
        dispatcher: compute::Config { image_dimensions },
    };
    cli::run(config)?;

    assert!(full_output_path.is_file());
    std::fs::remove_file(full_output_path)?;

    Ok(())
}

#[test]
fn swap_invalid() -> Result<(), Box<dyn Error>> {
    let (candidate_permutation_path, image_dimensions) =
        PermutationPath::from_input_path(test_utils::make_test_data_path_string([
            "image",
            "permutation",
            "invalid_permutation.png",
        ]))?;
    let config = Config {
        algorithm: AlgorithmConfig::Swap {
            candidate_permutation: candidate_permutation_path,
            displacement_goal: DisplacementGoalPath::from_raw(
                test_utils::make_test_data_path_string([
                    "image",
                    "displacement_goal",
                    "identity_displacement_goal.png",
                ]),
            ),
            permutation_output_path_no_extension: PermutationPath::from_raw(
                test_utils::make_test_output_path_string(["cli_swap_invalid"]),
            ),
            parameters: SwapParametersConfig {
                stop: SwapStopConfig::Unbounded(SwapStopThreshold::SwapsAccepted(0)),
                swap_acceptance_threshold: Default::default(),
            },
        },
        dispatcher: compute::Config { image_dimensions },
    };
    test_utils::assert_error_contains(cli::run(config), "out of bounds mapping (x, y, delta_x, delta_y) = (3, 10, 257, 511) for an image of dimensions (width, height) = (20, 25)");
    Ok(())
}

#[test]
fn invalid_permutation_format() -> Result<(), Box<dyn Error>> {
    let (candidate_permutation_path, image_dimensions) =
        PermutationPath::from_input_path(test_utils::make_test_data_path_string([
            "image", "image", "red.png",
        ]))?;
    let config = Config {
        algorithm: AlgorithmConfig::Swap {
            candidate_permutation: candidate_permutation_path,
            displacement_goal: DisplacementGoalPath::from_raw(
                test_utils::make_test_data_path_string([
                    "image",
                    "displacement_goal",
                    "identity_displacement_goal.png",
                ]),
            ),
            permutation_output_path_no_extension: PermutationPath::from_raw(
                test_utils::make_test_output_path_string(["cli_swap_invalid_permutation_format"]),
            ),
            parameters: SwapParametersConfig {
                stop: SwapStopConfig::Unbounded(SwapStopThreshold::SwapsAccepted(0)),
                swap_acceptance_threshold: Default::default(),
            },
        },
        dispatcher: compute::Config { image_dimensions },
    };
    test_utils::assert_error_contains(
        cli::run(config),
        &format!("not the expected format of {}", ImageFormat::Rgba8),
    );
    Ok(())
}

#[test]
fn invalid_displacement_goal_format() -> Result<(), Box<dyn Error>> {
    let (candidate_permutation_path, image_dimensions) =
        PermutationPath::from_input_path(test_utils::make_test_data_path_string([
            "image",
            "permutation",
            "identity_permutation.png",
        ]))?;
    let config = Config {
        algorithm: AlgorithmConfig::Swap {
            candidate_permutation: candidate_permutation_path,
            displacement_goal: DisplacementGoalPath::from_raw(
                test_utils::make_test_data_path_string(["image", "image", "red.png"]),
            ),
            permutation_output_path_no_extension: PermutationPath::from_raw(
                test_utils::make_test_output_path_string([
                    "cli_swap_invalid_displacement_goal_format",
                ]),
            ),
            parameters: SwapParametersConfig {
                stop: SwapStopConfig::Unbounded(SwapStopThreshold::SwapsAccepted(0)),
                swap_acceptance_threshold: Default::default(),
            },
        },
        dispatcher: compute::Config { image_dimensions },
    };
    test_utils::assert_error_contains(
        cli::run(config),
        &format!("not the expected format of {}", ImageFormat::Rgba8),
    );
    Ok(())
}

#[test]
fn save_missing_directory() -> Result<(), Box<dyn Error>> {
    let path = test_utils::make_test_output_path_string(["not_found", "cannot_create"]);
    let (candidate_permutation_path, image_dimensions) =
        PermutationPath::from_input_path(test_utils::make_test_data_path_string([
            "image",
            "permutation",
            "identity_permutation.png",
        ]))?;
    let config = Config {
        algorithm: AlgorithmConfig::Swap {
            candidate_permutation: candidate_permutation_path,
            displacement_goal: DisplacementGoalPath::from_raw(
                test_utils::make_test_data_path_string([
                    "image",
                    "displacement_goal",
                    "identity.png",
                ]),
            ),
            permutation_output_path_no_extension: PermutationPath::from_raw(path),
            parameters: SwapParametersConfig {
                stop: SwapStopConfig::Unbounded(SwapStopThreshold::SwapsAccepted(0)),
                swap_acceptance_threshold: Default::default(),
            },
        },
        dispatcher: compute::Config { image_dimensions },
    };
    test_utils::assert_error_contains(cli::run(config), "No such file or directory");
    Ok(())
}
