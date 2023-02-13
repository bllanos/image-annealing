use image_annealing_shader::CreateDisplacementGoalShaderContent;
use std::borrow::Cow;
use std::fs;

pub fn load_create_displacement_goal_shader_content(
    shader_name: &str,
) -> CreateDisplacementGoalShaderContent<'static> {
    let path = super::make_test_data_path([
        "shader",
        "create_displacement_goal",
        &format!("{}.wgsl", shader_name),
    ]);
    CreateDisplacementGoalShaderContent {
        body: Cow::Owned(fs::read_to_string(path).unwrap()),
    }
}
