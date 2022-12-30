//! References:
//! https://github.com/gfx-rs/naga/blob/master/cli/src/bin/naga.rs

use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ShaderValidationError {
    Parse(naga::front::wgsl::ParseError),
    Module(naga::WithSpan<naga::valid::ValidationError>),
}

impl fmt::Display for ShaderValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Parse(err) => write!(f, "unable to parse WGSL: {}", err),
            Self::Module(err) => write!(f, "shader module validation failed: {}", err),
        }
    }
}

impl Error for ShaderValidationError {}

pub fn validate_shader(shader: &str) -> Result<(), ShaderValidationError> {
    let module = naga::front::wgsl::parse_str(shader).map_err(ShaderValidationError::Parse)?;

    let mut validator = naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    );
    validator
        .validate(&module)
        .map_err(ShaderValidationError::Module)?;

    Ok(())
}

#[cfg(test)]
mod tests;
