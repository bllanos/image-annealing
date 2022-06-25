use image_annealing::compute::{self, Config, OutputStatus, SwapInput, SwapParameters, SwapPass};
use image_annealing::{CandidatePermutation, DisplacementGoal};
use std::error::Error;
use test_utils::algorithm::{assert_correct_default_swap_full_output, assert_step_until_success};
use test_utils::operation::{assert_correct_swap_count_output, SwapAcceptedCount};
use test_utils::permutation::DimensionsAndPermutation;

mod sum_at_least_two_elements_per_invocation {
    use super::super::border;
    use image_annealing::compute::SwapPass;
    use image_annealing_shaders::WorkgroupDimensions;
    use std::error::Error;

    fn at_least_two_elements_per_invocation() -> usize {
        let workgroup_dimensions = WorkgroupDimensions::count_swap();
        (workgroup_dimensions
            .invocation_count()
            .checked_mul(SwapPass::STRIDE.try_into().unwrap())
            .unwrap()
            .checked_mul(3)
            .unwrap()
            .checked_sub(1)
            .unwrap())
        .try_into()
        .unwrap()
    }

    #[test]
    fn long_accept_swap() -> Result<(), Box<dyn Error>> {
        border::dimensions_wxh(
            at_least_two_elements_per_invocation(),
            SwapPass::STRIDE,
            true,
        )
    }

    #[test]
    fn long_reject_swap() -> Result<(), Box<dyn Error>> {
        border::dimensions_wxh(
            at_least_two_elements_per_invocation(),
            SwapPass::STRIDE,
            false,
        )
    }

    #[test]
    fn tall_accept_swap() -> Result<(), Box<dyn Error>> {
        border::dimensions_wxh(
            SwapPass::STRIDE,
            at_least_two_elements_per_invocation(),
            true,
        )
    }

    #[test]
    fn tall_reject_swap() -> Result<(), Box<dyn Error>> {
        border::dimensions_wxh(
            SwapPass::STRIDE,
            at_least_two_elements_per_invocation(),
            false,
        )
    }
}

#[test]
fn skip_count_swap() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::reflect_around_center();
    let expected_permutation = test_utils::operation::swap(&permutation);
    let displacement_goal =
        DisplacementGoal::from_raw_candidate_permutation(expected_permutation.clone())?;
    let expected_displacement_goal = displacement_goal.as_ref().clone();

    let dispatcher = compute::create_dispatcher(&Config {
        image_dimensions: dimensions,
    })?;
    let swap_parameters = SwapParameters::from_sequence(SwapPass::Horizontal.into());
    let mut algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation::new(permutation.clone())?),
            displacement_goal: Some(displacement_goal),
        },
        &swap_parameters,
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

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
