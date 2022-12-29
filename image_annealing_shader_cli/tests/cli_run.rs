use image_annealing_shader::shader;
use image_annealing_shader_cli::args::{
    AssembleShaderOptions, DefaultShaderOutputOptions, Options,
};
use image_annealing_shader_cli::cli;
use image_annealing_shader_cli::config::{Config, UnverifiedConfig};
use std::error::Error;
use std::fs;

#[test]
fn assemble_create_displacement_goal_shader() -> Result<(), Box<dyn Error>> {
    let output_file =
        test_util::make_test_output_path(["cli_assemble_create_displacement_goal_shader.wgsl"]);
    assert!(!output_file.is_file());

    let shader_body_path = test_util::make_test_data_path_string([
        "shader",
        "create_displacement_goal",
        "copy_image.wgsl",
    ]);

    let config: Config = (UnverifiedConfig::CreateDisplacementGoal {
        body: shader_body_path,
    })
    .try_into()?;
    let options = Options::Assemble(AssembleShaderOptions {
        config: config.clone(),
        output_file: output_file.clone(),
    });
    cli::run(&options)?;

    let mut expected: Vec<u8> = Vec::new();
    match config {
        Config::CreateDisplacementGoal(ref inner) => {
            shader::create_displacement_goal_custom(&mut expected, inner)?
        }
    }
    let actual = std::fs::read(&output_file)?;
    assert_eq!(actual, expected);
    std::fs::remove_file(output_file)?;

    Ok(())
}

#[test]
fn output_default_shaders() -> Result<(), Box<dyn Error>> {
    let output_directory =
        test_util::make_test_output_path(["image_annealing_shader_cli_cli_run_default"]);
    assert!(!output_directory.exists());
    std::fs::create_dir(&output_directory)?;

    let options = Options::Default(DefaultShaderOutputOptions {
        output_directory: Some(output_directory.clone()),
    });
    cli::run(&options)?;

    // File contents are tested in
    // `image_annealing_shader_cli::output::tests::write_default_files::all_shaders()`
    assert_eq!(fs::read_dir(&output_directory)?.count(), 5);
    std::fs::remove_dir_all(output_directory)?;
    Ok(())
}
