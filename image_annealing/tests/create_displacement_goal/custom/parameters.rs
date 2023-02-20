use super::CUSTOM_SHADER_ENTRY_POINT;
use image_annealing::compute::{
    self, Config, CreateDisplacementGoalParameters, CreateDisplacementGoalPipelineOperation,
    CreateDisplacementGoalShaderConfig, OutputStatus, PipelineConfig, WorkgroupGridConfig,
};
use image_annealing::ImageDimensions;
use std::borrow::Cow;
use std::default::Default;
use std::error::Error;
use std::num::NonZeroU32;
use test_util::algorithm::assert_step_until_success;

#[test]
fn custom_entry_point() -> Result<(), Box<dyn Error>> {
    let dimensions = ImageDimensions::try_new(3, 4)?;

    let dispatcher = compute::create_dispatcher_block(&Config {
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
                    width: NonZeroU32::new(2).unwrap(),
                    height: NonZeroU32::new(3).unwrap(),
                },
            }),
        },
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert!(output.input_displacement_goal.is_none());
    assert!(output.permutation.is_none());
    assert!(output.image.is_none());
    assert_eq!(
        output.output_displacement_goal,
        super::make_filled_rectangle_displacement_goal(&dimensions, &dimensions)
    );
    assert!(algorithm.full_output_block().is_none());
    Ok(())
}

#[test]
fn block_size_workgroup_grid_config() -> Result<(), Box<dyn Error>> {
    let dimensions = ImageDimensions::try_new(8, 12)?;

    let dispatcher = compute::create_dispatcher_block(&Config {
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
    assert!(output.input_displacement_goal.is_none());
    assert!(output.permutation.is_none());
    assert!(output.image.is_none());
    assert_eq!(
        output.output_displacement_goal,
        super::make_filled_rectangle_displacement_goal(
            &dimensions,
            &ImageDimensions::try_new(4, 9)?
        )
    );
    assert!(algorithm.full_output_block().is_none());
    Ok(())
}

#[test]
fn fixed_workgroup_grid_config() -> Result<(), Box<dyn Error>> {
    let dimensions = ImageDimensions::try_new(8, 12)?;

    let dispatcher = compute::create_dispatcher_block(&Config {
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
                workgroup_grid: WorkgroupGridConfig::Fixed {
                    width: NonZeroU32::new(2).unwrap(),
                    height: NonZeroU32::new(3).unwrap(),
                },
            }),
        },
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert!(output.input_displacement_goal.is_none());
    assert!(output.permutation.is_none());
    assert!(output.image.is_none());
    assert_eq!(
        output.output_displacement_goal,
        super::make_filled_rectangle_displacement_goal(
            &dimensions,
            &ImageDimensions::try_new(4, 9)?
        )
    );
    assert!(algorithm.full_output_block().is_none());
    Ok(())
}
