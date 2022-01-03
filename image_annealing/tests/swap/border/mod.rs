use image_annealing::compute::{self, OutputStatus, SwapInput, SwapParameters};
use image_annealing::{CandidatePermutation, DisplacementGoal};
use std::convert::TryInto;
use std::error::Error;
use test_utils::algorithm::assert_step_until_success;
use test_utils::permutation::DimensionsAndPermutation;

fn dimensions_wxh<T>(width: T, height: T, accept_swap: bool) -> Result<(), Box<dyn Error>>
where
    T: TryInto<usize> + std::fmt::Debug + std::fmt::Display + Copy,
{
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::identity_with_dimensions(width, height);
    let expected_permutation = if accept_swap {
        test_utils::operation::swap(&permutation)
    } else {
        permutation.clone()
    };
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

#[test]
fn dimensions_1x1_accept_swap() -> Result<(), Box<dyn Error>> {
    dimensions_wxh(1, 1, true)
}

#[test]
fn dimensions_1x1_reject_swap() -> Result<(), Box<dyn Error>> {
    dimensions_wxh(1, 1, false)
}

#[test]
fn dimensions_1x2_accept_swap() -> Result<(), Box<dyn Error>> {
    dimensions_wxh(1, 2, true)
}

#[test]
fn dimensions_1x2_reject_swap() -> Result<(), Box<dyn Error>> {
    dimensions_wxh(1, 2, false)
}

#[test]
fn dimensions_2x1_accept_swap() -> Result<(), Box<dyn Error>> {
    dimensions_wxh(2, 1, true)
}

#[test]
fn dimensions_2x1_reject_swap() -> Result<(), Box<dyn Error>> {
    dimensions_wxh(2, 1, false)
}

#[test]
fn dimensions_2x2_accept_swap() -> Result<(), Box<dyn Error>> {
    dimensions_wxh(2, 2, true)
}

#[test]
fn dimensions_2x2_reject_swap() -> Result<(), Box<dyn Error>> {
    dimensions_wxh(2, 2, false)
}

#[test]
fn dimensions_2x3_accept_swap() -> Result<(), Box<dyn Error>> {
    dimensions_wxh(2, 3, true)
}

#[test]
fn dimensions_2x3_reject_swap() -> Result<(), Box<dyn Error>> {
    dimensions_wxh(2, 3, false)
}

#[test]
fn dimensions_3x2_accept_swap() -> Result<(), Box<dyn Error>> {
    dimensions_wxh(3, 2, true)
}

#[test]
fn dimensions_3x2_reject_swap() -> Result<(), Box<dyn Error>> {
    dimensions_wxh(3, 2, false)
}

#[test]
fn dimensions_3x3_accept_swap() -> Result<(), Box<dyn Error>> {
    dimensions_wxh(3, 3, true)
}

#[test]
fn dimensions_3x3_reject_swap() -> Result<(), Box<dyn Error>> {
    dimensions_wxh(3, 3, false)
}
