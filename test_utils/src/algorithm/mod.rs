use image_annealing::compute::format::VectorFieldImageBuffer;
use image_annealing::compute::{Algorithm, OutputStatus, SwapAlgorithm, SwapParameters, SwapPass};
use std::collections::HashSet;
use std::error::Error;

fn assert_output_vacancies<PartialOutput, FullOutput>(
    algorithm: &mut dyn Algorithm<PartialOutput, FullOutput>,
    statuses: HashSet<OutputStatus>,
) {
    let no_full_statuses = !statuses.iter().any(OutputStatus::is_full);
    if statuses.iter().any(OutputStatus::is_partial) {
        if no_full_statuses {
            assert!(algorithm.full_output().is_none());
        }
    } else if no_full_statuses {
        assert!(algorithm.partial_output().is_none());
        assert!(algorithm.full_output().is_none());
    }
}

pub fn assert_step_until_success<PartialOutput, FullOutput>(
    algorithm: &mut dyn Algorithm<PartialOutput, FullOutput>,
    status: OutputStatus,
) -> Result<(), Box<dyn Error>> {
    assert!(algorithm.partial_output().is_none());
    assert!(algorithm.full_output().is_none());
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

pub fn assert_step_until_error<PartialOutput, FullOutput>(
    algorithm: &mut dyn Algorithm<PartialOutput, FullOutput>,
    status: OutputStatus,
    message: &str,
) {
    crate::assert_error_contains(algorithm.step_until(status), message);
    assert!(algorithm.partial_output().is_none());
    assert!(algorithm.full_output().is_none());
    crate::assert_error_contains(
        algorithm.step(),
        "cannot proceed because of an error during the previous call to Algorithm::step",
    );
    assert!(algorithm.partial_output().is_none());
    assert!(algorithm.full_output().is_none());
}

pub fn default_swap_parameters() -> SwapParameters {
    SwapParameters {
        sequence: SwapPass::Horizontal.into(),
        swap_acceptance_threshold: Default::default(),
        count_swap: true,
    }
}

pub fn assert_correct_default_swap_full_output(
    algorithm: &mut SwapAlgorithm,
    input_permutation: &VectorFieldImageBuffer,
    displacement_goal: &VectorFieldImageBuffer,
    expected_permutation: &VectorFieldImageBuffer,
) {
    let mut output = algorithm.full_output().unwrap();
    let returned_input = output.input.as_mut().unwrap();
    assert_eq!(
        returned_input.permutation.as_mut().unwrap().as_ref(),
        input_permutation
    );
    assert_eq!(
        returned_input.displacement_goal.as_mut().unwrap().as_ref(),
        displacement_goal
    );
    assert_eq!(output.output_permutation.as_ref(), expected_permutation);
    assert_eq!(output.pass, SwapPass::Horizontal);
    assert!(algorithm.full_output().is_none());
}
