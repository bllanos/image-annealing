use image_annealing::compute::format::{
    ImageFileReader, ImageFileWriter, ImageFormat, VectorFieldImageBuffer,
};
use image_annealing::{compute, DisplacementGoal, ImageDimensions, ImageDimensionsHolder};
use image_annealing_cli::cli;
use image_annealing_cli::config::{
    AlgorithmConfig, Config, CreateDisplacementGoalInputConfig, InputDisplacementGoalPath,
    InputLosslessImagePath, InputPermutationPath, OutputDisplacementGoalPath,
    UnverifiedInputDisplacementGoalPath, UnverifiedInputLosslessImagePath,
    UnverifiedInputPermutationPath,
};
use image_annealing_cli_util::path::OutputFilePath;
use std::borrow::Cow;
use std::error::Error;
use test_util::permutation;

#[test]
fn create_displacement_goal_valid() -> Result<(), Box<dyn Error>> {
    let path = OutputDisplacementGoalPath(test_util::unique_absolute_output_file!());
    let full_output_path = VectorFieldImageBuffer::make_filename(&path.0 .0);
    assert!(!full_output_path.is_file());

    let dimensions = ImageDimensions::try_new(3, 4)?;
    let config = Config {
        algorithm: AlgorithmConfig::CreateDisplacementGoal {
            input: Default::default(),
            displacement_goal_output_path_no_extension: path,
        },
        dispatcher: compute::Config {
            image_dimensions: dimensions,
        },
    };
    cli::run(config)?;

    let output_displacement_goal = DisplacementGoal::load(&full_output_path)?;
    permutation::assert_is_identity(&output_displacement_goal);
    assert_eq!(output_displacement_goal.dimensions(), &dimensions);
    std::fs::remove_file(full_output_path)?;

    Ok(())
}

#[test]
fn create_displacement_goal_invalid() -> Result<(), Box<dyn Error>> {
    let (candidate_permutation_path, image_dimensions) =
        InputPermutationPath::try_from_unverified_with_path_context(
            UnverifiedInputPermutationPath(test_util::path::relative_input_file(
                "image/permutation/invalid_permutation.png",
            )),
            test_util::path::base_input().0,
        )?;
    let config = Config {
        algorithm: AlgorithmConfig::CreateDisplacementGoal {
            input: CreateDisplacementGoalInputConfig {
                candidate_permutation: Some(candidate_permutation_path),
                ..Default::default()
            },
            displacement_goal_output_path_no_extension: OutputDisplacementGoalPath(
                test_util::unique_absolute_output_file!(),
            ),
        },
        dispatcher: compute::Config { image_dimensions },
    };
    test_util::assert_error_contains(cli::run(config), "out of bounds mapping (x, y, delta_x, delta_y) = (3, 10, 257, 511) for an image of dimensions (width, height) = (20, 25)");
    Ok(())
}

#[test]
fn invalid_displacement_goal_format() -> Result<(), Box<dyn Error>> {
    let (displacement_goal_path, image_dimensions) =
        InputDisplacementGoalPath::try_from_unverified_with_path_context(
            UnverifiedInputDisplacementGoalPath(test_util::path::relative_input_file(
                "image/image/red.png",
            )),
            test_util::path::base_input().0,
        )?;
    let config = Config {
        algorithm: AlgorithmConfig::CreateDisplacementGoal {
            input: CreateDisplacementGoalInputConfig {
                displacement_goal: Some(displacement_goal_path),
                ..Default::default()
            },
            displacement_goal_output_path_no_extension: OutputDisplacementGoalPath(
                test_util::unique_absolute_output_file!(),
            ),
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
fn invalid_permutation_format() -> Result<(), Box<dyn Error>> {
    let (candidate_permutation_path, image_dimensions) =
        InputPermutationPath::try_from_unverified_with_path_context(
            UnverifiedInputPermutationPath(test_util::path::relative_input_file(
                "image/image/red.png",
            )),
            test_util::path::base_input().0,
        )?;
    let config = Config {
        algorithm: AlgorithmConfig::CreateDisplacementGoal {
            input: CreateDisplacementGoalInputConfig {
                candidate_permutation: Some(candidate_permutation_path),
                ..Default::default()
            },
            displacement_goal_output_path_no_extension: OutputDisplacementGoalPath(
                test_util::unique_absolute_output_file!(),
            ),
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
fn invalid_image_format() -> Result<(), Box<dyn Error>> {
    let (image_path, image_dimensions) =
        InputLosslessImagePath::try_from_unverified_with_path_context(
            UnverifiedInputLosslessImagePath::Rgba16(test_util::path::relative_input_file(
                "image/image/stripes.png",
            )),
            test_util::path::base_input().0,
        )?;
    let config = Config {
        algorithm: AlgorithmConfig::CreateDisplacementGoal {
            input: CreateDisplacementGoalInputConfig {
                image: Some(image_path),
                ..Default::default()
            },
            displacement_goal_output_path_no_extension: OutputDisplacementGoalPath(
                test_util::unique_absolute_output_file!(),
            ),
        },
        dispatcher: compute::Config { image_dimensions },
    };
    test_util::assert_error_contains(
        cli::run(config),
        &format!("not the expected format of {}", ImageFormat::Rgba16),
    );
    Ok(())
}

#[test]
fn save_missing_directory() -> Result<(), Box<dyn Error>> {
    let dimensions = ImageDimensions::try_new(3, 4)?;
    let config = Config {
        algorithm: AlgorithmConfig::CreateDisplacementGoal {
            input: Default::default(),
            displacement_goal_output_path_no_extension: OutputDisplacementGoalPath(OutputFilePath(
                Cow::Owned(test_util::path::unverified_absolute_output_path(
                    "not_found/cannot_create",
                )),
            )),
        },
        dispatcher: compute::Config {
            image_dimensions: dimensions,
        },
    };
    test_util::assert_error_contains(cli::run(config), "No such file or directory");
    Ok(())
}
