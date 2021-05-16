use image::Pixel;
use image_annealing::compute;
use image_annealing::compute::{
    OutputStatus, ValidatePermutationInput, ValidatePermutationParameters,
};
use image_annealing::image_utils::validation::test_utils::{self, DimensionsAndImage};
use image_annealing::image_utils::ImageDimensions;
use std::error::Error;

#[test]
fn run_once_identity() -> Result<(), Box<dyn Error>> {
    let DimensionsAndImage { image, dimensions } = test_utils::identity();
    let expected = image.clone();
    let dispatcher = compute::create_dispatcher(&dimensions)?;
    let mut algorithm = dispatcher.validate_permutation(
        ValidatePermutationInput {
            candidate_permutation: image,
        },
        ValidatePermutationParameters {},
    );
    let result = algorithm.step()?;
    assert_eq!(result, OutputStatus::FinalFullOutput);
    let output = algorithm.full_output().unwrap();
    assert_eq!(*output, expected);
    assert!(algorithm.partial_output().is_none());
    Ok(())
}

#[test]
fn run_twice_invalid_valid() -> Result<(), Box<dyn Error>> {
    let DimensionsAndImage { image, dimensions } = test_utils::non_identity();
    let expected = image.clone();
    let mut invalid_image = image.clone();
    invalid_image.get_pixel_mut(0, 0).channels_mut()[0] = 255;

    let mut dispatcher = compute::create_dispatcher(&dimensions)?;
    let mut algorithm = dispatcher.validate_permutation(
        ValidatePermutationInput {
            candidate_permutation: invalid_image,
        },
        ValidatePermutationParameters {},
    );
    let mut result = algorithm.step();
    result.expect_err("An invalid candidate permutation should trigger an error");
    assert!(algorithm.partial_output().is_none());
    assert!(algorithm.full_output().is_none());
    result = algorithm.step();
    result.expect_err("Attempting to repeat the validation should trigger an error");
    assert!(algorithm.partial_output().is_none());
    assert!(algorithm.full_output().is_none());

    dispatcher = algorithm.return_to_dispatcher();
    algorithm = dispatcher.validate_permutation(
        ValidatePermutationInput {
            candidate_permutation: image,
        },
        ValidatePermutationParameters {},
    );
    result = algorithm.step();
    assert_eq!(result?, OutputStatus::FinalFullOutput);
    let output = algorithm.full_output().unwrap();
    assert_eq!(*output, expected);
    assert!(algorithm.partial_output().is_none());
    Ok(())
}

#[test]
fn invalid_dimensions() -> Result<(), Box<dyn Error>> {
    let DimensionsAndImage { image, dimensions } = test_utils::non_identity();
    let invalid_dimensions =
        ImageDimensions::new(dimensions.width() + 1, dimensions.height()).unwrap();

    let dispatcher = compute::create_dispatcher(&invalid_dimensions)?;
    let mut algorithm = dispatcher.validate_permutation(
        ValidatePermutationInput {
            candidate_permutation: image,
        },
        ValidatePermutationParameters {},
    );
    let mut result = algorithm.step();
    result.expect_err("A mismatch in image dimensions should trigger an error");
    assert!(algorithm.partial_output().is_none());
    assert!(algorithm.full_output().is_none());
    result = algorithm.step();
    result.expect_err("Attempting to repeat the validation should trigger an error");
    assert!(algorithm.partial_output().is_none());
    assert!(algorithm.full_output().is_none());
    Ok(())
}
