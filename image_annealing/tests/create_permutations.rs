use image_annealing::compute;
use image_annealing::compute::{
    conversion, CreatePermutationInput, CreatePermutationParameters, OutputStatus,
};
use image_annealing::image_utils::ImageDimensions;
use std::error::Error;

#[test]
fn run_once() -> Result<(), Box<dyn Error>> {
    let dim = ImageDimensions::new(3, 4)?;
    let dispatcher = compute::create_dispatcher(&dim)?;
    let mut algorithm =
        dispatcher.create_permutation(CreatePermutationInput {}, CreatePermutationParameters {});
    let result = algorithm.step()?;
    assert_eq!(result, OutputStatus::FinalFullOutput);
    let output = algorithm.full_output().unwrap();
    let converted_output = conversion::as_vec(output);
    let mut expected: Vec<conversion::PermutationEntry> = Vec::with_capacity(dim.count());
    expected.resize(dim.count(), conversion::PermutationEntry(0, 0));
    assert_eq!(converted_output, expected);
    assert!(algorithm.partial_output().is_none());
    Ok(())
}

#[test]
fn run_twice() -> Result<(), Box<dyn Error>> {
    let dim = ImageDimensions::new(35, 42)?;
    let mut dispatcher = compute::create_dispatcher(&dim)?;

    let mut algorithm =
        dispatcher.create_permutation(CreatePermutationInput {}, CreatePermutationParameters {});
    let result = algorithm.step()?;
    assert_eq!(result, OutputStatus::FinalFullOutput);

    dispatcher = algorithm.return_to_dispatcher();

    algorithm =
        dispatcher.create_permutation(CreatePermutationInput {}, CreatePermutationParameters {});
    let result = algorithm.step()?;
    assert_eq!(result, OutputStatus::FinalFullOutput);

    let output = algorithm.full_output().unwrap();
    let converted_output = conversion::as_vec(output);
    let mut expected: Vec<conversion::PermutationEntry> = Vec::with_capacity(dim.count());
    expected.resize(dim.count(), conversion::PermutationEntry(0, 0));
    assert_eq!(converted_output, expected);
    assert!(algorithm.partial_output().is_none());
    Ok(())
}
