use image::DynamicImage;
use image_annealing::compute;
use image_annealing::compute::{CreatePermutationInput, CreatePermutationParameters};
use image_annealing::compute::{OutputStatus, PermuteInput, PermuteParameters};
use image_annealing::image_utils::ImageDimensions;
use std::default::Default;
use std::error::Error;
use test_utils::algorithm::{assert_step_until_error, assert_step_until_success};
use test_utils::permutation::DimensionsAndPermutation;

#[test]
fn run_once_identity() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::identity();
    let expected_permutation = permutation.clone();
    let original_image = test_utils::image::coordinates_to_colors(&dimensions);
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
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output().unwrap();
    assert_eq!(*output.permutation.unwrap().as_ref(), expected_permutation);
    assert_eq!(output.original_image.unwrap(), dynamic_original_image);
    assert_eq!(output.permuted_image, permuted_image);
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
    assert_step_until_error(algorithm.as_mut(), OutputStatus::FinalFullOutput, "entries (x, y, delta_x, delta_y) = (0, 0, 0, 1) and (x, y, delta_x, delta_y) = (0, 2, 0, -1) both map to location (x, y) = (0, 1)");

    dispatcher = algorithm.return_to_dispatcher();
    algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(permutation),
            original_image: Some(dynamic_original_image.clone()),
        },
        PermuteParameters {},
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;
    let output = algorithm.full_output().unwrap();
    assert_eq!(*output.permutation.unwrap().as_ref(), expected_permutation);
    assert_eq!(output.original_image.unwrap(), dynamic_original_image);
    assert_eq!(output.permuted_image, permuted_image);
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
    let image = DynamicImage::ImageRgba16(test_utils::image::coordinates_to_colors(
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
    let image =
        DynamicImage::ImageRgba16(test_utils::image::coordinates_to_colors(&other_dimensions));

    let dispatcher = compute::create_dispatcher(&other_dimensions)?;
    let mut algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(permutation),
            original_image: Some(image),
        },
        PermuteParameters {},
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
    let dynamic_original_image = DynamicImage::ImageRgba16(original_image);

    let dispatcher = compute::create_dispatcher(&dimensions)?;
    let mut algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(permutation),
            original_image: Some(dynamic_original_image.clone()),
        },
        PermuteParameters {},
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output().unwrap();
    assert_eq!(*output.permutation.unwrap().as_ref(), expected_permutation);
    assert_eq!(output.original_image.unwrap(), dynamic_original_image);
    assert_eq!(output.permuted_image, permuted_image);
    Ok(())
}

#[test]
fn create_identity_permutation() -> Result<(), Box<dyn Error>> {
    let dimensions = ImageDimensions::new(3, 4)?;
    let mut dispatcher = compute::create_dispatcher(&dimensions)?;
    let mut algorithm =
        dispatcher.create_permutation(CreatePermutationInput {}, CreatePermutationParameters {});
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;
    dispatcher = algorithm.return_to_dispatcher();

    let original_image = test_utils::image::coordinates_to_colors(&dimensions);
    let permuted_image = test_utils::permutation::identity_permute(&original_image);
    let dynamic_original_image = DynamicImage::ImageRgba16(original_image);

    let mut algorithm = dispatcher.permute(
        PermuteInput {
            original_image: Some(dynamic_original_image.clone()),
            ..Default::default()
        },
        PermuteParameters {},
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output().unwrap();
    assert!(output.permutation.is_none());
    assert_eq!(output.original_image.unwrap(), dynamic_original_image);
    assert_eq!(output.permuted_image, permuted_image);
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
    let dynamic_original_image = DynamicImage::ImageRgba16(original_image.clone());

    let mut dispatcher = compute::create_dispatcher(&dimensions)?;
    let mut algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(permutation),
            original_image: Some(dynamic_original_image.clone()),
        },
        PermuteParameters {},
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let mut output = algorithm.full_output().unwrap();
    assert_eq!(*output.permutation.unwrap().as_ref(), expected_permutation);
    assert_eq!(output.original_image.unwrap(), dynamic_original_image);
    assert_eq!(output.permuted_image, permuted_image);

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
            candidate_permutation: Some(other_permutation),
            ..Default::default()
        },
        PermuteParameters {},
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    output = algorithm.full_output().unwrap();
    assert_eq!(*output.permutation.unwrap().as_ref(), expected_permutation);
    assert!(output.original_image.is_none());
    assert_eq!(output.permuted_image, permuted_image);

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
    let mut dynamic_original_image = DynamicImage::ImageRgba16(original_image.clone());

    let mut dispatcher = compute::create_dispatcher(&dimensions)?;
    let mut algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(permutation),
            original_image: Some(dynamic_original_image.clone()),
        },
        PermuteParameters {},
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let mut output = algorithm.full_output().unwrap();
    assert_eq!(*output.permutation.unwrap().as_ref(), expected_permutation);
    assert_eq!(output.original_image.unwrap(), dynamic_original_image);
    assert_eq!(output.permuted_image, permuted_image);

    dispatcher = algorithm.return_to_dispatcher();

    original_image = test_utils::image::coordinates_to_zero_alpha_colors(&dimensions);
    permuted_image =
        test_utils::permutation::bit_interpretation_cases_forward_permute(&original_image);
    dynamic_original_image = DynamicImage::ImageRgba16(original_image.clone());

    algorithm = dispatcher.permute(
        PermuteInput {
            original_image: Some(dynamic_original_image.clone()),
            ..Default::default()
        },
        PermuteParameters {},
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    output = algorithm.full_output().unwrap();
    assert!(output.permutation.is_none());
    assert_eq!(output.original_image.unwrap(), dynamic_original_image);
    assert_eq!(output.permuted_image, permuted_image);

    Ok(())
}

#[test]
fn forget_permutation() -> Result<(), Box<dyn Error>> {
    let dimensions = ImageDimensions::new(3, 4).unwrap();
    let dispatcher = compute::create_dispatcher(&dimensions).unwrap();
    let dynamic_original_image =
        DynamicImage::ImageRgba16(test_utils::image::coordinates_to_colors(&dimensions));

    let mut algorithm = dispatcher.permute(
        PermuteInput {
            original_image: Some(dynamic_original_image),
            ..Default::default()
        },
        PermuteParameters {},
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

    let dispatcher = compute::create_dispatcher(&dimensions).unwrap();
    let mut algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(permutation),
            ..Default::default()
        },
        PermuteParameters {},
    );
    assert_step_until_error(
        algorithm.as_mut(),
        OutputStatus::FinalFullOutput,
        "an input image must be provided as there is none to reuse",
    );
    Ok(())
}
