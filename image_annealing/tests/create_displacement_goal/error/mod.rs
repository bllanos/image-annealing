use image_annealing::compute::format::{LosslessImage, Rgba16Image};
use image_annealing::compute::{
    self, Config, CreateDisplacementGoalInput, CreateDisplacementGoalParameters,
    CreateDisplacementGoalPipelineOperation, OutputStatus,
};
use image_annealing::{
    CandidatePermutation, DisplacementGoal, ImageDimensions, ImageDimensionsHolder,
};
use std::default::Default;
use std::error::Error;
use test_util::algorithm::{assert_step_until_error, assert_step_until_success};
use test_util::permutation::{assert_is_identity, DimensionsAndPermutation};

#[test]
fn invalid_displacement_goal_dimensions() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_util::permutation::non_identity();
    let other_dimensions = ImageDimensions::try_new(dimensions.width() + 1, dimensions.height())?;

    let dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: other_dimensions,
    })?;
    let mut algorithm = dispatcher.create_displacement_goal(
        CreateDisplacementGoalInput {
            displacement_goal: Some(DisplacementGoal::from_raw_candidate_permutation(
                permutation,
            )?),
            ..Default::default()
        },
        &Default::default(),
    );
    assert_step_until_error(
        algorithm.as_mut(),
        OutputStatus::FinalFullOutput,
        "mismatch in image dimensions, (width, height) = (3, 3) and (width, height) = (2, 3)",
    );
    Ok(())
}

#[test]
fn run_twice_invalid_permutation_valid() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_util::permutation::non_identity();
    let expected_permutation = permutation.clone();

    let DimensionsAndPermutation {
        permutation: invalid_permutation,
        dimensions: other_dimensions,
    } = test_util::permutation::duplicate();
    assert_eq!(dimensions, other_dimensions);

    let mut dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm = dispatcher.create_displacement_goal(
        CreateDisplacementGoalInput {
            candidate_permutation: Some(CandidatePermutation::from_vector_field(
                invalid_permutation,
            )?),
            ..Default::default()
        },
        &Default::default(),
    );
    assert_step_until_error(algorithm.as_mut(), OutputStatus::FinalFullOutput, "entries (x, y, delta_x, delta_y) = (0, 0, 0, 1) and (x, y, delta_x, delta_y) = (0, 2, 0, -1) both map to location (x, y) = (0, 1)");

    dispatcher = algorithm.return_to_dispatcher();
    algorithm = dispatcher.create_displacement_goal(
        CreateDisplacementGoalInput {
            candidate_permutation: Some(CandidatePermutation::from_vector_field(permutation)?),
            ..Default::default()
        },
        &Default::default(),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;
    let output = algorithm.full_output_block().unwrap();
    assert!(output.input_displacement_goal.is_none());
    assert_eq!(*output.permutation.unwrap().as_ref(), expected_permutation);
    assert!(output.image.is_none());
    assert_is_identity(&output.output_displacement_goal);
    assert_eq!(output.output_displacement_goal.dimensions(), &dimensions);
    Ok(())
}

#[test]
fn invalid_permutation_dimensions() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_util::permutation::non_identity();
    let other_dimensions = ImageDimensions::try_new(dimensions.width() + 1, dimensions.height())?;

    let dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: other_dimensions,
    })?;
    let mut algorithm = dispatcher.create_displacement_goal(
        CreateDisplacementGoalInput {
            candidate_permutation: Some(CandidatePermutation::from_vector_field(permutation)?),
            ..Default::default()
        },
        &Default::default(),
    );
    assert_step_until_error(
        algorithm.as_mut(),
        OutputStatus::FinalFullOutput,
        "mismatch in image dimensions, (width, height) = (3, 3) and (width, height) = (2, 3)",
    );
    Ok(())
}

#[test]
fn invalid_image_dimensions() -> Result<(), Box<dyn Error>> {
    let dimensions = ImageDimensions::try_new(2, 3)?;
    let invalid_dimensions =
        ImageDimensions::try_new(dimensions.width() + 1, dimensions.height()).unwrap();
    let image = LosslessImage::Rgba16(Rgba16Image::new(test_util::image::coordinates_to_colors(
        &invalid_dimensions,
    ))?);

    let dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm = dispatcher.create_displacement_goal(
        CreateDisplacementGoalInput {
            image: Some(image),
            ..Default::default()
        },
        &Default::default(),
    );
    assert_step_until_error(
        algorithm.as_mut(),
        OutputStatus::FinalFullOutput,
        "mismatch in image dimensions, (width, height) = (2, 3) and (width, height) = (3, 3)",
    );
    Ok(())
}

#[test]
fn no_pipeline() -> Result<(), Box<dyn Error>> {
    let dimensions = ImageDimensions::try_new(2, 3)?;

    let dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm = dispatcher.create_displacement_goal(
        Default::default(),
        &CreateDisplacementGoalParameters {
            pipeline_operation: CreateDisplacementGoalPipelineOperation::Preserve,
        },
    );
    assert_step_until_error(
        algorithm.as_mut(),
        OutputStatus::FinalFullOutput,
        "no displacement goal generation compute shader pipeline has been set",
    );
    Ok(())
}

#[test]
#[should_panic(expected = "expected ')', found '}'")]
fn invalid_shader() {
    let dimensions = ImageDimensions::try_new(2, 3).unwrap();

    let dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })
    .unwrap();
    let mut algorithm = dispatcher.create_displacement_goal(
        Default::default(),
        &test_util::shader::make_create_displacement_goal_parameters("parse_error"),
    );
    algorithm.step_until_finished().unwrap();
}
