use image::DynamicImage;
use image_annealing::compute::{
    self, CreatePermutationInput, CreatePermutationParameters, OutputStatus, PermuteInput,
    PermuteParameters, SwapInput, SwapParameters,
};
use image_annealing::{CandidatePermutation, ImageDimensions};
use std::error::Error;
use test_utils::algorithm::assert_step_until_success;
use test_utils::permutation;
use test_utils::permutation::DimensionsAndPermutation;

#[test]
fn run_once() -> Result<(), Box<dyn Error>> {
    let dim = ImageDimensions::new(3, 4)?;
    let dispatcher = compute::create_dispatcher(&dim)?;
    let mut algorithm =
        dispatcher.create_permutation(CreatePermutationInput {}, CreatePermutationParameters {});
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;
    let output = algorithm.full_output().unwrap().validated_permutation;
    permutation::assert_is_identity(&output);
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

    let output = algorithm.full_output().unwrap().validated_permutation;
    permutation::assert_is_identity(&output);
    Ok(())
}

#[test]
fn overwrite_permute() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::bit_interpretation_cases();
    let expected_permutation = permutation.clone();
    let original_image = test_utils::image::coordinates_to_colors(&dimensions);
    let permuted_image =
        test_utils::permutation::bit_interpretation_cases_forward_permute(&original_image);
    let dynamic_original_image = DynamicImage::ImageRgba16(original_image);

    let mut dispatcher = compute::create_dispatcher(&dimensions)?;
    let mut algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(CandidatePermutation(permutation)),
            original_image: Some(dynamic_original_image.clone()),
        },
        PermuteParameters {},
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output().unwrap();
    assert_eq!(*output.permutation.unwrap().as_ref(), expected_permutation);
    assert_eq!(output.original_image.unwrap(), dynamic_original_image);
    assert_eq!(output.permuted_image, permuted_image);

    dispatcher = algorithm.return_to_dispatcher();

    let mut algorithm =
        dispatcher.create_permutation(CreatePermutationInput {}, CreatePermutationParameters {});
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output().unwrap().validated_permutation;
    permutation::assert_is_identity(&output);
    Ok(())
}

#[test]
fn overwrite_swap() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::reflect_around_center();
    let expected_permutation = test_utils::operation::swap(&permutation);

    let mut dispatcher = compute::create_dispatcher(&dimensions)?;
    let mut algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation(permutation.clone())),
        },
        SwapParameters {},
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output().unwrap();
    assert_eq!(*output.input_permutation.unwrap().as_ref(), permutation);
    assert_eq!(*output.output_permutation.as_ref(), expected_permutation);
    dispatcher = algorithm.return_to_dispatcher();

    let mut algorithm =
        dispatcher.create_permutation(CreatePermutationInput {}, CreatePermutationParameters {});
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output().unwrap().validated_permutation;
    permutation::assert_is_identity(&output);
    Ok(())
}
