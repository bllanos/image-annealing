use image::DynamicImage;
use image_annealing::compute;
use image_annealing::compute::{OutputStatus, PermuteInput, PermuteParameters};
use image_annealing::image_utils::ImageDimensions;
use std::error::Error;
use test_utils::permutation::DimensionsAndPermutation;

#[test]
fn run_once_identity() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::identity();
    let expected_permutation = permutation.clone();
    let original_image = test_utils::image::coordinates_as_colors(&dimensions);
    let permuted_image = test_utils::permutation::identity_permute(&original_image);
    let dynamic_original_image = DynamicImage::ImageRgba16(original_image);

    let dispatcher = compute::create_dispatcher(&dimensions)?;
    let mut algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(permutation),
            original_image: Some(dynamic_original_image.clone()),
        },
        PermuteParameters {},
    );
    algorithm.step_until(OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output().unwrap();
    assert_eq!(*(output.permutation.unwrap()), expected_permutation);
    assert_eq!(output.original_image.unwrap(), dynamic_original_image);
    assert_eq!(output.permuted_image, permuted_image);
    assert!(algorithm.partial_output().is_none());
    Ok(())
}

#[test]
fn run_twice_invalid_permutation_valid() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::non_identity();
    let expected_permutation = permutation.clone();
    let original_image = test_utils::image::coordinates_as_colors(&dimensions);
    let permuted_image = test_utils::permutation::non_identity_forward_permute(&original_image);
    let dynamic_original_image = DynamicImage::ImageRgba16(original_image);

    let DimensionsAndPermutation {
        permutation: invalid_permutation,
        dimensions: other_dimensions,
    } = test_utils::permutation::duplicate();
    assert_eq!(dimensions, other_dimensions);

    let mut dispatcher = compute::create_dispatcher(&dimensions)?;
    let mut algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(invalid_permutation),
            original_image: Some(dynamic_original_image.clone()),
        },
        PermuteParameters {},
    );
    let mut result = algorithm.step_until(OutputStatus::FinalFullOutput);
    result.expect_err("An invalid candidate permutation should trigger an error");
    assert!(algorithm.partial_output().is_none());
    assert!(algorithm.full_output().is_none());
    result = algorithm.step_until(OutputStatus::FinalFullOutput);
    result.expect_err("Attempting to repeat the failed operation should trigger an error");
    assert!(algorithm.partial_output().is_none());
    assert!(algorithm.full_output().is_none());

    dispatcher = algorithm.return_to_dispatcher();
    algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(permutation),
            original_image: Some(dynamic_original_image.clone()),
        },
        PermuteParameters {},
    );
    algorithm.step_until(OutputStatus::FinalFullOutput)?;
    let output = algorithm.full_output().unwrap();
    assert_eq!(*(output.permutation.unwrap()), expected_permutation);
    assert_eq!(output.original_image.unwrap(), dynamic_original_image);
    assert_eq!(output.permuted_image, permuted_image);
    assert!(algorithm.partial_output().is_none());
    Ok(())
}

#[test]
fn invalid_image_dimensions() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::non_identity();
    let invalid_dimensions =
        ImageDimensions::new(dimensions.width() + 1, dimensions.height()).unwrap();
    let image = DynamicImage::ImageRgba16(test_utils::image::coordinates_as_colors(
        &invalid_dimensions,
    ));

    let dispatcher = compute::create_dispatcher(&dimensions)?;
    let mut algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(permutation),
            original_image: Some(image),
        },
        PermuteParameters {},
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
