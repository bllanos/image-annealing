use crate::args::{AssembleShaderOptions, DefaultShaderOutputOptions, Options};
use crate::config::Config;
use crate::output;
use image_annealing_shader::{shader, validate};
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone)]
enum ShaderValidationError {
    Parse,
    Module,
}

impl fmt::Display for ShaderValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Parse => write!(f, "generated shader is not valid WGSL"),
            Self::Module => write!(f, "generated shader module validation failed"),
        }
    }
}

impl Error for ShaderValidationError {}

pub fn run(options: &Options) -> Result<(), Box<dyn Error>> {
    match options {
        Options::Assemble(assemble_shader_options) => assemble_shader(assemble_shader_options),
        Options::Default(default_shader_output_options) => {
            output_default_shaders(default_shader_output_options)
        }
    }
}

fn assemble_shader(options: &AssembleShaderOptions) -> Result<(), Box<dyn Error>> {
    let shader = create_shader(&options.config);
    let mut file_writer = File::create(&options.output_file)?;
    file_writer.write_all(&shader)?;
    let shader_string = String::from_utf8(shader).unwrap();
    match validate::validate_shader(&shader_string) {
        Err(validate::ShaderValidationError::Parse(e)) => {
            let output_file_string = options.output_file.to_string_lossy();
            e.emit_to_stderr_with_path(&shader_string, &output_file_string);
            Err(Box::new(ShaderValidationError::Parse))
        }
        Err(validate::ShaderValidationError::Module(e)) => {
            let output_file_string = options.output_file.to_string_lossy();
            emit_annotated_error(&e, &output_file_string, &shader_string);
            print_err(&e);
            Err(Box::new(ShaderValidationError::Module))
        }
        Ok(_) => Ok(()),
    }
}

fn output_default_shaders(options: &DefaultShaderOutputOptions) -> Result<(), Box<dyn Error>> {
    output::write_default_files(options.output_directory.as_ref()).and(Ok(()))
}

fn create_shader(config: &Config) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();
    match config {
        Config::CreateDisplacementGoal(content) => {
            shader::create_displacement_goal_custom(&mut v, content).unwrap();
        }
    }
    v
}

fn emit_annotated_error<E: Error>(
    annotated_error: &naga::WithSpan<E>,
    filename: &str,
    source: &str,
) {
    // Reference:
    // https://github.com/gfx-rs/naga/blob/master/cli/src/bin/naga.rs

    use codespan_reporting::diagnostic::{Diagnostic, Label};
    use codespan_reporting::term::{self, termcolor};

    let files = codespan_reporting::files::SimpleFile::new(filename, source);
    let config = codespan_reporting::term::Config::default();
    let writer = termcolor::StandardStream::stderr(termcolor::ColorChoice::Auto);

    let diagnostic = Diagnostic::error().with_labels(
        annotated_error
            .spans()
            .map(|(span, desc)| {
                Label::primary((), span.to_range().unwrap()).with_message(desc.to_owned())
            })
            .collect(),
    );

    term::emit(&mut writer.lock(), &config, &files, &diagnostic)
        .expect("cannot write annotated error");
}

fn print_err(error: &dyn Error) {
    // Reference:
    // https://github.com/gfx-rs/naga/blob/master/cli/src/bin/naga.rs

    eprint!("{}", error);

    let mut e = error.source();
    if e.is_some() {
        eprintln!(": ");
    } else {
        eprintln!();
    }

    while let Some(source) = e {
        eprintln!("\t{}", source);
        e = source.source();
    }
}

#[cfg(test)]
mod tests;
