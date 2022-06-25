use image_annealing::compute::{self, Config, OutputStatus, SwapInput};
use image_annealing::{CandidatePermutation, DisplacementGoal, ImageDimensions};
use std::default::Default;
use std::error::Error;
use test_utils::algorithm::{
    assert_correct_default_swap_full_output, assert_step_until_error, assert_step_until_success,
};
use test_utils::operation::{assert_correct_swap_count_output, SwapAcceptedCount};
use test_utils::permutation::DimensionsAndPermutation;

#[test]
fn invalid_permutation_dimensions() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::non_identity();
    let displacement_goal = test_utils::displacement_goal::identity(&dimensions);
    let other_dimensions = ImageDimensions::new(dimensions.width() + 1, dimensions.height())?;

    let dispatcher = compute::create_dispatcher(&Config {
        image_dimensions: other_dimensions,
    })?;
    let mut algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation::new(permutation)?),
            displacement_goal: Some(displacement_goal),
        },
        &test_utils::algorithm::default_swap_parameters(),
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
    } = test_utils::permutation::non_identity();
    let other_dimensions = ImageDimensions::new(dimensions.width() + 1, dimensions.height())?;
    let displacement_goal = test_utils::displacement_goal::identity(&other_dimensions);

    let dispatcher = compute::create_dispatcher(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation::new(permutation)?),
            displacement_goal: Some(displacement_goal),
        },
        &test_utils::algorithm::default_swap_parameters(),
    );
    assert_step_until_error(
        algorithm.as_mut(),
        OutputStatus::FinalPartialOutput,
        "mismatch in image dimensions, (width, height) = (2, 3) and (width, height) = (3, 3)",
    );
    Ok(())
}

#[test]
fn forget_permutation() -> Result<(), Box<dyn Error>> {
    let dimensions = ImageDimensions::new(3, 4)?;
    let displacement_goal = test_utils::displacement_goal::identity(&dimensions);
    let dispatcher = compute::create_dispatcher(&Config {
        image_dimensions: dimensions,
    })?;

    let mut algorithm = dispatcher.swap(
        SwapInput {
            displacement_goal: Some(displacement_goal),
            ..Default::default()
        },
        &test_utils::algorithm::default_swap_parameters(),
    );
    assert_step_until_error(
        algorithm.as_mut(),
        OutputStatus::FinalPartialOutput,
        "an input permutation must be provided as there is none to reuse",
    );
    Ok(())
}

#[test]
fn forget_displacement_goal() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::identity();
    let dispatcher = compute::create_dispatcher(&Config {
        image_dimensions: dimensions,
    })?;

    let mut algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation::new(permutation)?),
            ..Default::default()
        },
        &test_utils::algorithm::default_swap_parameters(),
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
    } = test_utils::permutation::non_identity();
    let expected_permutation = test_utils::operation::swap(&permutation);
    let displacement_goal =
        DisplacementGoal::from_raw_candidate_permutation(expected_permutation.clone())?;
    let expected_displacement_goal = displacement_goal.as_ref().clone();

    let DimensionsAndPermutation {
        permutation: invalid_permutation,
        dimensions: other_dimensions,
    } = test_utils::permutation::duplicate();
    assert_eq!(dimensions, other_dimensions);

    let mut dispatcher = compute::create_dispatcher(&Config {
        image_dimensions: dimensions,
    })?;
    let swap_parameters = test_utils::algorithm::default_swap_parameters();
    let mut algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation::new(invalid_permutation)?),
            displacement_goal: Some(displacement_goal),
        },
        &swap_parameters,
    );
    assert_step_until_error(algorithm.as_mut(), OutputStatus::FinalPartialOutput, "entries (x, y, delta_x, delta_y) = (0, 0, 0, 1) and (x, y, delta_x, delta_y) = (0, 2, 0, -1) both map to location (x, y) = (0, 1)");

    dispatcher = algorithm.return_to_dispatcher();
    algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation::new(permutation.clone())?),
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
