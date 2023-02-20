use super::CUSTOM_SHADER_ENTRY_POINT;
use image_annealing::compute::{
    self, Config, CreateDisplacementGoalInput, CreateDisplacementGoalParameters,
    CreateDisplacementGoalPipelineOperation, CreateDisplacementGoalShaderConfig, OutputStatus,
    PipelineConfig, WorkgroupGridConfig,
};
use image_annealing::{CandidatePermutation, DisplacementGoal, ImageDimensions, VectorField};
use std::borrow::Cow;
use std::default::Default;
use std::error::Error;
use std::num::NonZeroU32;
use test_util::algorithm::assert_step_until_success;
use test_util::image::DimensionsAndRgba8Buffer;
use test_util::permutation::{assert_is_identity, DimensionsAndPermutation};

#[test]
fn default_preserve() -> Result<(), Box<dyn Error>> {
    let dimensions = ImageDimensions::try_new(3, 4)?;

    let mut dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm =
        dispatcher.create_displacement_goal(Default::default(), &Default::default());
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert_is_identity(&output.output_displacement_goal);

    dispatcher = algorithm.return_to_dispatcher();

    algorithm = dispatcher.create_displacement_goal(
        Default::default(),
        &CreateDisplacementGoalParameters {
            pipeline_operation: CreateDisplacementGoalPipelineOperation::Preserve,
        },
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert_is_identity(&output.output_displacement_goal);
    Ok(())
}

#[test]
fn default_default() -> Result<(), Box<dyn Error>> {
    let dimensions = ImageDimensions::try_new(3, 4)?;

    let mut dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm =
        dispatcher.create_displacement_goal(Default::default(), &Default::default());
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert_is_identity(&output.output_displacement_goal);

    dispatcher = algorithm.return_to_dispatcher();

    algorithm = dispatcher.create_displacement_goal(Default::default(), &Default::default());
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert_is_identity(&output.output_displacement_goal);
    Ok(())
}

#[test]
fn default_set() -> Result<(), Box<dyn Error>> {
    let DimensionsAndRgba8Buffer { image, dimensions } = test_util::image::nonzero_rgba8_colors();
    let displacement_goal = DisplacementGoal::from_vector_field(image.clone())?;
    let expected_output_displacement_goal = displacement_goal.clone();

    let mut dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;

    let mut algorithm =
        dispatcher.create_displacement_goal(Default::default(), &Default::default());
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert_is_identity(&output.output_displacement_goal);

    dispatcher = algorithm.return_to_dispatcher();

    algorithm = dispatcher.create_displacement_goal(
        CreateDisplacementGoalInput {
            displacement_goal: Some(displacement_goal),
            ..Default::default()
        },
        &test_util::shader::make_create_displacement_goal_parameters("copy_displacement_goal"),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert_eq!(
        output.input_displacement_goal.as_ref().unwrap(),
        &expected_output_displacement_goal
    );
    assert!(output.permutation.is_none());
    assert!(output.image.is_none());
    assert_eq!(
        &output.output_displacement_goal,
        &expected_output_displacement_goal
    );
    Ok(())
}

#[test]
fn set_preserve() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_util::permutation::non_identity();
    let expected_output_permutation = permutation.clone();

    let mut dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm = dispatcher.create_displacement_goal(
        CreateDisplacementGoalInput {
            candidate_permutation: Some(CandidatePermutation::from_vector_field(permutation)?),
            ..Default::default()
        },
        &test_util::shader::make_create_displacement_goal_parameters("copy_permutation"),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert!(output.input_displacement_goal.is_none());
    assert_eq!(
        output.permutation.as_ref().unwrap(),
        &expected_output_permutation
    );
    assert!(output.image.is_none());
    assert_eq!(
        &output.output_displacement_goal,
        &expected_output_permutation
    );

    dispatcher = algorithm.return_to_dispatcher();

    let displacement_goal = DisplacementGoal::identity(&dimensions);
    algorithm = dispatcher.create_displacement_goal(
        CreateDisplacementGoalInput {
            displacement_goal: Some(displacement_goal),
            ..Default::default()
        },
        &CreateDisplacementGoalParameters {
            pipeline_operation: CreateDisplacementGoalPipelineOperation::Preserve,
        },
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert_is_identity(output.input_displacement_goal.as_ref().unwrap());
    assert!(output.permutation.is_none());
    assert!(output.image.is_none());
    assert_eq!(
        &output.output_displacement_goal,
        &expected_output_permutation
    );
    Ok(())
}

#[test]
fn set_default() -> Result<(), Box<dyn Error>> {
    let DimensionsAndRgba8Buffer { image, dimensions } = test_util::image::nonzero_rgba8_colors();
    let displacement_goal = DisplacementGoal::from_vector_field(image.clone())?;
    let expected_output_displacement_goal = displacement_goal.clone();

    let mut dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;

    let mut algorithm = dispatcher.create_displacement_goal(
        CreateDisplacementGoalInput {
            displacement_goal: Some(displacement_goal),
            ..Default::default()
        },
        &test_util::shader::make_create_displacement_goal_parameters("copy_displacement_goal"),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert_eq!(
        output.input_displacement_goal.as_ref().unwrap(),
        &expected_output_displacement_goal
    );
    assert!(output.permutation.is_none());
    assert!(output.image.is_none());
    assert_eq!(
        &output.output_displacement_goal,
        &expected_output_displacement_goal
    );

    dispatcher = algorithm.return_to_dispatcher();

    algorithm = dispatcher.create_displacement_goal(Default::default(), &Default::default());
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert_is_identity(&output.output_displacement_goal);
    Ok(())
}

#[test]
fn set_set_identical() -> Result<(), Box<dyn Error>> {
    let DimensionsAndRgba8Buffer { image, dimensions } = test_util::image::nonzero_rgba8_colors();
    let mut displacement_goal = DisplacementGoal::from_vector_field(image.clone())?;
    let expected_output_displacement_goal = displacement_goal.clone();

    let mut dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm = dispatcher.create_displacement_goal(
        CreateDisplacementGoalInput {
            displacement_goal: Some(displacement_goal),
            ..Default::default()
        },
        &test_util::shader::make_create_displacement_goal_parameters("copy_displacement_goal"),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert_eq!(
        output.input_displacement_goal.as_ref().unwrap(),
        &expected_output_displacement_goal
    );
    assert!(output.permutation.is_none());
    assert!(output.image.is_none());
    assert_eq!(
        &output.output_displacement_goal,
        &expected_output_displacement_goal
    );

    dispatcher = algorithm.return_to_dispatcher();

    displacement_goal = DisplacementGoal::identity(&dimensions);
    algorithm = dispatcher.create_displacement_goal(
        CreateDisplacementGoalInput {
            displacement_goal: Some(displacement_goal),
            ..Default::default()
        },
        &test_util::shader::make_create_displacement_goal_parameters("copy_displacement_goal"),
    );

    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert_is_identity(output.input_displacement_goal.as_ref().unwrap());
    assert!(output.permutation.is_none());
    assert!(output.image.is_none());
    assert_is_identity(&output.output_displacement_goal);
    Ok(())
}

#[test]
fn set_set_different_shader() -> Result<(), Box<dyn Error>> {
    let DimensionsAndRgba8Buffer { image, dimensions } = test_util::image::nonzero_rgba8_colors();
    let displacement_goal = DisplacementGoal::from_vector_field(image.clone())?;
    let expected_output_displacement_goal = displacement_goal.clone();

    let mut dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;

    let mut algorithm = dispatcher.create_displacement_goal(
        CreateDisplacementGoalInput {
            displacement_goal: Some(displacement_goal),
            ..Default::default()
        },
        &test_util::shader::make_create_displacement_goal_parameters("copy_displacement_goal"),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert_eq!(
        output.input_displacement_goal.as_ref().unwrap(),
        &expected_output_displacement_goal
    );
    assert!(output.permutation.is_none());
    assert!(output.image.is_none());
    assert_eq!(
        &output.output_displacement_goal,
        &expected_output_displacement_goal
    );

    dispatcher = algorithm.return_to_dispatcher();

    algorithm = dispatcher.create_displacement_goal(
        Default::default(),
        &test_util::shader::make_create_displacement_goal_parameters("copy_permutation"),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert_is_identity(&output.output_displacement_goal);
    Ok(())
}

#[test]
fn set_set_different_workgroup_grid() -> Result<(), Box<dyn Error>> {
    let dimensions = ImageDimensions::try_new(8, 12)?;

    let mut dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm = dispatcher.create_displacement_goal(
        Default::default(),
        &CreateDisplacementGoalParameters {
            pipeline_operation: CreateDisplacementGoalPipelineOperation::Set(PipelineConfig {
                shader_config: CreateDisplacementGoalShaderConfig {
                    content: test_util::shader::load_create_displacement_goal_shader_content(
                        "custom_entry_point",
                    ),
                    entry_point: Cow::Borrowed(CUSTOM_SHADER_ENTRY_POINT),
                },
                workgroup_grid: WorkgroupGridConfig::BlockSize {
                    width: NonZeroU32::new(6).unwrap(),
                    height: NonZeroU32::new(4).unwrap(),
                },
            }),
        },
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert_eq!(
        output.output_displacement_goal,
        super::make_filled_rectangle_displacement_goal(
            &dimensions,
            &ImageDimensions::try_new(4, 9)?
        )
    );

    dispatcher = algorithm.return_to_dispatcher();
    algorithm = dispatcher.create_displacement_goal(
        Default::default(),
        &CreateDisplacementGoalParameters {
            pipeline_operation: CreateDisplacementGoalPipelineOperation::Set(PipelineConfig {
                shader_config: CreateDisplacementGoalShaderConfig {
                    content: test_util::shader::load_create_displacement_goal_shader_content(
                        "custom_entry_point",
                    ),
                    entry_point: Cow::Borrowed(CUSTOM_SHADER_ENTRY_POINT),
                },
                workgroup_grid: WorkgroupGridConfig::BlockSize {
                    width: NonZeroU32::new(2).unwrap(),
                    height: NonZeroU32::new(4).unwrap(),
                },
            }),
        },
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert_eq!(
        output.output_displacement_goal,
        super::make_filled_rectangle_displacement_goal(
            &dimensions,
            &ImageDimensions::try_new(8, 9)?
        )
    );
    Ok(())
}

#[test]
fn set_preserve_preserve() -> Result<(), Box<dyn Error>> {
    let dimensions = ImageDimensions::try_new(8, 12)?;
    let expected_displacement_goal = super::make_filled_rectangle_displacement_goal(
        &dimensions,
        &ImageDimensions::try_new(4, 9)?,
    );

    let mut dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm = dispatcher.create_displacement_goal(
        Default::default(),
        &CreateDisplacementGoalParameters {
            pipeline_operation: CreateDisplacementGoalPipelineOperation::Set(PipelineConfig {
                shader_config: CreateDisplacementGoalShaderConfig {
                    content: test_util::shader::load_create_displacement_goal_shader_content(
                        "custom_entry_point",
                    ),
                    entry_point: Cow::Borrowed(CUSTOM_SHADER_ENTRY_POINT),
                },
                workgroup_grid: WorkgroupGridConfig::BlockSize {
                    width: NonZeroU32::new(6).unwrap(),
                    height: NonZeroU32::new(4).unwrap(),
                },
            }),
        },
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert_eq!(
        &output.output_displacement_goal,
        &expected_displacement_goal
    );

    for _ in 0..2 {
        dispatcher = algorithm.return_to_dispatcher();
        algorithm = dispatcher.create_displacement_goal(
            Default::default(),
            &CreateDisplacementGoalParameters {
                pipeline_operation: CreateDisplacementGoalPipelineOperation::Preserve,
            },
        );
        assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

        let output = algorithm.full_output_block().unwrap();
        assert_eq!(
            &output.output_displacement_goal,
            &expected_displacement_goal
        );
    }
    Ok(())
}

#[test]
fn set_preserve_default() -> Result<(), Box<dyn Error>> {
    let DimensionsAndRgba8Buffer { image, dimensions } = test_util::image::nonzero_rgba8_colors();
    let displacement_goal = DisplacementGoal::from_vector_field(image.clone())?;
    let expected_output_displacement_goal = displacement_goal.clone();

    let mut dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;

    let mut algorithm = dispatcher.create_displacement_goal(
        CreateDisplacementGoalInput {
            displacement_goal: Some(displacement_goal),
            ..Default::default()
        },
        &test_util::shader::make_create_displacement_goal_parameters("copy_displacement_goal"),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert_eq!(
        output.input_displacement_goal.as_ref().unwrap(),
        &expected_output_displacement_goal
    );
    assert!(output.permutation.is_none());
    assert!(output.image.is_none());
    assert_eq!(
        &output.output_displacement_goal,
        &expected_output_displacement_goal
    );

    dispatcher = algorithm.return_to_dispatcher();
    algorithm = dispatcher.create_displacement_goal(
        Default::default(),
        &CreateDisplacementGoalParameters {
            pipeline_operation: CreateDisplacementGoalPipelineOperation::Preserve,
        },
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert!(output.input_displacement_goal.is_none());
    assert!(output.permutation.is_none());
    assert!(output.image.is_none());
    assert_eq!(
        &output.output_displacement_goal,
        &expected_output_displacement_goal
    );

    dispatcher = algorithm.return_to_dispatcher();

    algorithm = dispatcher.create_displacement_goal(Default::default(), &Default::default());
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert_is_identity(&output.output_displacement_goal);
    Ok(())
}

#[test]
fn set_preserve_set_different_shader() -> Result<(), Box<dyn Error>> {
    let DimensionsAndRgba8Buffer { image, dimensions } = test_util::image::nonzero_rgba8_colors();
    let displacement_goal = DisplacementGoal::from_vector_field(image.clone())?;
    let expected_output_displacement_goal = displacement_goal.clone();

    let mut dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;

    let mut algorithm = dispatcher.create_displacement_goal(
        CreateDisplacementGoalInput {
            displacement_goal: Some(displacement_goal),
            ..Default::default()
        },
        &test_util::shader::make_create_displacement_goal_parameters("copy_displacement_goal"),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert_eq!(
        output.input_displacement_goal.as_ref().unwrap(),
        &expected_output_displacement_goal
    );
    assert!(output.permutation.is_none());
    assert!(output.image.is_none());
    assert_eq!(
        &output.output_displacement_goal,
        &expected_output_displacement_goal
    );

    dispatcher = algorithm.return_to_dispatcher();
    algorithm = dispatcher.create_displacement_goal(
        Default::default(),
        &CreateDisplacementGoalParameters {
            pipeline_operation: CreateDisplacementGoalPipelineOperation::Preserve,
        },
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert!(output.input_displacement_goal.is_none());
    assert!(output.permutation.is_none());
    assert!(output.image.is_none());
    assert_eq!(
        &output.output_displacement_goal,
        &expected_output_displacement_goal
    );

    dispatcher = algorithm.return_to_dispatcher();

    algorithm = dispatcher.create_displacement_goal(
        Default::default(),
        &test_util::shader::make_create_displacement_goal_parameters("copy_permutation"),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert_is_identity(&output.output_displacement_goal);
    Ok(())
}
