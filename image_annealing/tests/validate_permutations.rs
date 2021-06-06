use image_annealing::compute;
use image_annealing::compute::{
    OutputStatus, ValidatePermutationInput, ValidatePermutationParameters,
};
use image_annealing::image_utils::ImageDimensions;
use std::error::Error;
use test_utils::permutation::{self, DimensionsAndPermutation};

#[test]
fn run_once_identity() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = permutation::identity();
    let expected = permutation.clone();
    let dispatcher = compute::create_dispatcher(&dimensions)?;
    let mut algorithm = dispatcher.validate_permutation(
        ValidatePermutationInput {
            candidate_permutation: permutation,
        },
        ValidatePermutationParameters {},
    );
    algorithm.step_until(OutputStatus::FinalFullOutput)?;
    let output = algorithm.full_output().unwrap();
    assert_eq!(*output.data(), expected);
    assert!(algorithm.partial_output().is_none());
    Ok(())
}

#[test]
fn run_twice_invalid_valid() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = permutation::non_identity();
    let expected = permutation.clone();
    let DimensionsAndPermutation {
        permutation: invalid_image,
        dimensions: other_dimensions,
    } = permutation::duplicate();
    assert_eq!(dimensions, other_dimensions);

    let mut dispatcher = compute::create_dispatcher(&dimensions)?;
    let mut algorithm = dispatcher.validate_permutation(
        ValidatePermutationInput {
            candidate_permutation: invalid_image,
        },
        ValidatePermutationParameters {},
    );
    let mut result = algorithm.step_until(OutputStatus::FinalFullOutput);
    result.expect_err("An invalid candidate permutation should trigger an error");
    assert!(algorithm.partial_output().is_none());
    assert!(algorithm.full_output().is_none());
    result = algorithm.step_until(OutputStatus::FinalFullOutput);
    result.expect_err("Attempting to repeat the validation should trigger an error");
    assert!(algorithm.partial_output().is_none());
    assert!(algorithm.full_output().is_none());

    dispatcher = algorithm.return_to_dispatcher();
    algorithm = dispatcher.validate_permutation(
        ValidatePermutationInput {
            candidate_permutation: permutation,
        },
        ValidatePermutationParameters {},
    );
    algorithm.step_until(OutputStatus::FinalFullOutput)?;
    let output = algorithm.full_output().unwrap();
    assert_eq!(*output.data(), expected);
    assert!(algorithm.partial_output().is_none());
    Ok(())
}

#[test]
fn invalid_dimensions() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = permutation::non_identity();
    let invalid_dimensions =
        ImageDimensions::new(dimensions.width() + 1, dimensions.height()).unwrap();

    let dispatcher = compute::create_dispatcher(&invalid_dimensions)?;
    let mut algorithm = dispatcher.validate_permutation(
        ValidatePermutationInput {
            candidate_permutation: permutation,
        },
        ValidatePermutationParameters {},
    );
    let mut result = algorithm.step_until(OutputStatus::FinalFullOutput);
    result.expect_err("A mismatch in image dimensions should trigger an error");
    assert!(algorithm.partial_output().is_none());
    assert!(algorithm.full_output().is_none());
    result = algorithm.step_until(OutputStatus::FinalFullOutput);
    result.expect_err("Attempting to repeat the validation should trigger an error");
    assert!(algorithm.partial_output().is_none());
    assert!(algorithm.full_output().is_none());
    Ok(())
}
