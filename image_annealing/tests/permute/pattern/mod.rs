use image_annealing::compute::format::{LosslessImage, Rgba16Image};
use image_annealing::compute::{self, Config, OutputStatus, PermuteInput};
use image_annealing::CandidatePermutation;
use std::default::Default;
use std::error::Error;
use test_util::algorithm::{assert_step_until_success, assert_step_until_success_async};
use test_util::permutation::DimensionsAndPermutation;

#[test]
fn run_once_identity() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_util::permutation::identity();
    let expected_permutation = permutation.clone();
    let original_image = test_util::image::coordinates_to_colors(&dimensions);
    let permuted_lossless_image = LosslessImage::Rgba16(Rgba16Image::new(
        test_util::permutation::identity_permute(&original_image),
    )?);
    let original_lossless_image = LosslessImage::Rgba16(Rgba16Image::new(original_image)?);

    let dispatcher = compute::create_dispatcher_block(&Config {
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
    assert!(algorithm.full_output_block().is_none());
    Ok(())
}

async fn run_once_identity_async_inner() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_util::permutation::identity();
    let expected_permutation = permutation.clone();
    let original_image = test_util::image::coordinates_to_colors(&dimensions);
    let permuted_lossless_image = LosslessImage::Rgba16(Rgba16Image::new(
        test_util::permutation::identity_permute(&original_image),
    )?);
    let original_lossless_image = LosslessImage::Rgba16(Rgba16Image::new(original_image)?);

    let dispatcher = compute::create_dispatcher(&Config {
        image_dimensions: dimensions,
    })
    .await?;
    let mut algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(CandidatePermutation::from_vector_field(permutation)?),
            original_image: Some(original_lossless_image.clone()),
        },
        &Default::default(),
    );
    assert_step_until_success_async(algorithm.as_mut(), OutputStatus::FinalFullOutput).await?;

    let output = algorithm.full_output().await.unwrap();
    assert_eq!(*output.permutation.unwrap().as_ref(), expected_permutation);
    assert_eq!(output.original_image.unwrap(), original_lossless_image);
    assert_eq!(output.permuted_image, permuted_lossless_image);
    assert!(algorithm.full_output().await.is_none());
    Ok(())
}

#[test]
fn run_once_identity_async() -> Result<(), Box<dyn Error>> {
    futures::executor::block_on(run_once_identity_async_inner())
}

#[test]
fn bit_interpretation_cases() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_util::permutation::bit_interpretation_cases();
    let expected_permutation = permutation.clone();
    let original_image = test_util::image::coordinates_to_colors(&dimensions);
    let permuted_image =
        test_util::permutation::bit_interpretation_cases_forward_permute(&original_image);
    let original_lossless_image = LosslessImage::Rgba16(Rgba16Image::new(original_image)?);

    let dispatcher = compute::create_dispatcher_block(&Config {
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
    assert_eq!(
        output.permuted_image,
        LosslessImage::Rgba16(Rgba16Image::new(permuted_image)?)
    );
    Ok(())
}
