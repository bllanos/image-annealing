use image_annealing::compute::format::{LosslessImage, Rgba16Image};
use image_annealing::compute::{self, Config, OutputStatus, PermuteInput, SwapInput};
use image_annealing::{
    CandidatePermutation, DisplacementGoal, ImageDimensions, ImageDimensionsHolder,
};
use std::error::Error;
use test_util::algorithm::{
    assert_correct_default_swap_full_output, assert_step_until_success,
    assert_step_until_success_async,
};
use test_util::operation::{assert_correct_swap_count_output, SwapAcceptedCount};
use test_util::permutation;
use test_util::permutation::DimensionsAndPermutation;

#[test]
fn run_once() -> Result<(), Box<dyn Error>> {
    let dim = ImageDimensions::try_new(3, 4)?;
    let dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dim,
    })?;
    let mut algorithm = dispatcher.create_permutation(Default::default(), &Default::default());
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;
    let output = algorithm.full_output_block().unwrap().validated_permutation;
    permutation::assert_is_identity(&output);
    assert_eq!(output.dimensions(), &dim);
    assert!(algorithm.full_output_block().is_none());
    Ok(())
}

async fn run_once_async_inner() -> Result<(), Box<dyn Error>> {
    let dim = ImageDimensions::try_new(3, 4)?;
    let dispatcher = compute::create_dispatcher(&Config {
        image_dimensions: dim,
    })
    .await?;
    let mut algorithm = dispatcher.create_permutation(Default::default(), &Default::default());
    assert_step_until_success_async(algorithm.as_mut(), OutputStatus::FinalFullOutput).await?;
    let output = algorithm.full_output().await.unwrap().validated_permutation;
    permutation::assert_is_identity(&output);
    assert_eq!(output.dimensions(), &dim);
    assert!(algorithm.full_output().await.is_none());
    Ok(())
}

#[test]
fn run_once_async() -> Result<(), Box<dyn Error>> {
    futures::executor::block_on(run_once_async_inner())
}

#[test]
fn run_twice() -> Result<(), Box<dyn Error>> {
    let dim = ImageDimensions::try_new(35, 42)?;
    let mut dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dim,
    })?;

    let mut algorithm = dispatcher.create_permutation(Default::default(), &Default::default());
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    dispatcher = algorithm.return_to_dispatcher();

    algorithm = dispatcher.create_permutation(Default::default(), &Default::default());
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap().validated_permutation;
    permutation::assert_is_identity(&output);
    Ok(())
}

#[test]
fn overwrite_permute() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_util::permutation::bit_interpretation_cases();
    let expected_permutation = permutation.clone();
    let original_image = test_util::image::coordinates_to_colors(&dimensions);
    let permuted_lossless_image = LosslessImage::Rgba16(Rgba16Image::new(
        test_util::permutation::bit_interpretation_cases_forward_permute(&original_image),
    )?);
    let original_lossless_image = LosslessImage::Rgba16(Rgba16Image::new(original_image)?);

    let mut dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(CandidatePermutation::from_vector_field(permutation)?),
            original_image: Some(original_lossless_image.clone()),
        },
        &Default::default(),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert_eq!(*output.permutation.unwrap().as_ref(), expected_permutation);
    assert_eq!(output.original_image.unwrap(), original_lossless_image);
    assert_eq!(output.permuted_image, permuted_lossless_image);

    dispatcher = algorithm.return_to_dispatcher();

    let mut algorithm = dispatcher.create_permutation(Default::default(), &Default::default());
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap().validated_permutation;
    permutation::assert_is_identity(&output);
    Ok(())
}

#[test]
fn overwrite_swap() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_util::permutation::reflect_around_center();
    let expected_permutation = test_util::operation::swap(&permutation);
    let displacement_goal =
        DisplacementGoal::from_raw_candidate_permutation(expected_permutation.clone())?;
    let expected_displacement_goal = displacement_goal.as_ref().clone();

    let mut dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;
    let swap_parameters = test_util::algorithm::default_swap_parameters();
    let mut algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation::from_vector_field(
                permutation.clone(),
            )?),
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
        algorithm.as_mut(),
        &swap_parameters,
        &dimensions,
        SwapAcceptedCount::All,
    );
    dispatcher = algorithm.return_to_dispatcher();

    let mut algorithm = dispatcher.create_permutation(Default::default(), &Default::default());
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap().validated_permutation;
    permutation::assert_is_identity(&output);
    Ok(())
}
