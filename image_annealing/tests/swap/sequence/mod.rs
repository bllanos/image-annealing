use image_annealing::compute::format::{LosslessImage, Rgba16Image};
use image_annealing::compute::{
    self, Config, CreatePermutationInput, CreatePermutationParameters, OutputStatus, PermuteInput,
    SwapInput,
};
use image_annealing::{CandidatePermutation, DisplacementGoal};
use std::default::Default;
use std::error::Error;
use test_utils::algorithm::assert_step_until_success;
use test_utils::operation::{assert_correct_swap_count_output, SwapAcceptedCount};
use test_utils::permutation::DimensionsAndPermutation;

#[test]
fn create_identity_permutation() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::identity();
    let expected_permutation = test_utils::operation::swap(&permutation);
    let displacement_goal =
        DisplacementGoal::from_raw_candidate_permutation(expected_permutation.clone())?;
    let expected_displacement_goal = displacement_goal.as_ref().clone();

    let mut dispatcher = compute::create_dispatcher(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm =
        dispatcher.create_permutation(CreatePermutationInput {}, CreatePermutationParameters {});
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;
    dispatcher = algorithm.return_to_dispatcher();

    let swap_parameters = test_utils::algorithm::default_swap_parameters();
    let mut algorithm = dispatcher.swap(
        SwapInput {
            displacement_goal: Some(displacement_goal),
            ..Default::default()
        },
        swap_parameters.clone(),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialAndFullOutput)?;

    let output = algorithm.full_output().unwrap();
    assert!(output.input_permutation.is_none());
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
    let original_lossless_image = LosslessImage::Rgba16(Rgba16Image::new(original_image)?);

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

    let output = algorithm.full_output().unwrap();
    assert_eq!(*output.permutation.unwrap().as_ref(), expected_permutation);
    assert_eq!(output.original_image.unwrap(), original_lossless_image);
    assert_eq!(
        output.permuted_image,
        LosslessImage::Rgba16(Rgba16Image::new(permuted_image)?)
    );

    dispatcher = algorithm.return_to_dispatcher();

    expected_permutation = test_utils::operation::swap(&expected_permutation);
    let displacement_goal =
        DisplacementGoal::from_raw_candidate_permutation(expected_permutation.clone())?;
    let expected_displacement_goal = displacement_goal.as_ref().clone();
    let swap_parameters = test_utils::algorithm::default_swap_parameters();
    let mut algorithm = dispatcher.swap(
        SwapInput {
            displacement_goal: Some(displacement_goal),
            ..Default::default()
        },
        swap_parameters.clone(),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialAndFullOutput)?;

    let output = algorithm.full_output().unwrap();
    assert!(output.input_permutation.is_none());
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
    Ok(())
}

#[test]
fn reuse_nothing() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::non_identity();
    let mut expected_permutation = test_utils::operation::swap(&permutation);
    let mut displacement_goal =
        DisplacementGoal::from_raw_candidate_permutation(expected_permutation.clone())?;
    let mut expected_displacement_goal = displacement_goal.as_ref().clone();

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

    let DimensionsAndPermutation {
        permutation: other_permutation,
        dimensions: other_dimensions,
    } = test_utils::permutation::identity();
    assert_eq!(dimensions, other_dimensions);
    expected_permutation = test_utils::operation::swap(&other_permutation);
    displacement_goal =
        DisplacementGoal::from_raw_candidate_permutation(expected_permutation.clone())?;
    expected_displacement_goal = displacement_goal.as_ref().clone();
    algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation::new(other_permutation.clone())?),
            displacement_goal: Some(displacement_goal),
        },
        swap_parameters.clone(),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialAndFullOutput)?;

    let output = algorithm.full_output().unwrap();
    assert_eq!(
        *output.input_permutation.unwrap().as_ref(),
        other_permutation
    );
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
    Ok(())
}

#[test]
fn run_twice_reflect_around_center() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::reflect_around_center();
    let intermediate_permutation = test_utils::operation::swap(&permutation);
    let intermediate_displacement_goal =
        DisplacementGoal::from_raw_candidate_permutation(intermediate_permutation.clone())?;
    let expected_intermediate_displacement_goal = intermediate_displacement_goal.as_ref().clone();

    let mut dispatcher = compute::create_dispatcher(&Config {
        image_dimensions: dimensions,
    })?;
    let swap_parameters = test_utils::algorithm::default_swap_parameters();
    let mut algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation::new(permutation.clone())?),
            displacement_goal: Some(intermediate_displacement_goal),
        },
        swap_parameters.clone(),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialAndFullOutput)?;

    let output = algorithm.full_output().unwrap();
    assert_eq!(*output.input_permutation.unwrap().as_ref(), permutation);
    assert_eq!(
        *output.input_displacement_goal.unwrap().as_ref(),
        expected_intermediate_displacement_goal
    );
    assert_eq!(
        *output.output_permutation.as_ref(),
        intermediate_permutation
    );
    assert_correct_swap_count_output(
        algorithm.partial_output(),
        &swap_parameters,
        &dimensions,
        SwapAcceptedCount::All,
    );
    dispatcher = algorithm.return_to_dispatcher();

    algorithm = dispatcher.swap(Default::default(), swap_parameters.clone());
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialAndFullOutput)?;

    let output = algorithm.full_output().unwrap();
    assert!(output.input_permutation.is_none());
    assert!(output.input_displacement_goal.is_none());
    assert_eq!(
        *output.output_permutation.as_ref(),
        intermediate_permutation
    );
    assert_correct_swap_count_output(
        algorithm.partial_output(),
        &swap_parameters,
        &dimensions,
        SwapAcceptedCount::None,
    );
    Ok(())
}
