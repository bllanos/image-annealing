use crate::config::{Config, UnverifiedConfig};
use bpaf::{Bpaf, ShellComp};
use std::path::PathBuf;

#[derive(Debug, Bpaf, Eq, PartialEq)]
#[bpaf(command("assemble"))]
/// Assemble and validate a shader
pub struct AssembleShaderOptions {
    /// Path of the configuration file for creating the shader
    #[bpaf(long, short, argument::<String>("CONFIG_FILE"), complete_shell(ShellComp::File { mask: Some("*.json") }), parse(image_annealing_cli_util::config::io::parse_config_file::<Config, UnverifiedConfig, String>))]
    pub config: Config<'static>,
    /// Path of the shader file to output
    #[bpaf(long, short('f'), argument("OUTPUT_FILE"), complete_shell(ShellComp::File { mask: Some("*.wgsl") }))]
    pub output_file: PathBuf,
}

#[derive(Debug, Bpaf, Eq, PartialEq)]
#[bpaf(command("default"))]
/// Output default shader files
pub struct DefaultShaderOutputOptions {
    /// Path of the directory into which to output built-in shaders
    /// (defaults to the current working directory)
    #[bpaf(long, short('d'), argument("DIRECTORY"), complete_shell(ShellComp::Dir { mask: None }))]
    pub output_directory: Option<PathBuf>,
}

#[derive(Debug, Bpaf, Eq, PartialEq)]
#[bpaf(generate(make_option_parser), options, version)]
/// Generate shader files
pub enum Options {
    Assemble(#[bpaf(external(assemble_shader_options))] AssembleShaderOptions),
    Default(#[bpaf(external(default_shader_output_options))] DefaultShaderOutputOptions),
}

#[cfg(test)]
mod tests;
