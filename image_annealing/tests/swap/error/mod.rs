use image_annealing::compute::{self, OutputStatus, SwapInput};
use image_annealing::{CandidatePermutation, DisplacementGoal, ImageDimensions};
use std::default::Default;
use std::error::Error;
use test_utils::algorithm::{assert_step_until_error, assert_step_until_success};
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

    let dispatcher = compute::create_dispatcher(&other_dimensions)?;
    let mut algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation(permutation)),
            displacement_goal: Some(displacement_goal),
        },
        test_utils::algorithm::default_swap_parameters(),
    );
    assert_step_until_error(
        algorithm.as_mut(),
        OutputStatus::FinalPartialAndFullOutput,
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

    let dispatcher = compute::create_dispatcher(&dimensions)?;
    let mut algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation(permutation)),
            displacement_goal: Some(displacement_goal),
        },
        test_utils::algorithm::default_swap_parameters(),
    );
    assert_step_until_error(
        algorithm.as_mut(),
        OutputStatus::FinalPartialAndFullOutput,
        "mismatch in image dimensions, (width, height) = (2, 3) and (width, height) = (3, 3)",
    );
    Ok(())
}

#[test]
fn forget_permutation() -> Result<(), Box<dyn Error>> {
    let dimensions = ImageDimensions::new(3, 4)?;
    let displacement_goal = test_utils::displacement_goal::identity(&dimensions);
    let dispatcher = compute::create_dispatcher(&dimensions)?;

    let mut algorithm = dispatcher.swap(
        SwapInput {
            displacement_goal: Some(displacement_goal),
            ..Default::default()
        },
        test_utils::algorithm::default_swap_parameters(),
    );
    assert_step_until_error(
        algorithm.as_mut(),
        OutputStatus::FinalPartialAndFullOutput,
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
    let dispatcher = compute::create_dispatcher(&dimensions)?;

    let mut algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation(permutation)),
            ..Default::default()
        },
        test_utils::algorithm::default_swap_parameters(),
    );
    assert_step_until_error(
        algorithm.as_mut(),
        OutputStatus::FinalPartialAndFullOutput,
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
    let displacement_goal = DisplacementGoal::from_candidate_permutation(CandidatePermutation(
        expected_permutation.clone(),
    ))?;
    let expected_displacement_goal = displacement_goal.as_ref().clone();

    let DimensionsAndPermutation {
        permutation: invalid_permutation,
        dimensions: other_dimensions,
    } = test_utils::permutation::duplicate();
    assert_eq!(dimensions, other_dimensions);

    let mut dispatcher = compute::create_dispatcher(&dimensions)?;
    let swap_parameters = test_utils::algorithm::default_swap_parameters();
    let mut algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation(invalid_permutation)),
            displacement_goal: Some(displacement_goal),
        },
        swap_parameters.clone(),
    );
    assert_step_until_error(algorithm.as_mut(), OutputStatus::FinalPartialAndFullOutput, "entries (x, y, delta_x, delta_y) = (0, 0, 0, 1) and (x, y, delta_x, delta_y) = (0, 2, 0, -1) both map to location (x, y) = (0, 1)");

    dispatcher = algorithm.return_to_dispatcher();
    algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation(permutation.clone())),
            displacement_goal: Some(DisplacementGoal::from_vector_field(
                expected_displacement_goal.clone(),
            )),
        },
        swap_parameters.clone(),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialAndFullOutput)?;
    let output = algorithm.full_output().unwrap();
    assert_eq!(*output.input_permutation.unwrap().as_ref(), permutation);
    assert_eq!(
        *output.input_displacement_goal.unwrap().as_ref(),
        expected_displacement_goal
    );
    assert_eq!(*output.output_permutation.as_ref(), expected_permutation);
    assert_correct_swap_count_output(
        algorithm.partial_output(),
        &swap_parameters,
        &dimensions,
        SwapAcceptedCount::All,
    );
    Ok(())
}
