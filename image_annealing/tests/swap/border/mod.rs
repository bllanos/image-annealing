use image_annealing::compute::{self, OutputStatus, SwapInput, SwapParameters};
use image_annealing::{CandidatePermutation, DisplacementGoal};
use std::convert::TryInto;
use std::error::Error;
use test_utils::algorithm::assert_step_until_success;
use test_utils::permutation::DimensionsAndPermutation;

fn dimensions_wxh<T>(width: T, height: T) -> Result<(), Box<dyn Error>>
where
    T: TryInto<usize> + std::fmt::Debug + std::fmt::Display + Copy,
{
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::identity_with_dimensions(width, height);
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

#[test]
fn dimensions_1x1() -> Result<(), Box<dyn Error>> {
    dimensions_wxh(1, 1)
}

#[test]
fn dimensions_1x2() -> Result<(), Box<dyn Error>> {
    dimensions_wxh(1, 2)
}

#[test]
fn dimensions_2x1() -> Result<(), Box<dyn Error>> {
    dimensions_wxh(2, 1)
}

#[test]
fn dimensions_2x2() -> Result<(), Box<dyn Error>> {
    dimensions_wxh(2, 2)
}

#[test]
fn dimensions_2x3() -> Result<(), Box<dyn Error>> {
    dimensions_wxh(2, 3)
}

#[test]
fn dimensions_3x2() -> Result<(), Box<dyn Error>> {
    dimensions_wxh(3, 2)
}

#[test]
fn dimensions_3x3() -> Result<(), Box<dyn Error>> {
    dimensions_wxh(3, 3)
}
