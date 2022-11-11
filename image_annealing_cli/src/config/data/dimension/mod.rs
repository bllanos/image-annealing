use image_annealing::{ImageDimensions, InvalidDimensionError};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UnverifiedImageDimensionsConfig {
    pub width: usize,
    pub height: usize,
}

impl TryFrom<UnverifiedImageDimensionsConfig> for ImageDimensions {
    type Error = InvalidDimensionError<usize>;

    fn try_from(value: UnverifiedImageDimensionsConfig) -> Result<Self, Self::Error> {
        Self::try_new(value.width, value.height)
    }
}

#[cfg(test)]
mod tests;
