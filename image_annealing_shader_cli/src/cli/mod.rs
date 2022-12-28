use crate::args::{AssembleShaderOptions, DefaultShaderOutputOptions, Options};
use crate::config::Config;
use crate::output;
use image_annealing_shader::{shader, validate};
use std::error::Error;
use std::fs::File;
use std::io::Write;

pub fn run(options: &Options) -> Result<(), Box<dyn Error>> {
    match options {
        Options::Assemble(assemble_shader_options) => assemble_shader(assemble_shader_options),
        Options::Default(default_shader_output_options) => {
            output_default_shaders(default_shader_output_options)
        }
    }
}

fn assemble_shader(options: &AssembleShaderOptions) -> Result<(), Box<dyn Error>> {
    let shader = create_shader(&options.config)?;
    let mut file_writer = File::create(&options.output_file)?;
    file_writer.write_all(&shader)?;
    validate::validate_shader(&String::from_utf8(shader)?)
}

fn output_default_shaders(options: &DefaultShaderOutputOptions) -> Result<(), Box<dyn Error>> {
    output::write_default_files(options.output_directory.as_ref()).and(Ok(()))
}

fn create_shader(config: &Config) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut v: Vec<u8> = Vec::new();
    match config {
        Config::CreateDisplacementGoal(content) => {
            shader::create_displacement_goal_custom(&mut v, content)?;
        }
    }
    Ok(v)
}
