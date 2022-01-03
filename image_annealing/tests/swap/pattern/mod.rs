use image_annealing::compute::{self, OutputStatus, SwapInput, SwapParameters};
use image_annealing::{CandidatePermutation, DisplacementGoal};
use std::error::Error;
use test_utils::algorithm::assert_step_until_success;
use test_utils::permutation::DimensionsAndPermutation;

#[test]
fn run_once_identity() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::identity();
    let expected_permutation = test_utils::operation::swap(&permutation);
    let displacement_goal = DisplacementGoal::from_candidate_permutation(CandidatePermutation(
        expected_permutation.clone(),
    ))?;
    let expected_displacement_goal = displacement_goal.as_ref().clone();

    let dispatcher = compute::create_dispatcher(&dimensions)?;
    let mut algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation(permutation.clone())),
            displacement_goal: Some(displacement_goal),
        },
        SwapParameters {},
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output().unwrap();
    assert_eq!(*output.input_permutation.unwrap().as_ref(), permutation);
    assert_eq!(
        *output.input_displacement_goal.unwrap().as_ref(),
        expected_displacement_goal
    );
    assert_eq!(*output.output_permutation.as_ref(), expected_permutation);
    assert!(algorithm.full_output().is_none());
    Ok(())
}

#[test]
fn reflect_around_center() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::reflect_around_center();
    let expected_permutation = test_utils::operation::swap(&permutation);
    let displacement_goal = DisplacementGoal::from_candidate_permutation(CandidatePermutation(
        expected_permutation.clone(),
    ))?;
    let expected_displacement_goal = displacement_goal.as_ref().clone();

    let dispatcher = compute::create_dispatcher(&dimensions)?;
    let mut algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation(permutation.clone())),
            displacement_goal: Some(displacement_goal),
        },
        SwapParameters {},
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output().unwrap();
    assert_eq!(*output.input_permutation.unwrap().as_ref(), permutation);
    assert_eq!(
        *output.input_displacement_goal.unwrap().as_ref(),
        expected_displacement_goal
    );
    assert_eq!(*output.output_permutation.as_ref(), expected_permutation);
    Ok(())
}
