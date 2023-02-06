use image_annealing::compute::format::VectorFieldImageBuffer;
use image_annealing::compute::{
    Algorithm, OutputStatus, SwapAlgorithm, SwapFullOutput, SwapParameters, SwapPass,
};
use std::collections::HashSet;
use std::error::Error;

fn assert_output_vacancies<T: ?Sized, PartialOutput: Send, FullOutput: Send>(
    algorithm: &mut T,
    statuses: HashSet<OutputStatus>,
) where
    T: Algorithm<PartialOutput, FullOutput>,
{
    let no_full_statuses = !statuses.iter().any(OutputStatus::is_full);
    if statuses.iter().any(OutputStatus::is_partial) {
        if no_full_statuses {
            assert!(algorithm.full_output_block().is_none());
        }
    } else if no_full_statuses {
        assert!(algorithm.partial_output_block().is_none());
        assert!(algorithm.full_output_block().is_none());
    }
}

async fn assert_output_vacancies_async<T: ?Sized, PartialOutput: Send, FullOutput: Send>(
    algorithm: &mut T,
    statuses: HashSet<OutputStatus>,
) where
    T: Algorithm<PartialOutput, FullOutput>,
{
    let no_full_statuses = !statuses.iter().any(OutputStatus::is_full);
    if statuses.iter().any(OutputStatus::is_partial) {
        if no_full_statuses {
            assert!(algorithm.full_output().await.is_none());
        }
    } else if no_full_statuses {
        assert!(algorithm.partial_output().await.is_none());
        assert!(algorithm.full_output().await.is_none());
    }
}

pub fn assert_step_until_success<T: ?Sized, PartialOutput: Send, FullOutput: Send>(
    algorithm: &mut T,
    status: OutputStatus,
) -> Result<(), Box<dyn Error>>
where
    T: Algorithm<PartialOutput, FullOutput>,
{
    assert!(algorithm.partial_output_block().is_none());
    assert!(algorithm.full_output_block().is_none());
    let mut status_set = HashSet::<OutputStatus>::new();
    loop {
        let current_status = algorithm.step()?;
        status_set.insert(current_status);
        if current_status == status {
            break;
        }
    }
    assert_output_vacancies(algorithm, status_set);
    if status.is_final() {
        crate::assert_error_contains(
            algorithm.step(),
            "Algorithm::step cannot be called after the final output has been computed",
        );
    }
    Ok(())
}

pub async fn assert_step_until_success_async<T: ?Sized, PartialOutput: Send, FullOutput: Send>(
    algorithm: &mut T,
    status: OutputStatus,
) -> Result<(), Box<dyn Error>>
where
    T: Algorithm<PartialOutput, FullOutput>,
{
    assert!(algorithm.partial_output().await.is_none());
    assert!(algorithm.full_output().await.is_none());
    let mut status_set = HashSet::<OutputStatus>::new();
    loop {
        let current_status = algorithm.step()?;
        status_set.insert(current_status);
        if current_status == status {
            break;
        }
    }
    assert_output_vacancies_async(algorithm, status_set).await;
    if status.is_final() {
        crate::assert_error_contains(
            algorithm.step(),
            "Algorithm::step cannot be called after the final output has been computed",
        );
    }
    Ok(())
}

pub fn assert_step_until_error<T: ?Sized, PartialOutput: Send, FullOutput: Send>(
    algorithm: &mut T,
    status: OutputStatus,
    message: &str,
) where
    T: Algorithm<PartialOutput, FullOutput>,
{
    crate::assert_error_contains(algorithm.step_until(status), message);
    assert!(algorithm.partial_output_block().is_none());
    assert!(algorithm.full_output_block().is_none());
    crate::assert_error_contains(
        algorithm.step(),
        "cannot proceed because of an error during the previous call to Algorithm::step",
    );
    assert!(algorithm.partial_output_block().is_none());
    assert!(algorithm.full_output_block().is_none());
}

pub fn default_swap_parameters() -> SwapParameters {
    SwapParameters {
        sequence: SwapPass::Horizontal.into(),
        swap_acceptance_threshold: Default::default(),
        count_swap: true,
    }
}

fn assert_correct_default_swap_full_output_inner(
    output_option: Option<SwapFullOutput>,
    input_permutation: &VectorFieldImageBuffer,
    displacement_goal: &VectorFieldImageBuffer,
    expected_permutation: &VectorFieldImageBuffer,
) {
    let output = output_option.unwrap();
    let returned_input = output.input.as_ref().unwrap();
    assert_eq!(
        returned_input.permutation.as_ref().unwrap().as_ref(),
        input_permutation
    );
    assert_eq!(
        returned_input.displacement_goal.as_ref().unwrap().as_ref(),
        displacement_goal
    );
    assert_eq!(output.output_permutation.as_ref(), expected_permutation);
    assert_eq!(output.pass, SwapPass::Horizontal);
}

pub fn assert_correct_default_swap_full_output(
    algorithm: &mut SwapAlgorithm,
    input_permutation: &VectorFieldImageBuffer,
    displacement_goal: &VectorFieldImageBuffer,
    expected_permutation: &VectorFieldImageBuffer,
) {
    let output = algorithm.full_output_block();
    assert_correct_default_swap_full_output_inner(
        output,
        input_permutation,
        displacement_goal,
        expected_permutation,
    );
    assert!(algorithm.full_output_block().is_none());
}

pub async fn assert_correct_default_swap_full_output_async(
    algorithm: &mut SwapAlgorithm,
    input_permutation: &VectorFieldImageBuffer,
    displacement_goal: &VectorFieldImageBuffer,
    expected_permutation: &VectorFieldImageBuffer,
) {
    let output = algorithm.full_output().await;
    assert_correct_default_swap_full_output_inner(
        output,
        input_permutation,
        displacement_goal,
        expected_permutation,
    );
    assert!(algorithm.full_output().await.is_none());
}
