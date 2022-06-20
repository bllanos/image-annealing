use image_annealing::compute::format::{ImageFileWriter, ImageFormat, VectorFieldImageBuffer};
use image_annealing::compute::{self, SwapPassSequence};
use image_annealing_cli::cli;
use image_annealing_cli::config::{
    AlgorithmConfig, Config, DisplacementGoalPath, ImagePath, IterationCount, PermutationPath,
    SwapParametersConfig, SwapStopConfig,
};
use std::error::Error;
use std::num::NonZeroUsize;

fn make_swap_parameters() -> SwapParametersConfig {
    SwapParametersConfig {
        stop: SwapStopConfig::Bounded {
            iteration_count: IterationCount(NonZeroUsize::new(2).unwrap()),
            threshold: None,
        },
        swap_acceptance_threshold: Default::default(),
        swap_pass_sequence: SwapPassSequence::from_passes([
            compute::SwapPass::Vertical,
            compute::SwapPass::OffsetVertical,
        ])
        .unwrap(),
        output_intermediate_permutations: true,
    }
}

#[test]
fn swap_valid() -> Result<(), Box<dyn Error>> {
    let path_prefix = test_utils::make_test_output_path_string(["cli_swap"]);
    let paths_no_extension = [
        format!("{}_round_0_pass_0_vertical", path_prefix),
        format!("{}_round_0_pass_1_offset_vertical", path_prefix),
        format!("{}_round_1_pass_0_vertical", path_prefix),
        format!("{}_round_1_pass_1_offset_vertical", path_prefix),
    ];
    let full_output_paths = paths_no_extension
        .iter()
        .map(|path| VectorFieldImageBuffer::make_filename(path))
        .collect::<Vec<_>>();
    for full_output_path in full_output_paths.iter() {
        assert!(!full_output_path.is_file());
    }

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
            permutation_output_path_prefix: PermutationPath::from_raw(path_prefix),
            parameters: make_swap_parameters(),
        },
        dispatcher: compute::Config { image_dimensions },
    };
    cli::run(config)?;

    for full_output_path in full_output_paths {
        assert!(full_output_path.is_file());
        std::fs::remove_file(full_output_path)?;
    }

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
            permutation_output_path_prefix: PermutationPath::from_raw(
                test_utils::make_test_output_path_string(["cli_swap_invalid"]),
            ),
            parameters: make_swap_parameters(),
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
            permutation_output_path_prefix: PermutationPath::from_raw(
                test_utils::make_test_output_path_string(["cli_swap_invalid_permutation_format"]),
            ),
            parameters: make_swap_parameters(),
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
            permutation_output_path_prefix: PermutationPath::from_raw(
                test_utils::make_test_output_path_string([
                    "cli_swap_invalid_displacement_goal_format",
                ]),
            ),
            parameters: make_swap_parameters(),
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
            permutation_output_path_prefix: PermutationPath::from_raw(path),
            parameters: make_swap_parameters(),
        },
        dispatcher: compute::Config { image_dimensions },
    };
    test_utils::assert_error_contains(cli::run(config), "No such file or directory");
    Ok(())
}
