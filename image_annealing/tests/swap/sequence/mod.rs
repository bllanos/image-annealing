use image_annealing::compute::conversion::{self, VectorFieldEntry};
use image_annealing::compute::format::{LosslessImage, Rgba16Image};
use image_annealing::compute::{
    self, Config, CreateDisplacementGoalParameters, CreatePermutationInput,
    CreatePermutationParameters, OutputStatus, PermuteInput, SwapInput, SwapParameters, SwapPass,
};
use image_annealing::{CandidatePermutation, DisplacementGoal};
use std::default::Default;
use std::error::Error;
use test_utils::algorithm::{assert_correct_default_swap_full_output, assert_step_until_success};
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

    let mut dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm =
        dispatcher.create_permutation(CreatePermutationInput {}, &CreatePermutationParameters {});
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;
    dispatcher = algorithm.return_to_dispatcher();

    let swap_parameters = test_utils::algorithm::default_swap_parameters();
    let mut algorithm = dispatcher.swap(
        SwapInput {
            displacement_goal: Some(displacement_goal),
            ..Default::default()
        },
        &swap_parameters,
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert!(output.input.as_ref().unwrap().permutation.is_none());
    assert_eq!(
        output
            .input
            .as_ref()
            .unwrap()
            .displacement_goal
            .as_ref()
            .unwrap()
            .as_ref(),
        &expected_displacement_goal
    );
    assert_eq!(*output.output_permutation.as_ref(), expected_permutation);
    assert_eq!(output.pass, SwapPass::Horizontal);
    assert_correct_swap_count_output(
        algorithm.as_mut(),
        &swap_parameters,
        &dimensions,
        SwapAcceptedCount::All,
    );
    Ok(())
}

#[test]
fn zero_initialized_permutation() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::identity();
    let expected_permutation = test_utils::operation::swap(&permutation);
    let displacement_goal =
        DisplacementGoal::from_raw_candidate_permutation(expected_permutation.clone())?;
    let expected_displacement_goal = displacement_goal.as_ref().clone();

    let dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;

    let swap_parameters = test_utils::algorithm::default_swap_parameters();
    let mut algorithm = dispatcher.swap(
        SwapInput {
            displacement_goal: Some(displacement_goal),
            ..Default::default()
        },
        &swap_parameters,
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert!(output.input.as_ref().unwrap().permutation.is_none());
    assert_eq!(
        output
            .input
            .as_ref()
            .unwrap()
            .displacement_goal
            .as_ref()
            .unwrap()
            .as_ref(),
        &expected_displacement_goal
    );
    assert_eq!(*output.output_permutation.as_ref(), expected_permutation);
    assert_eq!(output.pass, SwapPass::Horizontal);
    assert_correct_swap_count_output(
        algorithm.as_mut(),
        &swap_parameters,
        &dimensions,
        SwapAcceptedCount::All,
    );
    Ok(())
}

#[test]
fn create_identity_displacement_goal() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::non_identity();
    let mut dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;

    let mut algorithm = dispatcher
        .create_displacement_goal(Default::default(), &CreateDisplacementGoalParameters {});
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;
    dispatcher = algorithm.return_to_dispatcher();

    let expected_permutation = conversion::to_image(
        &dimensions,
        &[
            VectorFieldEntry(0, 1),
            VectorFieldEntry(0, 0),
            VectorFieldEntry(0, -1),
            VectorFieldEntry(-1, 1),
            VectorFieldEntry(1, -1),
            VectorFieldEntry(0, 0),
        ],
    );
    let swap_parameters = test_utils::algorithm::default_swap_parameters();
    let mut algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation::new(permutation.clone())?),
            ..Default::default()
        },
        &swap_parameters,
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialOutput)?;

    let output = algorithm.full_output_block().unwrap();
    let returned_input = output.input.as_ref().unwrap();
    assert_eq!(
        returned_input.permutation.as_ref().unwrap().as_ref(),
        &permutation
    );
    assert!(returned_input.displacement_goal.is_none());
    assert_eq!(output.output_permutation.as_ref(), &expected_permutation);
    assert_eq!(output.pass, SwapPass::Horizontal);
    assert!(algorithm.full_output_block().is_none());

    assert_correct_swap_count_output(
        algorithm.as_mut(),
        &swap_parameters,
        &dimensions,
        SwapAcceptedCount::Some(vec![1]),
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

    let mut dispatcher = compute::create_dispatcher_block(&Config {
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

    let output = algorithm.full_output_block().unwrap();
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
        &swap_parameters,
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert!(output.input.as_ref().unwrap().permutation.is_none());
    assert_eq!(
        output
            .input
            .as_ref()
            .unwrap()
            .displacement_goal
            .as_ref()
            .unwrap()
            .as_ref(),
        &expected_displacement_goal
    );
    assert_eq!(*output.output_permutation.as_ref(), expected_permutation);
    assert_eq!(output.pass, SwapPass::Horizontal);
    assert_correct_swap_count_output(
        algorithm.as_mut(),
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

    let mut dispatcher = compute::create_dispatcher_block(&Config {
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
        algorithm.as_mut(),
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
        &swap_parameters,
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialOutput)?;

    assert_correct_default_swap_full_output(
        algorithm.as_mut(),
        &other_permutation,
        &expected_displacement_goal,
        &expected_permutation,
    );
    assert_correct_swap_count_output(
        algorithm.as_mut(),
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

    let mut dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;
    let swap_parameters = test_utils::algorithm::default_swap_parameters();
    let mut algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation::new(permutation.clone())?),
            displacement_goal: Some(intermediate_displacement_goal),
        },
        &swap_parameters,
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialOutput)?;

    assert_correct_default_swap_full_output(
        algorithm.as_mut(),
        &permutation,
        &expected_intermediate_displacement_goal,
        &intermediate_permutation,
    );
    assert_correct_swap_count_output(
        algorithm.as_mut(),
        &swap_parameters,
        &dimensions,
        SwapAcceptedCount::All,
    );
    dispatcher = algorithm.return_to_dispatcher();

    algorithm = dispatcher.swap(Default::default(), &swap_parameters);
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert!(output.input.is_none());
    assert_eq!(
        *output.output_permutation.as_ref(),
        intermediate_permutation
    );
    assert_eq!(output.pass, SwapPass::Horizontal);
    assert_correct_swap_count_output(
        algorithm.as_mut(),
        &swap_parameters,
        &dimensions,
        SwapAcceptedCount::None,
    );
    Ok(())
}

#[test]
fn run_twice_previous_pass_not_counted() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::eight_cycle();
    let intermediate_permutation = conversion::to_image(
        &dimensions,
        &[
            VectorFieldEntry(1, 0),
            VectorFieldEntry(1, 0),
            VectorFieldEntry(0, 1),
            VectorFieldEntry(0, 0),
            VectorFieldEntry(0, 0),
            VectorFieldEntry(-1, 1),
            VectorFieldEntry(0, -2),
            VectorFieldEntry(-1, 0),
            VectorFieldEntry(0, 0),
        ],
    );
    let final_permutation = conversion::to_image(
        &dimensions,
        &[
            VectorFieldEntry(2, 0),
            VectorFieldEntry(0, 0),
            VectorFieldEntry(0, 1),
            VectorFieldEntry(0, 0),
            VectorFieldEntry(0, 0),
            VectorFieldEntry(-1, 1),
            VectorFieldEntry(0, -2),
            VectorFieldEntry(-1, 0),
            VectorFieldEntry(0, 0),
        ],
    );

    let intermediate_displacement_goal = DisplacementGoal::from_raw_candidate_permutation(
        test_utils::permutation::eight_cycle2().permutation,
    )?;
    let expected_intermediate_displacement_goal = intermediate_displacement_goal.as_ref().clone();

    let mut dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;
    let pass = SwapPass::OffsetVertical;
    let mut swap_parameters = SwapParameters::from_sequence(pass.into());
    assert!(!swap_parameters.count_swap);

    let mut algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation::new(permutation.clone())?),
            displacement_goal: Some(intermediate_displacement_goal),
        },
        &swap_parameters,
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    let returned_input = output.input.as_ref().unwrap();
    assert_eq!(
        returned_input.permutation.as_ref().unwrap().as_ref(),
        &permutation
    );
    assert_eq!(
        returned_input.displacement_goal.as_ref().unwrap().as_ref(),
        &expected_intermediate_displacement_goal
    );
    assert_eq!(
        output.output_permutation.as_ref(),
        &intermediate_permutation
    );
    assert_eq!(output.pass, pass);
    assert!(algorithm.full_output_block().is_none());

    assert_correct_swap_count_output(
        algorithm.as_mut(),
        &swap_parameters,
        &dimensions,
        SwapAcceptedCount::Some(vec![2]),
    );
    dispatcher = algorithm.return_to_dispatcher();

    swap_parameters = test_utils::algorithm::default_swap_parameters();
    algorithm = dispatcher.swap(Default::default(), &swap_parameters);
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert!(output.input.is_none());
    assert_eq!(*output.output_permutation.as_ref(), final_permutation);
    assert_eq!(output.pass, SwapPass::Horizontal);
    assert!(algorithm.full_output_block().is_none());

    assert_correct_swap_count_output(
        algorithm.as_mut(),
        &swap_parameters,
        &dimensions,
        SwapAcceptedCount::Some(vec![1]),
    );
    Ok(())
}
