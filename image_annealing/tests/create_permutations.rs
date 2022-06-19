use image_annealing::compute::format::{LosslessImage, Rgba16Image};
use image_annealing::compute::{
    self, Config, CreatePermutationInput, CreatePermutationParameters, OutputStatus, PermuteInput,
    SwapInput,
};
use image_annealing::{CandidatePermutation, DisplacementGoal, ImageDimensions};
use std::error::Error;
use test_utils::algorithm::{assert_correct_default_swap_full_output, assert_step_until_success};
use test_utils::operation::{assert_correct_swap_count_output, SwapAcceptedCount};
use test_utils::permutation;
use test_utils::permutation::DimensionsAndPermutation;

#[test]
fn run_once() -> Result<(), Box<dyn Error>> {
    let dim = ImageDimensions::new(3, 4)?;
    let dispatcher = compute::create_dispatcher(&Config {
        image_dimensions: dim,
    })?;
    let mut algorithm =
        dispatcher.create_permutation(CreatePermutationInput {}, &CreatePermutationParameters {});
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;
    let output = algorithm.full_output().unwrap().validated_permutation;
    permutation::assert_is_identity(&output);
    assert!(algorithm.full_output().is_none());
    Ok(())
}

#[test]
fn run_twice() -> Result<(), Box<dyn Error>> {
    let dim = ImageDimensions::new(35, 42)?;
    let mut dispatcher = compute::create_dispatcher(&Config {
        image_dimensions: dim,
    })?;

    let mut algorithm =
        dispatcher.create_permutation(CreatePermutationInput {}, &CreatePermutationParameters {});
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    dispatcher = algorithm.return_to_dispatcher();

    algorithm =
        dispatcher.create_permutation(CreatePermutationInput {}, &CreatePermutationParameters {});
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
    let permuted_lossless_image = LosslessImage::Rgba16(Rgba16Image::new(
        test_utils::permutation::bit_interpretation_cases_forward_permute(&original_image),
    )?);
    let original_lossless_image = LosslessImage::Rgba16(Rgba16Image::new(original_image)?);

    let mut dispatcher = compute::create_dispatcher(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(CandidatePermutation::new(permutation)?),
            original_image: Some(original_lossless_image.clone()),
        },
        &Default::default(),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output().unwrap();
    assert_eq!(*output.permutation.unwrap().as_ref(), expected_permutation);
    assert_eq!(output.original_image.unwrap(), original_lossless_image);
    assert_eq!(output.permuted_image, permuted_lossless_image);

    dispatcher = algorithm.return_to_dispatcher();

    let mut algorithm =
        dispatcher.create_permutation(CreatePermutationInput {}, &CreatePermutationParameters {});
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
    let displacement_goal =
        DisplacementGoal::from_raw_candidate_permutation(expected_permutation.clone())?;
    let expected_displacement_goal = displacement_goal.as_ref().clone();

    let mut dispatcher = compute::create_dispatcher(&Config {
        image_dimensions: dimensions,
    })?;
    let swap_parameters = test_utils::algorithm::default_swap_parameters();
    let mut algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation::new(permutation.clone())?),
            displacement_goal: Some(displacement_goal),
        },
        &swap_parameters,
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialOutput)?;

    assert_correct_default_swap_full_output(
        algorithm.as_mut(),
        &permutation,
        &expected_displacement_goal,
        &expected_permutation,
    );
    assert_correct_swap_count_output(
        algorithm.partial_output(),
        &swap_parameters,
        &dimensions,
        SwapAcceptedCount::All,
    );
    dispatcher = algorithm.return_to_dispatcher();

    let mut algorithm =
        dispatcher.create_permutation(CreatePermutationInput {}, &CreatePermutationParameters {});
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output().unwrap().validated_permutation;
    permutation::assert_is_identity(&output);
    Ok(())
}
