use image_annealing::compute::format::{ImageFormat, LosslessImage, Rgba16Image};
use image_annealing::compute::{self, Config, OutputStatus, PermuteInput, PermuteParameters};
use image_annealing::{CandidatePermutation, ImageDimensions};
use std::default::Default;
use std::error::Error;
use test_utils::algorithm::{assert_step_until_error, assert_step_until_success};
use test_utils::permutation::DimensionsAndPermutation;

#[test]
fn run_twice_invalid_permutation_valid() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::non_identity();
    let expected_permutation = permutation.clone();
    let original_image = test_utils::image::coordinates_to_colors(&dimensions);
    let permuted_lossless_image = LosslessImage::Rgba16(Rgba16Image::new(
        test_utils::permutation::non_identity_forward_permute(&original_image),
    )?);
    let original_lossless_image = LosslessImage::Rgba16(Rgba16Image::new(original_image)?);

    let DimensionsAndPermutation {
        permutation: invalid_permutation,
        dimensions: other_dimensions,
    } = test_utils::permutation::duplicate();
    assert_eq!(dimensions, other_dimensions);

    let mut dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(CandidatePermutation::new(invalid_permutation)?),
            original_image: Some(original_lossless_image.clone()),
        },
        &Default::default(),
    );
    assert_step_until_error(algorithm.as_mut(), OutputStatus::FinalFullOutput, "entries (x, y, delta_x, delta_y) = (0, 0, 0, 1) and (x, y, delta_x, delta_y) = (0, 2, 0, -1) both map to location (x, y) = (0, 1)");

    dispatcher = algorithm.return_to_dispatcher();
    algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(CandidatePermutation::new(permutation)?),
            original_image: Some(original_lossless_image.clone()),
        },
        &Default::default(),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;
    let output = algorithm.full_output_block().unwrap();
    assert_eq!(*output.permutation.unwrap().as_ref(), expected_permutation);
    assert_eq!(output.original_image.unwrap(), original_lossless_image);
    assert_eq!(output.permuted_image, permuted_lossless_image);
    Ok(())
}

#[test]
fn invalid_image_dimensions() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::non_identity();
    let invalid_dimensions =
        ImageDimensions::try_new(dimensions.width() + 1, dimensions.height()).unwrap();
    let image = LosslessImage::Rgba16(Rgba16Image::new(test_utils::image::coordinates_to_colors(
        &invalid_dimensions,
    ))?);

    let dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(CandidatePermutation::new(permutation)?),
            original_image: Some(image),
        },
        &Default::default(),
    );
    assert_step_until_error(
        algorithm.as_mut(),
        OutputStatus::FinalFullOutput,
        "mismatch in image dimensions, (width, height) = (2, 3) and (width, height) = (3, 3)",
    );
    Ok(())
}

#[test]
fn invalid_permutation_dimensions() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::non_identity();
    let other_dimensions =
        ImageDimensions::try_new(dimensions.width() + 1, dimensions.height()).unwrap();
    let image = LosslessImage::Rgba16(Rgba16Image::new(test_utils::image::coordinates_to_colors(
        &other_dimensions,
    ))?);

    let dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: other_dimensions,
    })?;
    let mut algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(CandidatePermutation::new(permutation)?),
            original_image: Some(image),
        },
        &Default::default(),
    );
    assert_step_until_error(
        algorithm.as_mut(),
        OutputStatus::FinalFullOutput,
        "mismatch in image dimensions, (width, height) = (3, 3) and (width, height) = (2, 3)",
    );
    Ok(())
}

#[test]
fn forget_image() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::identity();

    let dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })
    .unwrap();
    let mut algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(CandidatePermutation::new(permutation)?),
            ..Default::default()
        },
        &PermuteParameters {
            permuted_image_format: Some(ImageFormat::Rgba16),
        },
    );
    assert_step_until_error(
        algorithm.as_mut(),
        OutputStatus::FinalFullOutput,
        "an input image must be provided as there is none to reuse",
    );
    Ok(())
}

#[test]
fn forgot_format() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::identity();
    let expected_permutation = permutation.clone();
    let original_image = test_utils::image::coordinates_to_colors(&dimensions);
    let permuted_image = test_utils::permutation::identity_permute(&original_image);
    let original_lossless_image = LosslessImage::Rgba16(Rgba16Image::new(original_image)?);

    let mut dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(CandidatePermutation::new(permutation.clone())?),
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

    dispatcher = algorithm.return_to_dispatcher();

    algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(CandidatePermutation::new(permutation)?),
            ..Default::default()
        },
        &Default::default(),
    );
    assert_step_until_error(
        algorithm.as_mut(),
        OutputStatus::FinalFullOutput,
        "no image format was provided for the permuted output image",
    );

    Ok(())
}

#[test]
fn format_mismatch() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::identity();
    let original_image = test_utils::image::coordinates_to_colors(&dimensions);
    let original_lossless_image = LosslessImage::Rgba16(Rgba16Image::new(original_image)?);

    let dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(CandidatePermutation::new(permutation)?),
            original_image: Some(original_lossless_image),
        },
        &PermuteParameters {
            permuted_image_format: Some(ImageFormat::Rgba8),
        },
    );

    assert_step_until_error(
        algorithm.as_mut(),
        OutputStatus::FinalFullOutput,
        "permutation input image has format 16-bit RGBA, but the requested output format is 8-bit RGBA",
    );

    Ok(())
}
