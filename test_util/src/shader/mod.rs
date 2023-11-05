use image_annealing::compute::{
    CreateDisplacementGoalParameters, CreateDisplacementGoalPipelineOperation,
    CreateDisplacementGoalShaderConfig, PipelineConfig, WorkgroupGridConfig,
};
use image_annealing_cli_util::path::TryFromWithPathContext;
use image_annealing_cli_util::text::UnverifiedInputTextFilePath;
use image_annealing_shader::{CreateDisplacementGoalShaderContent, SHADER_ENTRY_POINT};
use std::borrow::Cow;
use std::num::NonZeroU32;

pub fn load_create_displacement_goal_shader_content(
    shader_name: &str,
) -> CreateDisplacementGoalShaderContent<'static> {
    let relative_path = format!("shader/create_displacement_goal/{}.wgsl", shader_name);
    let path = UnverifiedInputTextFilePath(crate::path::relative_input_file(&relative_path));
    CreateDisplacementGoalShaderContent {
        body: Cow::Owned(<String as TryFromWithPathContext<
            UnverifiedInputTextFilePath,
        >>::try_from_with_path_context(
            path,
            crate::path::base_input().0
        ).unwrap()),
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
