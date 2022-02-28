use image_annealing::compute::{Algorithm, OutputStatus, SwapParameters, SwapPassSelection};
use std::error::Error;

fn assert_output_vacancies<PartialOutput, FullOutput>(
    algorithm: &mut dyn Algorithm<PartialOutput, FullOutput>,
    status: OutputStatus,
) {
    match status {
        OutputStatus::NoNewOutput => {
            assert!(algorithm.partial_output().is_none());
            assert!(algorithm.full_output().is_none());
        }
        OutputStatus::NewPartialOutput | OutputStatus::FinalPartialOutput => {
            assert!(algorithm.full_output().is_none());
        }
        OutputStatus::NewFullOutput | OutputStatus::FinalFullOutput => {
            assert!(algorithm.partial_output().is_none());
        }
        OutputStatus::NewPartialAndFullOutput | OutputStatus::FinalPartialAndFullOutput => (),
    }
}

pub fn assert_step_until_success<PartialOutput, FullOutput>(
    algorithm: &mut dyn Algorithm<PartialOutput, FullOutput>,
    status: OutputStatus,
) -> Result<(), Box<dyn Error>> {
    assert!(algorithm.partial_output().is_none());
    assert!(algorithm.full_output().is_none());
    algorithm.step_until(status)?;
    assert_output_vacancies(algorithm, status);
    match status {
        OutputStatus::NoNewOutput
        | OutputStatus::NewPartialOutput
        | OutputStatus::NewFullOutput
        | OutputStatus::NewPartialAndFullOutput => Ok(()),
        OutputStatus::FinalPartialOutput
        | OutputStatus::FinalFullOutput
        | OutputStatus::FinalPartialAndFullOutput => {
            crate::assert_error_contains(
                algorithm.step(),
                "Algorithm::step cannot be called after the final output has been computed",
            );
            Ok(())
        }
    }
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
    SwapParameters::new(SwapPassSelection::HORIZONTAL, true).unwrap()
}
