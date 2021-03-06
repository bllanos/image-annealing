use image_annealing::compute;
use image_annealing::compute::{
    OutputStatus, ValidatePermutationInput, ValidatePermutationParameters,
};
use image_annealing::image_utils::ImageDimensions;
use std::error::Error;
use test_utils::algorithm::{assert_step_until_error, assert_step_until_success};
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
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;
    let output = algorithm.full_output().unwrap();
    assert_eq!(*output.as_ref(), expected);
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
    assert_step_until_error(algorithm.as_mut(), OutputStatus::FinalFullOutput);

    dispatcher = algorithm.return_to_dispatcher();
    algorithm = dispatcher.validate_permutation(
        ValidatePermutationInput {
            candidate_permutation: permutation,
        },
        ValidatePermutationParameters {},
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;
    let output = algorithm.full_output().unwrap();
    assert_eq!(*output.as_ref(), expected);
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
    assert_step_until_error(algorithm.as_mut(), OutputStatus::FinalFullOutput);
    Ok(())
}
