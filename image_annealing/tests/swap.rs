use image::DynamicImage;
use image_annealing::compute;
use image_annealing::compute::{
    CreatePermutationInput, CreatePermutationParameters, OutputStatus, PermuteInput,
    PermuteParameters, SwapInput, SwapParameters,
};
use image_annealing::image_utils::validation::CandidatePermutation;
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
    let expected_permutation = test_utils::operation::swap(&permutation);

    let dispatcher = compute::create_dispatcher(&dimensions)?;
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
    Ok(())
}

#[test]
fn run_twice_invalid_permutation_valid() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::non_identity();
    let expected_permutation = test_utils::operation::swap(&permutation);

    let DimensionsAndPermutation {
        permutation: invalid_permutation,
        dimensions: other_dimensions,
    } = test_utils::permutation::duplicate();
    assert_eq!(dimensions, other_dimensions);

    let mut dispatcher = compute::create_dispatcher(&dimensions)?;
    let mut algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation(invalid_permutation)),
        },
        SwapParameters {},
    );
    assert_step_until_error(algorithm.as_mut(), OutputStatus::FinalFullOutput, "entries (x, y, delta_x, delta_y) = (0, 0, 0, 1) and (x, y, delta_x, delta_y) = (0, 2, 0, -1) both map to location (x, y) = (0, 1)");

    dispatcher = algorithm.return_to_dispatcher();
    algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation(permutation.clone())),
        },
        SwapParameters {},
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;
    let output = algorithm.full_output().unwrap();
    assert_eq!(*output.input_permutation.unwrap().as_ref(), permutation);
    assert_eq!(*output.output_permutation.as_ref(), expected_permutation);
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

    let dispatcher = compute::create_dispatcher(&other_dimensions)?;
    let mut algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation(permutation)),
        },
        SwapParameters {},
    );
    assert_step_until_error(
        algorithm.as_mut(),
        OutputStatus::FinalFullOutput,
        "mismatch in image dimensions, (width, height) = (3, 3) and (width, height) = (2, 3)",
    );
    Ok(())
}

#[test]
fn reflect_around_center() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::reflect_around_center();
    let expected_permutation = test_utils::operation::swap(&permutation);

    let dispatcher = compute::create_dispatcher(&dimensions)?;
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
    Ok(())
}

#[test]
fn create_identity_permutation() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::identity();
    let expected_permutation = test_utils::operation::swap(&permutation);

    let mut dispatcher = compute::create_dispatcher(&dimensions)?;
    let mut algorithm =
        dispatcher.create_permutation(CreatePermutationInput {}, CreatePermutationParameters {});
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;
    dispatcher = algorithm.return_to_dispatcher();

    let mut algorithm = dispatcher.swap(Default::default(), SwapParameters {});
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output().unwrap();
    assert!(output.input_permutation.is_none());
    assert_eq!(*output.output_permutation.as_ref(), expected_permutation);
    Ok(())
}

#[test]
fn reuse_permutation() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::non_identity();
    let mut expected_permutation = permutation.clone();
    let original_image = test_utils::image::coordinates_to_colors(&dimensions);
    let permuted_image = test_utils::permutation::non_identity_forward_permute(&original_image);
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

    expected_permutation = test_utils::operation::swap(&expected_permutation);
    let mut algorithm = dispatcher.swap(Default::default(), SwapParameters {});
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output().unwrap();
    assert!(output.input_permutation.is_none());
    assert_eq!(*output.output_permutation.as_ref(), expected_permutation);
    Ok(())
}

#[test]
fn forget_permutation() -> Result<(), Box<dyn Error>> {
    let dimensions = ImageDimensions::new(3, 4).unwrap();
    let dispatcher = compute::create_dispatcher(&dimensions).unwrap();

    let mut algorithm = dispatcher.swap(Default::default(), SwapParameters {});
    assert_step_until_error(
        algorithm.as_mut(),
        OutputStatus::FinalFullOutput,
        "an input permutation must be provided as there is none to reuse",
    );
    Ok(())
}

#[test]
fn run_twice_reflect_around_center() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::reflect_around_center();
    let intermediate_permutation = test_utils::operation::swap(&permutation);

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
    assert_eq!(
        *output.output_permutation.as_ref(),
        intermediate_permutation
    );
    dispatcher = algorithm.return_to_dispatcher();

    let mut algorithm = dispatcher.swap(Default::default(), SwapParameters {});
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output().unwrap();
    assert!(output.input_permutation.is_none());
    assert_eq!(*output.output_permutation.as_ref(), permutation);
    Ok(())
}

mod borders {
    use image_annealing::compute;
    use image_annealing::compute::{OutputStatus, SwapInput, SwapParameters};
    use image_annealing::image_utils::validation::CandidatePermutation;
    use std::error::Error;
    use test_utils::algorithm::assert_step_until_success;
    use test_utils::permutation::DimensionsAndPermutation;

    #[test]
    fn single_pixel() -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation {
            permutation,
            dimensions,
        } = test_utils::permutation::identity_with_dimensions(1, 1);
        let expected_permutation = test_utils::operation::swap(&permutation);

        let dispatcher = compute::create_dispatcher(&dimensions)?;
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
        Ok(())
    }

    #[test]
    fn even_even_dimensions() -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation {
            permutation,
            dimensions,
        } = test_utils::permutation::identity_with_dimensions(2, 2);
        let expected_permutation = test_utils::operation::swap(&permutation);

        let dispatcher = compute::create_dispatcher(&dimensions)?;
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
        Ok(())
    }

    #[test]
    fn even_odd_dimensions() -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation {
            permutation,
            dimensions,
        } = test_utils::permutation::identity_with_dimensions(2, 1);
        let expected_permutation = test_utils::operation::swap(&permutation);

        let dispatcher = compute::create_dispatcher(&dimensions)?;
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
        Ok(())
    }

    #[test]
    fn odd_even_dimensions() -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation {
            permutation,
            dimensions,
        } = test_utils::permutation::identity_with_dimensions(1, 2);
        let expected_permutation = test_utils::operation::swap(&permutation);

        let dispatcher = compute::create_dispatcher(&dimensions)?;
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
        Ok(())
    }
}
