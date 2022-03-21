use image_annealing::compute::format::{ImageFormat, LosslessImage, Rgba16Image};
use image_annealing::compute::{
    self, Config, CreatePermutationInput, CreatePermutationParameters, OutputStatus, PermuteInput,
    PermuteParameters, SwapInput,
};
use image_annealing::{CandidatePermutation, DisplacementGoal, ImageDimensions};
use std::default::Default;
use std::error::Error;
use test_utils::algorithm::{assert_step_until_error, assert_step_until_success};
use test_utils::operation::{assert_correct_swap_count_output, SwapAcceptedCount};
use test_utils::permutation::DimensionsAndPermutation;

#[test]
fn run_once_identity() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::identity();
    let expected_permutation = permutation.clone();
    let original_image = test_utils::image::coordinates_to_colors(&dimensions);
    let permuted_lossless_image = LosslessImage::Rgba16(Rgba16Image::new(
        test_utils::permutation::identity_permute(&original_image),
    )?);
    let original_lossless_image = LosslessImage::Rgba16(Rgba16Image::new(original_image)?);

    let dispatcher = compute::create_dispatcher(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(CandidatePermutation::new(permutation)?),
            original_image: Some(original_lossless_image.clone()),
        },
        Default::default(),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output().unwrap();
    assert_eq!(*output.permutation.unwrap().as_ref(), expected_permutation);
    assert_eq!(output.original_image.unwrap(), original_lossless_image);
    assert_eq!(output.permuted_image, permuted_lossless_image);
    assert!(algorithm.full_output().is_none());
    Ok(())
}

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

    let mut dispatcher = compute::create_dispatcher(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(CandidatePermutation::new(invalid_permutation)?),
            original_image: Some(original_lossless_image.clone()),
        },
        Default::default(),
    );
    assert_step_until_error(algorithm.as_mut(), OutputStatus::FinalFullOutput, "entries (x, y, delta_x, delta_y) = (0, 0, 0, 1) and (x, y, delta_x, delta_y) = (0, 2, 0, -1) both map to location (x, y) = (0, 1)");

    dispatcher = algorithm.return_to_dispatcher();
    algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(CandidatePermutation::new(permutation)?),
            original_image: Some(original_lossless_image.clone()),
        },
        Default::default(),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;
    let output = algorithm.full_output().unwrap();
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
        ImageDimensions::new(dimensions.width() + 1, dimensions.height()).unwrap();
    let image = LosslessImage::Rgba16(Rgba16Image::new(test_utils::image::coordinates_to_colors(
        &invalid_dimensions,
    ))?);

    let dispatcher = compute::create_dispatcher(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(CandidatePermutation::new(permutation)?),
            original_image: Some(image),
        },
        Default::default(),
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
        ImageDimensions::new(dimensions.width() + 1, dimensions.height()).unwrap();
    let image = LosslessImage::Rgba16(Rgba16Image::new(test_utils::image::coordinates_to_colors(
        &other_dimensions,
    ))?);

    let dispatcher = compute::create_dispatcher(&Config {
        image_dimensions: other_dimensions,
    })?;
    let mut algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(CandidatePermutation::new(permutation)?),
            original_image: Some(image),
        },
        Default::default(),
    );
    assert_step_until_error(
        algorithm.as_mut(),
        OutputStatus::FinalFullOutput,
        "mismatch in image dimensions, (width, height) = (3, 3) and (width, height) = (2, 3)",
    );
    Ok(())
}

#[test]
fn bit_interpretation_cases() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::bit_interpretation_cases();
    let expected_permutation = permutation.clone();
    let original_image = test_utils::image::coordinates_to_colors(&dimensions);
    let permuted_image =
        test_utils::permutation::bit_interpretation_cases_forward_permute(&original_image);
    let original_lossless_image = LosslessImage::Rgba16(Rgba16Image::new(original_image)?);

    let dispatcher = compute::create_dispatcher(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(CandidatePermutation::new(permutation)?),
            original_image: Some(original_lossless_image.clone()),
        },
        Default::default(),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output().unwrap();
    assert_eq!(*output.permutation.unwrap().as_ref(), expected_permutation);
    assert_eq!(output.original_image.unwrap(), original_lossless_image);
    assert_eq!(
        output.permuted_image,
        LosslessImage::Rgba16(Rgba16Image::new(permuted_image)?)
    );
    Ok(())
}

#[test]
fn create_identity_permutation() -> Result<(), Box<dyn Error>> {
    let dimensions = ImageDimensions::new(3, 4)?;
    let mut dispatcher = compute::create_dispatcher(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm =
        dispatcher.create_permutation(CreatePermutationInput {}, CreatePermutationParameters {});
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;
    dispatcher = algorithm.return_to_dispatcher();

    let original_image = test_utils::image::coordinates_to_colors(&dimensions);
    let permuted_image = test_utils::permutation::identity_permute(&original_image);
    let original_lossless_image = LosslessImage::Rgba16(Rgba16Image::new(original_image)?);

    let mut algorithm = dispatcher.permute(
        PermuteInput {
            original_image: Some(original_lossless_image.clone()),
            ..Default::default()
        },
        Default::default(),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output().unwrap();
    assert!(output.permutation.is_none());
    assert_eq!(output.original_image.unwrap(), original_lossless_image);
    assert_eq!(
        output.permuted_image,
        LosslessImage::Rgba16(Rgba16Image::new(permuted_image)?)
    );
    Ok(())
}

#[test]
fn reuse_swap_permutation() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::non_identity();
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
        swap_parameters.clone(),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialAndFullOutput)?;

    let output = algorithm.full_output().unwrap();
    assert_eq!(*output.input_permutation.unwrap().as_ref(), permutation);
    assert_eq!(
        *output.input_displacement_goal.unwrap().as_ref(),
        expected_displacement_goal
    );
    assert_eq!(*output.output_permutation.as_ref(), expected_permutation);
    assert_correct_swap_count_output(
        algorithm.partial_output(),
        &swap_parameters,
        &dimensions,
        SwapAcceptedCount::All,
    );
    dispatcher = algorithm.return_to_dispatcher();

    let original_image = test_utils::image::coordinates_to_colors(&dimensions);
    let permuted_image =
        test_utils::permutation::non_identity_horizontal_swap_forward_permute(&original_image);
    let original_lossless_image = LosslessImage::Rgba16(Rgba16Image::new(original_image)?);

    let mut algorithm = dispatcher.permute(
        PermuteInput {
            original_image: Some(original_lossless_image.clone()),
            ..Default::default()
        },
        Default::default(),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output().unwrap();
    assert!(output.permutation.is_none());
    assert_eq!(output.original_image.unwrap(), original_lossless_image);
    assert_eq!(
        output.permuted_image,
        LosslessImage::Rgba16(Rgba16Image::new(permuted_image)?)
    );
    Ok(())
}

#[test]
fn reuse_image() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::identity();
    let mut expected_permutation = permutation.clone();
    let original_image = test_utils::image::coordinates_to_colors(&dimensions);
    let mut permuted_image = test_utils::permutation::identity_permute(&original_image);
    let original_lossless_image = LosslessImage::Rgba16(Rgba16Image::new(original_image.clone())?);

    let mut dispatcher = compute::create_dispatcher(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(CandidatePermutation::new(permutation)?),
            original_image: Some(original_lossless_image.clone()),
        },
        Default::default(),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let mut output = algorithm.full_output().unwrap();
    assert_eq!(*output.permutation.unwrap().as_ref(), expected_permutation);
    assert_eq!(output.original_image.unwrap(), original_lossless_image);
    assert_eq!(
        output.permuted_image,
        LosslessImage::Rgba16(Rgba16Image::new(permuted_image)?)
    );

    dispatcher = algorithm.return_to_dispatcher();
    let DimensionsAndPermutation {
        permutation: other_permutation,
        dimensions: other_dimensions,
    } = test_utils::permutation::non_identity();
    assert_eq!(dimensions, other_dimensions);
    expected_permutation = other_permutation.clone();
    permuted_image = test_utils::permutation::non_identity_forward_permute(&original_image);

    algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(CandidatePermutation::new(other_permutation)?),
            ..Default::default()
        },
        PermuteParameters {
            permuted_image_format: Some(ImageFormat::Rgba16),
        },
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    output = algorithm.full_output().unwrap();
    assert_eq!(*output.permutation.unwrap().as_ref(), expected_permutation);
    assert!(output.original_image.is_none());
    assert_eq!(
        output.permuted_image,
        LosslessImage::Rgba16(Rgba16Image::new(permuted_image)?)
    );

    Ok(())
}

#[test]
fn reuse_permutation() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::bit_interpretation_cases();
    let expected_permutation = permutation.clone();
    let mut original_image = test_utils::image::coordinates_to_colors(&dimensions);
    let mut permuted_image =
        test_utils::permutation::bit_interpretation_cases_forward_permute(&original_image);
    let mut original_lossless_image =
        LosslessImage::Rgba16(Rgba16Image::new(original_image.clone())?);

    let mut dispatcher = compute::create_dispatcher(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(CandidatePermutation::new(permutation)?),
            original_image: Some(original_lossless_image.clone()),
        },
        Default::default(),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let mut output = algorithm.full_output().unwrap();
    assert_eq!(*output.permutation.unwrap().as_ref(), expected_permutation);
    assert_eq!(output.original_image.unwrap(), original_lossless_image);
    assert_eq!(
        output.permuted_image,
        LosslessImage::Rgba16(Rgba16Image::new(permuted_image)?)
    );

    dispatcher = algorithm.return_to_dispatcher();

    original_image = test_utils::image::coordinates_to_zero_alpha_colors(&dimensions);
    permuted_image =
        test_utils::permutation::bit_interpretation_cases_forward_permute(&original_image);
    original_lossless_image = LosslessImage::Rgba16(Rgba16Image::new(original_image)?);

    algorithm = dispatcher.permute(
        PermuteInput {
            original_image: Some(original_lossless_image.clone()),
            ..Default::default()
        },
        Default::default(),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    output = algorithm.full_output().unwrap();
    assert!(output.permutation.is_none());
    assert_eq!(output.original_image.unwrap(), original_lossless_image);
    assert_eq!(
        output.permuted_image,
        LosslessImage::Rgba16(Rgba16Image::new(permuted_image)?)
    );

    Ok(())
}

#[test]
fn reuse_nothing() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::identity();
    let mut expected_permutation = permutation.clone();
    let mut original_image = test_utils::image::coordinates_to_colors(&dimensions);
    let mut permuted_image = test_utils::permutation::identity_permute(&original_image);
    let mut original_lossless_image = LosslessImage::Rgba16(Rgba16Image::new(original_image)?);

    let mut dispatcher = compute::create_dispatcher(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(CandidatePermutation::new(permutation)?),
            original_image: Some(original_lossless_image.clone()),
        },
        Default::default(),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let mut output = algorithm.full_output().unwrap();
    assert_eq!(*output.permutation.unwrap().as_ref(), expected_permutation);
    assert_eq!(output.original_image.unwrap(), original_lossless_image);
    assert_eq!(
        output.permuted_image,
        LosslessImage::Rgba16(Rgba16Image::new(permuted_image)?)
    );

    dispatcher = algorithm.return_to_dispatcher();
    let DimensionsAndPermutation {
        permutation: other_permutation,
        dimensions: other_dimensions,
    } = test_utils::permutation::non_identity();
    assert_eq!(dimensions, other_dimensions);
    expected_permutation = other_permutation.clone();
    original_image = test_utils::image::coordinates_to_zero_alpha_colors(&dimensions);
    permuted_image = test_utils::permutation::non_identity_forward_permute(&original_image);
    original_lossless_image = LosslessImage::Rgba16(Rgba16Image::new(original_image)?);

    algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(CandidatePermutation::new(other_permutation)?),
            original_image: Some(original_lossless_image.clone()),
        },
        Default::default(),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    output = algorithm.full_output().unwrap();
    assert_eq!(*output.permutation.unwrap().as_ref(), expected_permutation);
    assert_eq!(output.original_image.unwrap(), original_lossless_image);
    assert_eq!(
        output.permuted_image,
        LosslessImage::Rgba16(Rgba16Image::new(permuted_image)?)
    );

    Ok(())
}

#[test]
fn forget_permutation() -> Result<(), Box<dyn Error>> {
    let dimensions = ImageDimensions::new(3, 4).unwrap();
    let dispatcher = compute::create_dispatcher(&Config {
        image_dimensions: dimensions,
    })
    .unwrap();
    let original_lossless_image = LosslessImage::Rgba16(Rgba16Image::new(
        test_utils::image::coordinates_to_colors(&dimensions),
    )?);

    let mut algorithm = dispatcher.permute(
        PermuteInput {
            original_image: Some(original_lossless_image),
            ..Default::default()
        },
        Default::default(),
    );
    assert_step_until_error(
        algorithm.as_mut(),
        OutputStatus::FinalFullOutput,
        "an input permutation must be provided as there is none to reuse",
    );
    Ok(())
}

#[test]
fn forget_image() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::identity();

    let dispatcher = compute::create_dispatcher(&Config {
        image_dimensions: dimensions,
    })
    .unwrap();
    let mut algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(CandidatePermutation::new(permutation)?),
            ..Default::default()
        },
        PermuteParameters {
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
