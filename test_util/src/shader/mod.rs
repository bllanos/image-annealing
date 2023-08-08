use image_annealing::compute::{
    CreateDisplacementGoalParameters, CreateDisplacementGoalPipelineOperation,
    CreateDisplacementGoalShaderConfig, PipelineConfig, WorkgroupGridConfig,
};
use image_annealing_shader::{CreateDisplacementGoalShaderContent, SHADER_ENTRY_POINT};
use std::borrow::Cow;
use std::fs;
use std::num::NonZeroU32;

pub fn load_create_displacement_goal_shader_content(
    shader_name: &str,
) -> CreateDisplacementGoalShaderContent<'static> {
    let path = crate::path::absolute_input_file(&format!(
        "shader/create_displacement_goal/{}.wgsl",
        shader_name
    ))
    .0;
    CreateDisplacementGoalShaderContent {
        body: Cow::Owned(fs::read_to_string(path).unwrap()),
    }
}

pub fn make_create_displacement_goal_parameters(
    shader_name: &str,
) -> CreateDisplacementGoalParameters {
    CreateDisplacementGoalParameters {
        pipeline_operation: CreateDisplacementGoalPipelineOperation::Set(PipelineConfig {
            shader_config: CreateDisplacementGoalShaderConfig {
                content: load_create_displacement_goal_shader_content(shader_name),
                entry_point: Cow::Borrowed(SHADER_ENTRY_POINT),
            },
            workgroup_grid: WorkgroupGridConfig::BlockSize {
                width: NonZeroU32::new(16).unwrap(),
                height: NonZeroU32::new(16).unwrap(),
            },
        }),
    }
}
