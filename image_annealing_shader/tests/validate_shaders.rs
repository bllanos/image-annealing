use image_annealing_shader::{shader, validate};
use std::error::Error;

fn validate_shader(v: Vec<u8>) -> Result<(), Box<dyn Error>> {
    validate::validate_shader(&String::from_utf8(v)?)?;
    Ok(())
}

#[test]
fn count_swap() -> Result<(), Box<dyn Error>> {
    let mut v: Vec<u8> = Vec::new();
    shader::count_swap(&mut v)?;
    validate_shader(v)
}

#[test]
fn create_displacement_goal_default() -> Result<(), Box<dyn Error>> {
    let mut v: Vec<u8> = Vec::new();
    shader::create_displacement_goal_default(&mut v)?;
    validate_shader(v)
}

#[test]
fn create_permutation() -> Result<(), Box<dyn Error>> {
    let mut v: Vec<u8> = Vec::new();
    shader::create_permutation(&mut v)?;
    validate_shader(v)
}

#[test]
fn permute() -> Result<(), Box<dyn Error>> {
    let mut v: Vec<u8> = Vec::new();
    shader::permute(&mut v)?;
    validate_shader(v)
}

#[test]
fn swap() -> Result<(), Box<dyn Error>> {
    let mut v: Vec<u8> = Vec::new();
    shader::swap(&mut v)?;
    validate_shader(v)
}
