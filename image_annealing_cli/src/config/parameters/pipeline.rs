use image_annealing::compute::WorkgroupGridConfig;
use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::num::NonZeroU32;

#[derive(Debug, Clone)]
pub enum InvalidDimensionError {
    ZeroWidth,
    ZeroHeight,
}

impl fmt::Display for InvalidDimensionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ZeroWidth => write!(f, "width is zero"),
            Self::ZeroHeight => write!(f, "height is zero"),
        }
    }
}

impl Error for InvalidDimensionError {}

#[derive(Debug, Clone)]
pub enum InvalidWorkgroupGridDimensionError {
    BlockSize(InvalidDimensionError),
    Fixed(InvalidDimensionError),
}

impl fmt::Display for InvalidWorkgroupGridDimensionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BlockSize(e) => write!(f, "workgroup grid configuration block {}", e),
            Self::Fixed(e) => write!(f, "workgroup grid configured {}", e),
        }
    }
}

impl Error for InvalidWorkgroupGridDimensionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(match self {
            Self::BlockSize(e) | Self::Fixed(e) => e,
        })
    }
}

#[derive(Clone, Deserialize)]
pub enum UnverifiedWorkgroupGridConfig {
    BlockSize { width: u32, height: u32 },
    Fixed { width: u32, height: u32 },
}

impl TryFrom<UnverifiedWorkgroupGridConfig> for WorkgroupGridConfig {
    type Error = InvalidWorkgroupGridDimensionError;

    fn try_from(value: UnverifiedWorkgroupGridConfig) -> Result<Self, Self::Error> {
        Ok(match value {
            UnverifiedWorkgroupGridConfig::BlockSize { width, height } => {
                WorkgroupGridConfig::BlockSize {
                    width: NonZeroU32::new(width).ok_or(
                        InvalidWorkgroupGridDimensionError::BlockSize(
                            InvalidDimensionError::ZeroWidth,
                        ),
                    )?,
                    height: NonZeroU32::new(height).ok_or(
                        InvalidWorkgroupGridDimensionError::BlockSize(
                            InvalidDimensionError::ZeroHeight,
                        ),
                    )?,
                }
            }
            UnverifiedWorkgroupGridConfig::Fixed { width, height } => WorkgroupGridConfig::Fixed {
                width: NonZeroU32::new(width).ok_or(InvalidWorkgroupGridDimensionError::Fixed(
                    InvalidDimensionError::ZeroWidth,
                ))?,
                height: NonZeroU32::new(height).ok_or(
                    InvalidWorkgroupGridDimensionError::Fixed(InvalidDimensionError::ZeroHeight),
                )?,
            },
        })
    }
}

#[derive(Clone, Deserialize)]
pub struct UnverifiedPipelineConfig<T: Clone> {
    pub shader_config: T,
    pub workgroup_grid: UnverifiedWorkgroupGridConfig,
}

#[derive(Clone, Deserialize)]
pub enum UnverifiedPipelineOperationConfig<T: Clone> {
    Set(UnverifiedPipelineConfig<T>),
    SetDefault,
}

impl<T: Clone> Default for UnverifiedPipelineOperationConfig<T> {
    fn default() -> Self {
        Self::SetDefault
    }
}
