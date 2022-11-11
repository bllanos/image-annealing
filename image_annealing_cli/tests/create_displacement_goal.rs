use image_annealing::compute::format::{
    ImageFileReader, ImageFileWriter, ImageFormat, VectorFieldImageBuffer,
};
use image_annealing::{compute, DisplacementGoal, ImageDimensions, ImageDimensionsHolder};
use image_annealing_cli::cli;
use image_annealing_cli::config::{
    AlgorithmConfig, Config, CreateDisplacementGoalInputConfig, DisplacementGoalPath, ImagePath,
    LosslessImagePath, PermutationPath,
};
use std::error::Error;
use test_utils::permutation;

#[test]
fn create_displacement_goal_valid() -> Result<(), Box<dyn Error>> {
    let path = test_utils::make_test_output_path_string(["cli_create_displacement_goal"]);
    let full_output_path = VectorFieldImageBuffer::make_filename(&path);
    assert!(!full_output_path.is_file());

    let dimensions = ImageDimensions::try_new(3, 4)?;
    let config = Config {
        algorithm: AlgorithmConfig::CreateDisplacementGoal {
            input: Default::default(),
            displacement_goal_output_path_no_extension: DisplacementGoalPath::from_raw(path),
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
        PermutationPath::from_input_path(test_utils::make_test_data_path_string([
            "image",
            "permutation",
            "invalid_permutation.png",
        ]))?;
    let config = Config {
        algorithm: AlgorithmConfig::CreateDisplacementGoal {
            input: CreateDisplacementGoalInputConfig {
                candidate_permutation: Some(candidate_permutation_path),
                ..Default::default()
            },
            displacement_goal_output_path_no_extension: DisplacementGoalPath::from_raw(
                test_utils::make_test_output_path_string(["cli_create_displacement_goal_invalid"]),
            ),
        },
        dispatcher: compute::Config { image_dimensions },
    };
    test_utils::assert_error_contains(cli::run(config), "out of bounds mapping (x, y, delta_x, delta_y) = (3, 10, 257, 511) for an image of dimensions (width, height) = (20, 25)");
    Ok(())
}

#[test]
fn invalid_displacement_goal_format() -> Result<(), Box<dyn Error>> {
    let (displacement_goal_path, image_dimensions) =
        DisplacementGoalPath::from_input_path(test_utils::make_test_data_path_string([
            "image", "image", "red.png",
        ]))?;
    let config = Config {
        algorithm: AlgorithmConfig::CreateDisplacementGoal {
            input: CreateDisplacementGoalInputConfig {
                displacement_goal: Some(displacement_goal_path),
                ..Default::default()
            },
            displacement_goal_output_path_no_extension: DisplacementGoalPath::from_raw(
                test_utils::make_test_output_path_string([
                    "cli_create_displacement_goal_invalid_displacement_goal_format",
                ]),
            ),
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
fn invalid_permutation_format() -> Result<(), Box<dyn Error>> {
    let (candidate_permutation_path, image_dimensions) =
        PermutationPath::from_input_path(test_utils::make_test_data_path_string([
            "image", "image", "red.png",
        ]))?;
    let config = Config {
        algorithm: AlgorithmConfig::CreateDisplacementGoal {
            input: CreateDisplacementGoalInputConfig {
                candidate_permutation: Some(candidate_permutation_path),
                ..Default::default()
            },
            displacement_goal_output_path_no_extension: DisplacementGoalPath::from_raw(
                test_utils::make_test_output_path_string([
                    "cli_create_displacement_goal_invalid_permutation_format",
                ]),
            ),
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
fn invalid_image_format() -> Result<(), Box<dyn Error>> {
    let image_path = test_utils::make_test_data_path_string(["image", "image", "stripes.png"]);
    let image_dimensions = ImageDimensions::from_image_path(&image_path)?;
    let config = Config {
        algorithm: AlgorithmConfig::CreateDisplacementGoal {
            input: CreateDisplacementGoalInputConfig {
                image: Some(LosslessImagePath::Rgba16(image_path)),
                ..Default::default()
            },
            displacement_goal_output_path_no_extension: DisplacementGoalPath::from_raw(
                test_utils::make_test_output_path_string([
                    "cli_create_displacement_goal_invalid_image_format",
                ]),
            ),
        },
        dispatcher: compute::Config { image_dimensions },
    };
    test_utils::assert_error_contains(
        cli::run(config),
        &format!("not the expected format of {}", ImageFormat::Rgba16),
    );
    Ok(())
}

#[test]
fn save_missing_directory() -> Result<(), Box<dyn Error>> {
    let path = test_utils::make_test_output_path_string(["not_found", "cannot_create"]);

    let dimensions = ImageDimensions::try_new(3, 4)?;
    let config = Config {
        algorithm: AlgorithmConfig::CreateDisplacementGoal {
            input: Default::default(),
            displacement_goal_output_path_no_extension: DisplacementGoalPath::from_raw(path),
        },
        dispatcher: compute::Config {
            image_dimensions: dimensions,
        },
    };
    test_utils::assert_error_contains(cli::run(config), "No such file or directory");
    Ok(())
}
