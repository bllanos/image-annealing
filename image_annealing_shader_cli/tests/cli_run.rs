use image_annealing_cli_util::path::TryIntoWithPathContext;
use image_annealing_shader::shader;
use image_annealing_shader_cli::args::{
    AssembleShaderOptions, DefaultShaderOutputOptions, Options,
};
use image_annealing_shader_cli::cli;
use image_annealing_shader_cli::config::{
    Config, UnverifiedConfig, UnverifiedCreateDisplacementGoalConfig,
};
use std::error::Error;
use std::fs;

#[test]
fn assemble_create_displacement_goal_shader() -> Result<(), Box<dyn Error>> {
    let output_file = test_util::unique_absolute_output_file!();
    assert!(!output_file.0.is_file());

    let shader_body_path =
        test_util::path::relative_input_file("shader/create_displacement_goal/copy_image.wgsl");

    let config: Config =
        (UnverifiedConfig::CreateDisplacementGoal(UnverifiedCreateDisplacementGoalConfig {
            body: shader_body_path,
        }))
        .try_into_with_path_context(test_util::path::base_input().0)?;
    let options = Options::Assemble(AssembleShaderOptions {
        config: config.clone(),
        output_file: output_file.0.clone().into_owned(),
    });
    cli::run(&options)?;

    let mut expected: Vec<u8> = Vec::new();
    match config {
        Config::CreateDisplacementGoal(ref inner) => {
            shader::create_displacement_goal_custom(&mut expected, inner)?
        }
    }
    let actual = std::fs::read(&output_file.0)?;
    assert_eq!(actual, expected);
    std::fs::remove_file(&output_file.0)?;

    Ok(())
}

#[test]
fn output_default_shaders() -> Result<(), Box<dyn Error>> {
    let output_directory = test_util::unique_absolute_output_directory!();
    assert!(!output_directory.0.try_exists()?);
    std::fs::create_dir(&output_directory.0)?;

    let options = Options::Default(DefaultShaderOutputOptions {
        output_directory: Some(output_directory.0.clone().into_owned()),
    });
    cli::run(&options)?;

    // File contents are tested in
    // `image_annealing_shader_cli::output::tests::write_default_files::all_shaders()`
    assert_eq!(fs::read_dir(&output_directory.0)?.count(), 5);
    std::fs::remove_dir_all(&output_directory.0)?;
    Ok(())
}
