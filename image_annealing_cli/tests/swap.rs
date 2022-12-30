use image_annealing::compute::conversion::{self, VectorFieldEntry};
use image_annealing::compute::format::{
    ImageFileReader, ImageFileWriter, ImageFormat, VectorFieldImageBuffer,
};
use image_annealing::compute::{self, SwapPassSequence};
use image_annealing::ImageDimensions;
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
    // Ensure output files do not already exist
    let path_prefix = test_util::make_test_output_path_string(["cli_swap"]);
    let paths_no_extension = [
        format!("{}_round_0_pass_0_vertical", path_prefix),
        format!("{}_round_0_pass_1_offset_vertical", path_prefix),
        format!("{}_round_1_pass_0_vertical", path_prefix),
        format!("{}_round_1_pass_1_offset_vertical", path_prefix),
    ];
    let full_output_paths = paths_no_extension
        .iter()
        .map(VectorFieldImageBuffer::make_filename)
        .collect::<Vec<_>>();
    for full_output_path in full_output_paths.iter() {
        assert!(!full_output_path.is_file());
    }

    // Synthesize input files
    let dimensions = ImageDimensions::try_new(1, 6)?;
    let input_permutation = conversion::to_image(
        &dimensions,
        &[
            VectorFieldEntry(0, 1),
            VectorFieldEntry(0, -1),
            VectorFieldEntry(0, 0),
            VectorFieldEntry(0, 0),
            VectorFieldEntry(0, 0),
            VectorFieldEntry(0, 0),
        ],
    );
    let input_permutation_path_prefix =
        test_util::make_test_output_path(["cli_swap_input_permutation"]);
    let input_permutation_path =
        input_permutation.save_add_extension(input_permutation_path_prefix)?;

    let input_displacement_goal = conversion::to_image(
        &dimensions,
        &[
            VectorFieldEntry(0, 3),
            VectorFieldEntry(0, 3),
            VectorFieldEntry(0, 3),
            VectorFieldEntry(0, -3),
            VectorFieldEntry(0, -3),
            VectorFieldEntry(0, -3),
        ],
    );
    let input_displacement_goal_path_prefix =
        test_util::make_test_output_path(["cli_swap_input_displacement_goal"]);
    let input_displacement_goal_path =
        input_displacement_goal.save_add_extension(input_displacement_goal_path_prefix)?;

    let config = Config {
        algorithm: AlgorithmConfig::Swap {
            candidate_permutation: PermutationPath::from_raw_clone(
                input_permutation_path.to_str().unwrap(),
            ),
            displacement_goal: DisplacementGoalPath::from_raw_clone(
                input_displacement_goal_path.to_str().unwrap(),
            ),
            permutation_output_path_prefix: PermutationPath::from_raw(path_prefix),
            parameters: make_swap_parameters(),
        },
        dispatcher: compute::Config {
            image_dimensions: dimensions,
        },
    };
    cli::run(config)?;

    for (i, full_output_path) in full_output_paths.iter().enumerate() {
        let output_permutation = VectorFieldImageBuffer::load(full_output_path)?;
        let expected_permutation = match i {
            0 => &[
                VectorFieldEntry(0, 1),
                VectorFieldEntry(0, -1),
                VectorFieldEntry(0, 1),
                VectorFieldEntry(0, -1),
                VectorFieldEntry(0, 1),
                VectorFieldEntry(0, -1),
            ],
            1 => &[
                VectorFieldEntry(0, 1),
                VectorFieldEntry(0, 2),
                VectorFieldEntry(0, -2),
                VectorFieldEntry(0, 2),
                VectorFieldEntry(0, -2),
                VectorFieldEntry(0, -1),
            ],
            2 | 3 => &[
                VectorFieldEntry(0, 3),
                VectorFieldEntry(0, 0),
                VectorFieldEntry(0, 3),
                VectorFieldEntry(0, -3),
                VectorFieldEntry(0, 0),
                VectorFieldEntry(0, -3),
            ],
            _ => unreachable!(),
        };
        assert_eq!(
            &conversion::to_vec(&output_permutation),
            expected_permutation
        );
    }

    std::fs::remove_file(input_permutation_path)?;
    std::fs::remove_file(input_displacement_goal_path)?;
    for full_output_path in full_output_paths {
        std::fs::remove_file(full_output_path)?;
    }

    Ok(())
}

#[test]
fn swap_invalid() -> Result<(), Box<dyn Error>> {
    let (candidate_permutation_path, image_dimensions) =
        PermutationPath::from_input_path(test_util::make_test_data_path_string([
            "image",
            "permutation",
            "invalid_permutation.png",
        ]))?;
    let config = Config {
        algorithm: AlgorithmConfig::Swap {
            candidate_permutation: candidate_permutation_path,
            displacement_goal: DisplacementGoalPath::from_raw(
                test_util::make_test_data_path_string([
                    "image",
                    "displacement_goal",
                    "identity_displacement_goal.png",
                ]),
            ),
            permutation_output_path_prefix: PermutationPath::from_raw(
                test_util::make_test_output_path_string(["cli_swap_invalid"]),
            ),
            parameters: make_swap_parameters(),
        },
        dispatcher: compute::Config { image_dimensions },
    };
    test_util::assert_error_contains(cli::run(config), "out of bounds mapping (x, y, delta_x, delta_y) = (3, 10, 257, 511) for an image of dimensions (width, height) = (20, 25)");
    Ok(())
}

#[test]
fn invalid_permutation_format() -> Result<(), Box<dyn Error>> {
    let (candidate_permutation_path, image_dimensions) =
        PermutationPath::from_input_path(test_util::make_test_data_path_string([
            "image", "image", "red.png",
        ]))?;
    let config = Config {
        algorithm: AlgorithmConfig::Swap {
            candidate_permutation: candidate_permutation_path,
            displacement_goal: DisplacementGoalPath::from_raw(
                test_util::make_test_data_path_string([
                    "image",
                    "displacement_goal",
                    "identity_displacement_goal.png",
                ]),
            ),
            permutation_output_path_prefix: PermutationPath::from_raw(
                test_util::make_test_output_path_string(["cli_swap_invalid_permutation_format"]),
            ),
            parameters: make_swap_parameters(),
        },
        dispatcher: compute::Config { image_dimensions },
    };
    test_util::assert_error_contains(
        cli::run(config),
        &format!("not the expected format of {}", ImageFormat::Rgba8),
    );
    Ok(())
}

#[test]
fn invalid_displacement_goal_format() -> Result<(), Box<dyn Error>> {
    let (candidate_permutation_path, image_dimensions) =
        PermutationPath::from_input_path(test_util::make_test_data_path_string([
            "image",
            "permutation",
            "identity_permutation.png",
        ]))?;
    let config = Config {
        algorithm: AlgorithmConfig::Swap {
            candidate_permutation: candidate_permutation_path,
            displacement_goal: DisplacementGoalPath::from_raw(
                test_util::make_test_data_path_string(["image", "image", "red.png"]),
            ),
            permutation_output_path_prefix: PermutationPath::from_raw(
                test_util::make_test_output_path_string([
                    "cli_swap_invalid_displacement_goal_format",
                ]),
            ),
            parameters: make_swap_parameters(),
        },
        dispatcher: compute::Config { image_dimensions },
    };
    test_util::assert_error_contains(
        cli::run(config),
        &format!("not the expected format of {}", ImageFormat::Rgba8),
    );
    Ok(())
}

#[test]
fn save_missing_directory() -> Result<(), Box<dyn Error>> {
    let path = test_util::make_test_output_path_string(["not_found", "cannot_create"]);
    let (candidate_permutation_path, image_dimensions) =
        PermutationPath::from_input_path(test_util::make_test_data_path_string([
            "image",
            "permutation",
            "identity_permutation.png",
        ]))?;
    let config = Config {
        algorithm: AlgorithmConfig::Swap {
            candidate_permutation: candidate_permutation_path,
            displacement_goal: DisplacementGoalPath::from_raw(
                test_util::make_test_data_path_string([
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
    test_util::assert_error_contains(cli::run(config), "No such file or directory");
    Ok(())
}
