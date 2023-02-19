use image_annealing::compute::{self, Config, CreateDisplacementGoalInput, OutputStatus};
use image_annealing::{CandidatePermutation, DisplacementGoal};
use std::default::Default;
use std::error::Error;
use test_util::algorithm::assert_step_until_success;
use test_util::image::DimensionsAndRgba8Buffer;
use test_util::permutation::DimensionsAndPermutation;

#[test]
fn copy_displacement_goal() -> Result<(), Box<dyn Error>> {
    let DimensionsAndRgba8Buffer { image, dimensions } = test_util::image::nonzero_rgba8_colors();
    let displacement_goal = DisplacementGoal::from_vector_field(image.clone())?;
    let expected_output_displacement_goal = displacement_goal.clone();

    let dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm = dispatcher.create_displacement_goal(
        CreateDisplacementGoalInput {
            displacement_goal: Some(displacement_goal),
            ..Default::default()
        },
        &test_util::shader::make_create_displacement_goal_parameters("copy_displacement_goal"),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert_eq!(
        output.input_displacement_goal.as_ref().unwrap(),
        &expected_output_displacement_goal
    );
    assert!(output.permutation.is_none());
    assert!(output.image.is_none());
    assert_eq!(
        &output.output_displacement_goal,
        &expected_output_displacement_goal
    );
    assert!(algorithm.full_output_block().is_none());
    Ok(())
}

#[test]
fn copy_permutation() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_util::permutation::non_identity();
    let expected_output_permutation = permutation.clone();

    let dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm = dispatcher.create_displacement_goal(
        CreateDisplacementGoalInput {
            candidate_permutation: Some(CandidatePermutation::from_vector_field(permutation)?),
            ..Default::default()
        },
        &test_util::shader::make_create_displacement_goal_parameters("copy_permutation"),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert!(output.input_displacement_goal.is_none());
    assert_eq!(
        output.permutation.as_ref().unwrap(),
        &expected_output_permutation
    );
    assert!(output.image.is_none());
    assert_eq!(
        &output.output_displacement_goal,
        &expected_output_permutation
    );
    assert!(algorithm.full_output_block().is_none());
    Ok(())
}
