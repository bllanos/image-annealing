use image_annealing::compute::conversion::{self, VectorFieldEntry};
use image_annealing::compute::format::{LosslessImage, Rgba8Image};
use image_annealing::compute::{
    self, Config, CreateDisplacementGoalInput, OutputStatus, SwapInput, SwapPass,
};
use image_annealing::{CandidatePermutation, DisplacementGoal, ImageDimensionsHolder, VectorField};
use std::default::Default;
use std::error::Error;
use test_util::algorithm::{assert_correct_default_swap_full_output, assert_step_until_success};
use test_util::operation::{assert_correct_swap_count_output, SwapAcceptedCount};
use test_util::permutation::{assert_is_identity, DimensionsAndPermutation};

#[test]
fn overwrite_swap_displacement_goal() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_util::permutation::non_identity();
    let mut expected_permutation = test_util::operation::swap(&permutation);
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

    let mut algorithm =
        dispatcher.create_displacement_goal(Default::default(), &Default::default());
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert!(output.input_displacement_goal.is_none());
    assert!(output.permutation.is_none());
    assert!(output.image.is_none());
    assert_is_identity(&output.output_displacement_goal);
    assert_eq!(output.output_displacement_goal.dimensions(), &dimensions);
    assert!(algorithm.full_output_block().is_none());

    dispatcher = algorithm.return_to_dispatcher();

    expected_permutation = conversion::to_image(
        &dimensions,
        &[
            VectorFieldEntry(0, 1),
            VectorFieldEntry(0, 0),
            VectorFieldEntry(0, 1),
            VectorFieldEntry(-1, -1),
            VectorFieldEntry(1, -1),
            VectorFieldEntry(0, 0),
        ],
    );

    let mut algorithm = dispatcher.swap(Default::default(), &swap_parameters);
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert!(output.input.is_none());
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
fn custom_swap_displacement_goal() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_util::permutation::non_identity();
    let expected_permutation = test_util::operation::swap(&permutation);
    let expected_displacement_goal =
        DisplacementGoal::from_raw_candidate_permutation(expected_permutation.clone())?;
    let image = LosslessImage::Rgba8(Rgba8Image::new(
        expected_displacement_goal.clone().into_inner(),
    )?);

    let mut dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;

    let mut algorithm = dispatcher.create_displacement_goal(
        CreateDisplacementGoalInput {
            image: Some(image.clone()),
            ..Default::default()
        },
        &test_util::shader::make_create_displacement_goal_parameters("copy_image"),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert!(output.input_displacement_goal.is_none());
    assert!(output.permutation.is_none());
    assert_eq!(output.image.unwrap(), image);
    assert_eq!(output.output_displacement_goal, expected_displacement_goal);
    assert!(algorithm.full_output_block().is_none());

    dispatcher = algorithm.return_to_dispatcher();

    let swap_parameters = test_util::algorithm::default_swap_parameters();
    let mut algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation::from_vector_field(
                permutation.clone(),
            )?),
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
        SwapAcceptedCount::All,
    );

    Ok(())
}
