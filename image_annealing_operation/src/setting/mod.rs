use std::error::Error;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Backend {
    DirectX12,
    Metal,
    OpenGL,
    Vulkan,
    PrimaryNative,
    SecondaryNative,
    Any,
}

impl Default for Backend {
    fn default() -> Self {
        Self::PrimaryNative
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct InvalidBackendNameError(pub String);

impl fmt::Display for InvalidBackendNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "GPU backend name, \"{}\", is not a case-insensitive match with \"directx12\", \"dx12\", \"d3d12\", \"metal\", \"mtl\", \"opengl\", \"gles\", \"gl\", \"vulkan\", \"vk\", \"primary_native\", \"secondary_native\" or \"any\"",
            self.0,
        )
    }
}

impl Error for InvalidBackendNameError {}

impl TryFrom<&str> for Backend {
    type Error = InvalidBackendNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let normalized_value = value.trim();
        if normalized_value.eq_ignore_ascii_case("directx12")
            || normalized_value.eq_ignore_ascii_case("dx12")
            || normalized_value.eq_ignore_ascii_case("d3d12")
        {
            Ok(Self::DirectX12)
        } else if normalized_value.eq_ignore_ascii_case("metal")
            || normalized_value.eq_ignore_ascii_case("mtl")
        {
            Ok(Self::Metal)
        } else if normalized_value.eq_ignore_ascii_case("opengl")
            || normalized_value.eq_ignore_ascii_case("gles")
            || normalized_value.eq_ignore_ascii_case("gl")
        {
            Ok(Self::OpenGL)
        } else if normalized_value.eq_ignore_ascii_case("vulkan")
            || normalized_value.eq_ignore_ascii_case("vk")
        {
            Ok(Self::Vulkan)
        } else if normalized_value.eq_ignore_ascii_case("primary_native") {
            Ok(Self::PrimaryNative)
        } else if normalized_value.eq_ignore_ascii_case("secondary_native") {
            Ok(Self::SecondaryNative)
        } else if normalized_value.eq_ignore_ascii_case("any") {
            Ok(Self::Any)
        } else {
            Err(InvalidBackendNameError(normalized_value.to_string()))
        }
    }
}

impl From<Backend> for wgpu::Backends {
    fn from(value: Backend) -> Self {
        match value {
            Backend::DirectX12 => Self::DX12,
            Backend::Metal => Self::METAL,
            Backend::OpenGL => Self::GL,
            Backend::Vulkan => Self::VULKAN,
            Backend::PrimaryNative => Self::DX12 | Self::METAL | Self::VULKAN,
            Backend::SecondaryNative => Self::GL,
            Backend::Any => Self::DX12 | Self::METAL | Self::GL | Self::VULKAN,
        }
    }
}

#[cfg(test)]
mod tests;
