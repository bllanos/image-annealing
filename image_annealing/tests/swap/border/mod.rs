use image_annealing::compute::{self, Config, OutputStatus, SwapInput};
use image_annealing::{CandidatePermutation, DisplacementGoal};
use std::error::Error;
use test_utils::algorithm::{assert_correct_default_swap_full_output, assert_step_until_success};
use test_utils::operation::{assert_correct_swap_count_output, SwapAcceptedCount};
use test_utils::permutation::DimensionsAndPermutation;

pub fn dimensions_wxh<T>(width: T, height: T, accept_swap: bool) -> Result<(), Box<dyn Error>>
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
    let displacement_goal =
        DisplacementGoal::from_raw_candidate_permutation(expected_permutation.clone())?;
    let expected_displacement_goal = displacement_goal.as_ref().clone();

    let dispatcher = compute::create_dispatcher(&Config {
        image_dimensions: dimensions,
    })?;
    let swap_parameters = test_utils::algorithm::default_swap_parameters();
    let mut algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation::new(permutation.clone())?),
            displacement_goal: Some(displacement_goal),
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
        if accept_swap && expected_permutation != permutation {
            SwapAcceptedCount::All
        } else {
            SwapAcceptedCount::None
        },
    );
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
