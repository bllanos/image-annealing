use image_annealing::compute::conversion::{self, VectorFieldEntry};
use image_annealing::compute::format::{
    ImageFileReader, ImageFileWriter, ImageFormat, VectorFieldImageBuffer,
};
use image_annealing::compute::{self, SwapPassSequence};
use image_annealing::ImageDimensions;
use image_annealing_cli::cli;
use image_annealing_cli::config::{
    AlgorithmConfig, Config, InputDisplacementGoalPath, InputPermutationPath, IterationCount,
    OutputPermutationPath, SwapParametersConfig, SwapStopConfig, UnverifiedInputPermutationPath,
};
use image_annealing_cli_util::path::{InputFilePath, OutputFilePath};
use std::borrow::Cow;
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
    let path_prefix = test_util::unique_absolute_output_file!();
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
    let input_permutation_path_prefix = test_util::unique_absolute_output_file!();
    let input_permutation_path =
        input_permutation.save_add_extension(&input_permutation_path_prefix.0)?;

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
    let input_displacement_goal_path_prefix = test_util::unique_absolute_output_file!();
    let input_displacement_goal_path =
        input_displacement_goal.save_add_extension(&input_displacement_goal_path_prefix.0)?;

    let config = Config {
        algorithm: AlgorithmConfig::Swap {
            candidate_permutation: InputPermutationPath(InputFilePath(Cow::Borrowed(
                &input_permutation_path,
            ))),
            displacement_goal: InputDisplacementGoalPath(InputFilePath(Cow::Borrowed(
                &input_displacement_goal_path,
            ))),
            permutation_output_path_prefix: OutputPermutationPath(path_prefix),
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
        InputPermutationPath::try_from_unverified_with_path_context(
            UnverifiedInputPermutationPath(test_util::path::relative_input_file(
                "image/permutation/invalid_permutation.png",
            )),
            test_util::path::base_input().0,
        )?;
    let config = Config {
        algorithm: AlgorithmConfig::Swap {
            candidate_permutation: candidate_permutation_path,
            displacement_goal: InputDisplacementGoalPath(test_util::path::absolute_input_file(
                "image/displacement_goal/identity_displacement_goal.png",
            )),
            permutation_output_path_prefix: OutputPermutationPath(
                test_util::unique_absolute_output_file!(),
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
        InputPermutationPath::try_from_unverified_with_path_context(
            UnverifiedInputPermutationPath(test_util::path::relative_input_file(
                "image/image/red.png",
            )),
            test_util::path::base_input().0,
        )?;
    let config = Config {
        algorithm: AlgorithmConfig::Swap {
            candidate_permutation: candidate_permutation_path,
            displacement_goal: InputDisplacementGoalPath(test_util::path::absolute_input_file(
                "image/displacement_goal/identity_displacement_goal.png",
            )),
            permutation_output_path_prefix: OutputPermutationPath(
                test_util::unique_absolute_output_file!(),
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
        InputPermutationPath::try_from_unverified_with_path_context(
            UnverifiedInputPermutationPath(test_util::path::relative_input_file(
                "image/permutation/identity_permutation.png",
            )),
            test_util::path::base_input().0,
        )?;
    let config = Config {
        algorithm: AlgorithmConfig::Swap {
            candidate_permutation: candidate_permutation_path,
            displacement_goal: InputDisplacementGoalPath(test_util::path::absolute_input_file(
                "image/image/red.png",
            )),
            permutation_output_path_prefix: OutputPermutationPath(
                test_util::unique_absolute_output_file!(),
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
    let (candidate_permutation_path, image_dimensions) =
        InputPermutationPath::try_from_unverified_with_path_context(
            UnverifiedInputPermutationPath(test_util::path::relative_input_file(
                "image/permutation/identity_permutation.png",
            )),
            test_util::path::base_input().0,
        )?;
    let config = Config {
        algorithm: AlgorithmConfig::Swap {
            candidate_permutation: candidate_permutation_path,
            displacement_goal: InputDisplacementGoalPath(test_util::path::absolute_input_file(
                "image/displacement_goal/identity_displacement_goal.png",
            )),
            permutation_output_path_prefix: OutputPermutationPath(OutputFilePath(Cow::Owned(
                test_util::path::unverified_absolute_output_path("not_found/cannot_create"),
            ))),
            parameters: make_swap_parameters(),
        },
        dispatcher: compute::Config { image_dimensions },
    };
    test_util::assert_error_contains(cli::run(config), "No such file or directory");
    Ok(())
}
