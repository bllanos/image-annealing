use image_annealing::compute;
use image_annealing::compute::{
    conversion, CreatePermutationInput, CreatePermutationParameters, OutputStatus,
};
use image_annealing::image_utils::ImageDimensions;
use std::error::Error;
use test_utils::algorithm::assert_step_until_success;

#[test]
fn run_once() -> Result<(), Box<dyn Error>> {
    let dim = ImageDimensions::new(3, 4)?;
    let dispatcher = compute::create_dispatcher(&dim)?;
    let mut algorithm =
        dispatcher.create_permutation(CreatePermutationInput {}, CreatePermutationParameters {});
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;
    let output = algorithm.full_output().unwrap();
    let converted_output = conversion::to_vec(&output);
    let mut expected: Vec<conversion::PermutationEntry> = Vec::with_capacity(dim.count());
    expected.resize(dim.count(), conversion::PermutationEntry(0, 0));
    assert_eq!(converted_output, expected);
    Ok(())
}

#[test]
fn run_twice() -> Result<(), Box<dyn Error>> {
    let dim = ImageDimensions::new(35, 42)?;
    let mut dispatcher = compute::create_dispatcher(&dim)?;

    let mut algorithm =
        dispatcher.create_permutation(CreatePermutationInput {}, CreatePermutationParameters {});
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    dispatcher = algorithm.return_to_dispatcher();

    algorithm =
        dispatcher.create_permutation(CreatePermutationInput {}, CreatePermutationParameters {});
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output().unwrap();
    let converted_output = conversion::to_vec(&output);
    let mut expected: Vec<conversion::PermutationEntry> = Vec::with_capacity(dim.count());
    expected.resize(dim.count(), conversion::PermutationEntry(0, 0));
    assert_eq!(converted_output, expected);
    Ok(())
}
