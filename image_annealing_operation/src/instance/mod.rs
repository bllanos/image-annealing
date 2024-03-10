use crate::setting::Backend;
use std::error::Error;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DirectX12ShaderCompiler {
    Dxc,
    Fxc,
}

impl Default for DirectX12ShaderCompiler {
    fn default() -> Self {
        Self::Fxc
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct InvalidDirectX12ShaderCompilerNameError(pub String);

impl fmt::Display for InvalidDirectX12ShaderCompilerNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DirectX12 shader compiler name, \"{}\", is not a case-insensitive match with \"dxc\" or \"fxc\"",
            self.0,
        )
    }
}

impl Error for InvalidDirectX12ShaderCompilerNameError {}

impl TryFrom<&str> for DirectX12ShaderCompiler {
    type Error = InvalidDirectX12ShaderCompilerNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let normalized_value = value.trim();
        if normalized_value.eq_ignore_ascii_case("dxc") {
            Ok(Self::Dxc)
        } else if normalized_value.eq_ignore_ascii_case("fxc") {
            Ok(Self::Fxc)
        } else {
            Err(InvalidDirectX12ShaderCompilerNameError(
                normalized_value.to_string(),
            ))
        }
    }
}

impl From<DirectX12ShaderCompiler> for wgpu::Dx12Compiler {
    fn from(value: DirectX12ShaderCompiler) -> Self {
        match value {
            DirectX12ShaderCompiler::Dxc => Self::Dxc {
                dxil_path: None,
                dxc_path: None,
            },
            DirectX12ShaderCompiler::Fxc => Self::Fxc,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Opengles3MinorVersion {
    Automatic,
    Version0,
    Version1,
    Version2,
}

impl Default for Opengles3MinorVersion {
    fn default() -> Self {
        Self::Automatic
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct InvalidOpengles3MinorVersionStringError(pub String);

impl fmt::Display for InvalidOpengles3MinorVersionStringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "minor OpenGL ES 3 version, \"{}\", is not a case-insensitive match with \"automatic\", \"0\", \"1\" or \"2\"",
            self.0
        )
    }
}

impl Error for InvalidOpengles3MinorVersionStringError {}

impl TryFrom<&str> for Opengles3MinorVersion {
    type Error = InvalidOpengles3MinorVersionStringError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let normalized_value = value.trim();
        if normalized_value.eq_ignore_ascii_case("automatic") {
            Ok(Self::Automatic)
        } else if normalized_value.eq_ignore_ascii_case("0") {
            Ok(Self::Version0)
        } else if normalized_value.eq_ignore_ascii_case("1") {
            Ok(Self::Version1)
        } else if normalized_value.eq_ignore_ascii_case("2") {
            Ok(Self::Version2)
        } else {
            Err(InvalidOpengles3MinorVersionStringError(
                normalized_value.to_string(),
            ))
        }
    }
}

impl From<Opengles3MinorVersion> for wgpu::Gles3MinorVersion {
    fn from(value: Opengles3MinorVersion) -> Self {
        match value {
            Opengles3MinorVersion::Automatic => Self::Automatic,
            Opengles3MinorVersion::Version0 => Self::Version0,
            Opengles3MinorVersion::Version1 => Self::Version1,
            Opengles3MinorVersion::Version2 => Self::Version2,
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct InstanceConfig {
    pub backend: Backend,
    pub allow_underlying_noncompliant_adapter: bool,
    pub debug: bool,
    pub validation: bool,
    pub directx12_shader_compiler: DirectX12ShaderCompiler,
    pub opengles3_minor_version: Opengles3MinorVersion,
}

impl From<InstanceConfig> for wgpu::InstanceDescriptor {
    fn from(value: InstanceConfig) -> Self {
        let flags = if value.allow_underlying_noncompliant_adapter {
            wgpu::InstanceFlags::ALLOW_UNDERLYING_NONCOMPLIANT_ADAPTER
        } else {
            wgpu::InstanceFlags::empty()
        } | if value.debug {
            wgpu::InstanceFlags::DEBUG
        } else {
            wgpu::InstanceFlags::empty()
        } | if value.validation {
            wgpu::InstanceFlags::VALIDATION
        } else {
            wgpu::InstanceFlags::empty()
        };

        Self {
            backends: value.backend.into(),
            flags,
            dx12_shader_compiler: value.directx12_shader_compiler.into(),
            gles_minor_version: value.opengles3_minor_version.into(),
        }
    }
}

#[cfg(test)]
mod tests;
