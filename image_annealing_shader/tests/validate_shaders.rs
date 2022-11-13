//! References:
//! https://github.com/gfx-rs/naga/blob/master/cli/src/main.rs

use image_annealing_shader::shader;
use std::error::Error;

fn print_err(error: impl Error) {
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

fn validate_shader(shader: &str) -> Result<(), Box<dyn Error>> {
    let mut validator = naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    );

    let module = match naga::front::wgsl::parse_str(shader) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Unable to parse WGSL. Output the shader to a file and then run Naga's shader validation program for more information (https://github.com/gfx-rs/naga/blob/master/cli/src/main.rs).");
            print_err(&e);
            return Err(Box::new(e));
        }
    };

    if let Err(e) = validator.validate(&module) {
        eprintln!("Validation of WGSL failed. Output the shader to a file and then run Naga's shader validation program for more information (https://github.com/gfx-rs/naga/blob/master/cli/src/main.rs).");
        print_err(&e);
        return Err(Box::new(e));
    }

    Ok(())
}

#[test]
fn count_swap() -> Result<(), Box<dyn Error>> {
    let mut v: Vec<u8> = Vec::new();
    shader::count_swap(&mut v)?;
    validate_shader(&String::from_utf8(v)?)
}

#[test]
fn create_displacement_goal_default() -> Result<(), Box<dyn Error>> {
    let mut v: Vec<u8> = Vec::new();
    shader::create_displacement_goal_default(&mut v)?;
    validate_shader(&String::from_utf8(v)?)
}

#[test]
fn create_permutation() -> Result<(), Box<dyn Error>> {
    let mut v: Vec<u8> = Vec::new();
    shader::create_permutation(&mut v)?;
    validate_shader(&String::from_utf8(v)?)
}

#[test]
fn permute() -> Result<(), Box<dyn Error>> {
    let mut v: Vec<u8> = Vec::new();
    shader::permute(&mut v)?;
    validate_shader(&String::from_utf8(v)?)
}

#[test]
fn swap() -> Result<(), Box<dyn Error>> {
    let mut v: Vec<u8> = Vec::new();
    shader::swap(&mut v)?;
    validate_shader(&String::from_utf8(v)?)
}
