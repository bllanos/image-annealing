//! References:
//! https://github.com/gfx-rs/naga/blob/master/cli/src/main.rs

use image_annealing_shaders::shader;
use std::error::Error;

fn print_err(error: impl Error) {
    eprintln!("\t{}:", error);
    let mut e = error.source();
    while let Some(source) = e {
        eprintln!("\t\t{}", source);
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
            eprintln!("Unable to parse WGSL");
            print_err(&e);
            return Err(Box::new(e));
        }
    };

    if let Err(e) = validator.validate(&module) {
        eprintln!("Validation of WGSL failed");
        print_err(&e);
        return Err(Box::new(e));
    }

    Ok(())
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
