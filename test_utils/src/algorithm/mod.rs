use image_annealing::compute::{Algorithm, OutputStatus};
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
            algorithm
                .step()
                .expect_err("Attempting to step past completion should trigger an error");
            Ok(())
        }
    }
}

pub fn assert_step_until_error<PartialOutput, FullOutput>(
    algorithm: &mut dyn Algorithm<PartialOutput, FullOutput>,
    status: OutputStatus,
) {
    algorithm.step_until(status).unwrap_err();
    assert!(algorithm.partial_output().is_none());
    assert!(algorithm.full_output().is_none());
    algorithm
        .step()
        .expect_err("Attempting to step past failure should trigger an error");
    assert!(algorithm.partial_output().is_none());
    assert!(algorithm.full_output().is_none());
}
