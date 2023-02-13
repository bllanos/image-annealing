use image_annealing::compute::format::VectorFieldImageBuffer;
use image_annealing::compute::{
    self, Config, CreateDisplacementGoalParameters, CreateDisplacementGoalPipelineOperation,
    CreateDisplacementGoalShaderConfig, OutputStatus, PipelineConfig, WorkgroupGridConfig,
};
use image_annealing::{DisplacementGoal, ImageDimensions};
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
                    entry_point: Cow::Borrowed("entry_point"),
                },
                workgroup_grid: WorkgroupGridConfig::BlockSize {
                    width: NonZeroU32::new(16).unwrap(),
                    height: NonZeroU32::new(16).unwrap(),
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
        DisplacementGoal::from_vector_field(VectorFieldImageBuffer::from_pixel(
            dimensions.width().try_into().unwrap(),
            dimensions.height().try_into().unwrap(),
            image::Rgba([1, 2, 3, 4]),
        ))?
    );
    assert!(algorithm.full_output_block().is_none());
    Ok(())
}
