use image_annealing_cli_util::env::{self, InvalidBooleanEnvironmentVariableValueError};
use image_annealing_operation::instance::{
    DirectX12ShaderCompiler, InstanceConfig, InvalidDirectX12ShaderCompilerNameError,
    InvalidOpengles3MinorVersionStringError, Opengles3MinorVersion,
};
use image_annealing_operation::{Backend, InvalidBackendNameError};
use std::error::Error;
use std::fmt;

pub const NUMBER_OF_INSTANCE_ENVIRONMENT_VARIABLES: usize = 6;

pub const INSTANCE_ENVIRONMENT_VARIABLE_KEYS: [&'static str;
    NUMBER_OF_INSTANCE_ENVIRONMENT_VARIABLES] = [
    "WGPU_BACKEND_NAME",
    "WGPU_ALLOW_UNDERLYING_NONCOMPLIANT_ADAPTER",
    "WGPU_DEBUG",
    "WGPU_VALIDATION",
    "WGPU_DX12_COMPILER",
    "WGPU_GLES_MINOR_VERSION",
];

#[derive(Debug, Eq, PartialEq)]
pub enum InvalidInstanceConfigEnvironmentVariableValueError {
    BackendName(InvalidBackendNameError),
    Boolean(InvalidBooleanEnvironmentVariableValueError),
    DirectX12ShaderCompilerName(InvalidDirectX12ShaderCompilerNameError),
    Opengles3MinorVersionString(InvalidOpengles3MinorVersionStringError),
}

impl fmt::Display for InvalidInstanceConfigEnvironmentVariableValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BackendName(error) => error.fmt(f),
            Self::Boolean(error) => error.fmt(f),
            Self::DirectX12ShaderCompilerName(error) => error.fmt(f),
            Self::Opengles3MinorVersionString(error) => error.fmt(f),
        }
    }
}

impl Error for InvalidInstanceConfigEnvironmentVariableValueError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(match self {
            Self::BackendName(error) => error,
            Self::Boolean(error) => error,
            Self::DirectX12ShaderCompilerName(error) => error,
            Self::Opengles3MinorVersionString(error) => error,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct InvalidInstanceConfigEnvironmentVariableError {
    pub key: String,
    pub error: InvalidInstanceConfigEnvironmentVariableValueError,
}

impl InvalidInstanceConfigEnvironmentVariableError {
    pub fn new(key: &str, error: InvalidInstanceConfigEnvironmentVariableValueError) -> Self {
        Self {
            key: String::from(key),
            error,
        }
    }
}

impl fmt::Display for InvalidInstanceConfigEnvironmentVariableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "error interpreting environment variable \"{}\": {}",
            self.key, self.error
        )
    }
}

impl Error for InvalidInstanceConfigEnvironmentVariableError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}

pub fn environment_variables_to_instance_config(
    data: [Option<&str>; NUMBER_OF_INSTANCE_ENVIRONMENT_VARIABLES],
) -> Result<InstanceConfig, InvalidInstanceConfigEnvironmentVariableError> {
    let backend = match data[0] {
        Some(string) => Backend::try_from(string).map_err(|error| {
            InvalidInstanceConfigEnvironmentVariableError::new(
                INSTANCE_ENVIRONMENT_VARIABLE_KEYS[0],
                InvalidInstanceConfigEnvironmentVariableValueError::BackendName(error),
            )
        })?,
        None => Default::default(),
    };
    let allow_underlying_noncompliant_adapter = match data[1] {
        Some(string) => env::parse_bool_from_environment_variable(string).map_err(|error| {
            InvalidInstanceConfigEnvironmentVariableError::new(
                INSTANCE_ENVIRONMENT_VARIABLE_KEYS[1],
                InvalidInstanceConfigEnvironmentVariableValueError::Boolean(error),
            )
        })?,
        None => Default::default(),
    };
    let debug = match data[2] {
        Some(string) => env::parse_bool_from_environment_variable(string).map_err(|error| {
            InvalidInstanceConfigEnvironmentVariableError::new(
                INSTANCE_ENVIRONMENT_VARIABLE_KEYS[2],
                InvalidInstanceConfigEnvironmentVariableValueError::Boolean(error),
            )
        })?,
        None => Default::default(),
    };
    let validation = match data[3] {
        Some(string) => env::parse_bool_from_environment_variable(string).map_err(|error| {
            InvalidInstanceConfigEnvironmentVariableError::new(
                INSTANCE_ENVIRONMENT_VARIABLE_KEYS[3],
                InvalidInstanceConfigEnvironmentVariableValueError::Boolean(error),
            )
        })?,
        None => Default::default(),
    };
    let directx12_shader_compiler = match data[4] {
        Some(string) => DirectX12ShaderCompiler::try_from(string).map_err(|error| {
            InvalidInstanceConfigEnvironmentVariableError::new(
                INSTANCE_ENVIRONMENT_VARIABLE_KEYS[4],
                InvalidInstanceConfigEnvironmentVariableValueError::DirectX12ShaderCompilerName(
                    error,
                ),
            )
        })?,
        None => Default::default(),
    };
    let opengles3_minor_version = match data[5] {
        Some(string) => Opengles3MinorVersion::try_from(string).map_err(|error| {
            InvalidInstanceConfigEnvironmentVariableError::new(
                INSTANCE_ENVIRONMENT_VARIABLE_KEYS[5],
                InvalidInstanceConfigEnvironmentVariableValueError::Opengles3MinorVersionString(
                    error,
                ),
            )
        })?,
        None => Default::default(),
    };
    Ok(InstanceConfig {
        backend,
        allow_underlying_noncompliant_adapter,
        debug,
        validation,
        directx12_shader_compiler,
        opengles3_minor_version,
    })
}

// TODO add environment variable parsing functionality
pub const ADAPTER_ENVIRONMENT_VARIABLE_KEYS: [&'static str; 3] =
    ["WGPU_BACKEND_NAME", "WGPU_ADAPTER_NAME", "WGPU_POWER_PREF"];

// TODO add environment variable parsing functionality
pub const DEVICE_ENVIRONMENT_VARIABLE_KEYS: [&'static str; 1] = ["WGPU_TRACE"];

#[cfg(test)]
mod tests;
