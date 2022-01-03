use image::DynamicImage;
use image_annealing::compute::{
    self, CreatePermutationInput, CreatePermutationParameters, OutputStatus, PermuteInput,
    PermuteParameters, SwapInput, SwapParameters,
};
use image_annealing::{CandidatePermutation, DisplacementGoal};
use std::default::Default;
use std::error::Error;
use test_utils::algorithm::assert_step_until_success;
use test_utils::permutation::DimensionsAndPermutation;

#[test]
fn create_identity_permutation() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::identity();
    let expected_permutation = test_utils::operation::swap(&permutation);
    let displacement_goal = DisplacementGoal::from_candidate_permutation(CandidatePermutation(
        expected_permutation.clone(),
    ))?;
    let expected_displacement_goal = displacement_goal.as_ref().clone();

    let mut dispatcher = compute::create_dispatcher(&dimensions)?;
    let mut algorithm =
        dispatcher.create_permutation(CreatePermutationInput {}, CreatePermutationParameters {});
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;
    dispatcher = algorithm.return_to_dispatcher();

    let mut algorithm = dispatcher.swap(
        SwapInput {
            displacement_goal: Some(displacement_goal),
            ..Default::default()
        },
        SwapParameters {},
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output().unwrap();
    assert!(output.input_permutation.is_none());
    assert_eq!(
        *output.input_displacement_goal.unwrap().as_ref(),
        expected_displacement_goal
    );
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
    let displacement_goal = DisplacementGoal::from_candidate_permutation(CandidatePermutation(
        expected_permutation.clone(),
    ))?;
    let expected_displacement_goal = displacement_goal.as_ref().clone();
    let mut algorithm = dispatcher.swap(
        SwapInput {
            displacement_goal: Some(displacement_goal),
            ..Default::default()
        },
        SwapParameters {},
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output().unwrap();
    assert!(output.input_permutation.is_none());
    assert_eq!(
        *output.input_displacement_goal.unwrap().as_ref(),
        expected_displacement_goal
    );
    assert_eq!(*output.output_permutation.as_ref(), expected_permutation);
    Ok(())
}

#[test]
fn reuse_nothing() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::non_identity();
    let mut expected_permutation = test_utils::operation::swap(&permutation);
    let mut displacement_goal = DisplacementGoal::from_candidate_permutation(
        CandidatePermutation(expected_permutation.clone()),
    )?;
    let mut expected_displacement_goal = displacement_goal.as_ref().clone();

    let mut dispatcher = compute::create_dispatcher(&dimensions)?;
    let mut algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation(permutation.clone())),
            displacement_goal: Some(displacement_goal),
        },
        SwapParameters {},
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output().unwrap();
    assert_eq!(*output.input_permutation.unwrap().as_ref(), permutation);
    assert_eq!(
        *output.input_displacement_goal.unwrap().as_ref(),
        expected_displacement_goal
    );
    assert_eq!(*output.output_permutation.as_ref(), expected_permutation);

    dispatcher = algorithm.return_to_dispatcher();

    let DimensionsAndPermutation {
        permutation: other_permutation,
        dimensions: other_dimensions,
    } = test_utils::permutation::identity();
    assert_eq!(dimensions, other_dimensions);
    expected_permutation = test_utils::operation::swap(&other_permutation);
    displacement_goal = DisplacementGoal::from_candidate_permutation(CandidatePermutation(
        expected_permutation.clone(),
    ))?;
    expected_displacement_goal = displacement_goal.as_ref().clone();
    algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation(other_permutation.clone())),
            displacement_goal: Some(displacement_goal),
        },
        SwapParameters {},
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

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
    Ok(())
}

#[test]
fn run_twice_reflect_around_center() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::reflect_around_center();
    let intermediate_permutation = test_utils::operation::swap(&permutation);
    let intermediate_displacement_goal = DisplacementGoal::from_candidate_permutation(
        CandidatePermutation(intermediate_permutation.clone()),
    )?;
    let expected_intermediate_displacement_goal = intermediate_displacement_goal.as_ref().clone();

    let mut dispatcher = compute::create_dispatcher(&dimensions)?;
    let mut algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation(permutation.clone())),
            displacement_goal: Some(intermediate_displacement_goal),
        },
        SwapParameters {},
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

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
    dispatcher = algorithm.return_to_dispatcher();

    algorithm = dispatcher.swap(Default::default(), SwapParameters {});
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output().unwrap();
    assert!(output.input_permutation.is_none());
    assert!(output.input_displacement_goal.is_none());
    assert_eq!(
        *output.output_permutation.as_ref(),
        intermediate_permutation
    );
    Ok(())
}
