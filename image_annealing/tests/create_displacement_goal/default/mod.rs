use image_annealing::compute::format::{LosslessImage, Rgba16Image};
use image_annealing::compute::{self, Config, CreateDisplacementGoalInput, OutputStatus};
use image_annealing::{
    CandidatePermutation, DisplacementGoal, ImageDimensions, ImageDimensionsHolder,
};
use std::default::Default;
use std::error::Error;
use test_utils::algorithm::{assert_step_until_success, assert_step_until_success_async};
use test_utils::permutation::{assert_is_identity, DimensionsAndPermutation};

#[test]
fn run_once_all_inputs() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::non_identity();
    let displacement_goal = DisplacementGoal::from_raw_candidate_permutation(permutation.clone())?;
    let expected_output_displacement_goal = displacement_goal.clone();
    let expected_permutation = permutation.clone();
    let lossless_image = LosslessImage::Rgba16(Rgba16Image::new(
        test_utils::image::coordinates_to_colors(&dimensions),
    )?);
    let expected_image = lossless_image.clone();

    let dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm = dispatcher.create_displacement_goal(
        CreateDisplacementGoalInput {
            displacement_goal: Some(displacement_goal),
            candidate_permutation: Some(CandidatePermutation::new(permutation)?),
            image: Some(lossless_image),
        },
        &Default::default(),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert_eq!(
        output.input_displacement_goal.as_ref().unwrap(),
        &expected_output_displacement_goal
    );
    assert_eq!(*output.permutation.unwrap().as_ref(), expected_permutation);
    assert_eq!(output.image.unwrap(), expected_image);
    assert_is_identity(&output.output_displacement_goal);
    assert_eq!(output.output_displacement_goal.dimensions(), &dimensions);
    assert!(algorithm.full_output_block().is_none());
    Ok(())
}

#[test]
fn run_once_no_inputs() -> Result<(), Box<dyn Error>> {
    let dimensions = ImageDimensions::try_new(3, 4)?;

    let dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;
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
    Ok(())
}

async fn run_once_no_inputs_async_inner() -> Result<(), Box<dyn Error>> {
    let dimensions = ImageDimensions::try_new(3, 4)?;

    let dispatcher = compute::create_dispatcher(&Config {
        image_dimensions: dimensions,
    })
    .await?;
    let mut algorithm =
        dispatcher.create_displacement_goal(Default::default(), &Default::default());
    assert_step_until_success_async(algorithm.as_mut(), OutputStatus::FinalFullOutput).await?;

    let output = algorithm.full_output().await.unwrap();
    assert!(output.input_displacement_goal.is_none());
    assert!(output.permutation.is_none());
    assert!(output.image.is_none());
    assert_is_identity(&output.output_displacement_goal);
    assert_eq!(output.output_displacement_goal.dimensions(), &dimensions);
    assert!(algorithm.full_output().await.is_none());
    Ok(())
}

#[test]
fn run_once_no_inputs_async() -> Result<(), Box<dyn Error>> {
    futures::executor::block_on(run_once_no_inputs_async_inner())
}

#[test]
fn run_twice() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::non_identity();
    let displacement_goal = DisplacementGoal::from_raw_candidate_permutation(permutation.clone())?;
    let lossless_image = LosslessImage::Rgba16(Rgba16Image::new(
        test_utils::image::coordinates_to_colors(&dimensions),
    )?);

    let mut dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm = dispatcher.create_displacement_goal(
        CreateDisplacementGoalInput {
            displacement_goal: Some(displacement_goal),
            candidate_permutation: Some(CandidatePermutation::new(permutation)?),
            image: Some(lossless_image),
        },
        &Default::default(),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;
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
    Ok(())
}
