use image_annealing::compute::{self, Config, OutputStatus, SwapInput};
use image_annealing::{CandidatePermutation, DisplacementGoal, ImageDimensions, VectorField};
use std::default::Default;
use std::error::Error;
use test_util::algorithm::{
    assert_correct_default_swap_full_output, assert_step_until_error, assert_step_until_success,
};
use test_util::operation::{assert_correct_swap_count_output, SwapAcceptedCount};
use test_util::permutation::DimensionsAndPermutation;

#[test]
fn invalid_permutation_dimensions() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_util::permutation::non_identity();
    let displacement_goal = DisplacementGoal::identity(&dimensions);
    let other_dimensions = ImageDimensions::try_new(dimensions.width() + 1, dimensions.height())?;

    let dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: other_dimensions,
    })?;
    let mut algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation::from_vector_field(permutation)?),
            displacement_goal: Some(displacement_goal),
        },
        &test_util::algorithm::default_swap_parameters(),
    );
    assert_step_until_error(
        algorithm.as_mut(),
        OutputStatus::FinalPartialOutput,
        "mismatch in image dimensions, (width, height) = (3, 3) and (width, height) = (2, 3)",
    );
    Ok(())
}

#[test]
fn invalid_displacement_goal_dimensions() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_util::permutation::non_identity();
    let other_dimensions = ImageDimensions::try_new(dimensions.width() + 1, dimensions.height())?;
    let displacement_goal = DisplacementGoal::identity(&other_dimensions);

    let dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation::from_vector_field(permutation)?),
            displacement_goal: Some(displacement_goal),
        },
        &test_util::algorithm::default_swap_parameters(),
    );
    assert_step_until_error(
        algorithm.as_mut(),
        OutputStatus::FinalPartialOutput,
        "mismatch in image dimensions, (width, height) = (2, 3) and (width, height) = (3, 3)",
    );
    Ok(())
}

#[test]
fn forget_displacement_goal() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_util::permutation::identity();
    let dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;

    let mut algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation::from_vector_field(permutation)?),
            ..Default::default()
        },
        &test_util::algorithm::default_swap_parameters(),
    );
    assert_step_until_error(
        algorithm.as_mut(),
        OutputStatus::FinalPartialOutput,
        "an input displacement goal field must be provided as there is none to reuse",
    );
    Ok(())
}

#[test]
fn run_twice_invalid_permutation_valid() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_util::permutation::non_identity();
    let expected_permutation = test_util::operation::swap(&permutation);
    let displacement_goal =
        DisplacementGoal::from_raw_candidate_permutation(expected_permutation.clone())?;
    let expected_displacement_goal = displacement_goal.as_ref().clone();

    let DimensionsAndPermutation {
        permutation: invalid_permutation,
        dimensions: other_dimensions,
    } = test_util::permutation::duplicate();
    assert_eq!(dimensions, other_dimensions);

    let mut dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;
    let swap_parameters = test_util::algorithm::default_swap_parameters();
    let mut algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation::from_vector_field(
                invalid_permutation,
            )?),
            displacement_goal: Some(displacement_goal),
        },
        &swap_parameters,
    );
    assert_step_until_error(algorithm.as_mut(), OutputStatus::FinalPartialOutput, "entries (x, y, delta_x, delta_y) = (0, 0, 0, 1) and (x, y, delta_x, delta_y) = (0, 2, 0, -1) both map to location (x, y) = (0, 1)");

    dispatcher = algorithm.return_to_dispatcher();
    algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation::from_vector_field(
                permutation.clone(),
            )?),
            displacement_goal: Some(DisplacementGoal::from_vector_field(
                expected_displacement_goal.clone(),
            )?),
        },
        &swap_parameters,
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialOutput)?;
    assert_correct_default_swap_full_output(
        algorithm.as_mut(),
        &permutation,
        &expected_displacement_goal,
        &expected_permutation,
    );
    assert_correct_swap_count_output(
        algorithm.as_mut(),
        &swap_parameters,
        &dimensions,
        SwapAcceptedCount::All,
    );
    Ok(())
}
