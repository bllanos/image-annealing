use super::{
    ImageFileReader, ImageFileWriter, Rgba16Image, Rgba16Rgba8Image, Rgba16Rgba8x2Image,
    Rgba16x2Image, Rgba8Image, Rgba8x2Image, Rgba8x3Image, Rgba8x4Image,
};
use crate::{ImageDimensions, ImageDimensionsHolder};
use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImageFormat {
    Rgba8,
    Rgba8x2,
    Rgba8x3,
    Rgba8x4,
    Rgba16,
    Rgba16x2,
    Rgba16Rgba8,
    Rgba16Rgba8x2,
}

impl fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ImageFormat::Rgba8 => write!(f, "8-bit RGBA"),
            ImageFormat::Rgba8x2 => write!(f, "2 x 8-bit RGBA"),
            ImageFormat::Rgba8x3 => write!(f, "3 x 8-bit RGBA"),
            ImageFormat::Rgba8x4 => write!(f, "4 x 8-bit RGBA"),
            ImageFormat::Rgba16 => write!(f, "16-bit RGBA"),
            ImageFormat::Rgba16x2 => write!(f, "2 x 16-bit RGBA"),
            ImageFormat::Rgba16Rgba8 => write!(f, "16-bit RGBA + 8-bit RGBA"),
            ImageFormat::Rgba16Rgba8x2 => write!(f, "16-bit RGBA + 2 x 8-bit RGBA"),
        }
    }
}

#[derive(Debug)]
pub enum ImageFormatError {
    Mismatch {
        image_name: String,
        input_format: ImageFormat,
        output_format: ImageFormat,
    },
    Missing {
        image_name: String,
    },
    Unexpected {
        image_name: String,
        expected_format: ImageFormat,
    },
}

impl fmt::Display for ImageFormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ImageFormatError::Mismatch {
                image_name,
                input_format,
                output_format,
            } => write!(
                f,
                "{} input image has format {}, but the requested output format is {}",
                image_name, input_format, output_format,
            ),
            ImageFormatError::Missing { image_name } => write!(
                f,
                "no image format was provided for the {} image",
                image_name
            ),
            ImageFormatError::Unexpected {
                image_name,
                expected_format,
            } => write!(
                f,
                "actual format of image {} is not the expected format of {}",
                image_name, expected_format
            ),
        }
    }
}

impl Error for ImageFormatError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LosslessImage {
    Rgba8(Rgba8Image),
    Rgba8x2(Rgba8x2Image),
    Rgba8x3(Rgba8x3Image),
    Rgba8x4(Rgba8x4Image),
    Rgba16(Rgba16Image),
    Rgba16x2(Rgba16x2Image),
    Rgba16Rgba8(Rgba16Rgba8Image),
    Rgba16Rgba8x2(Rgba16Rgba8x2Image),
}

impl LosslessImage {
    pub fn load<P: AsRef<Path>>(format: ImageFormat, paths: &[P]) -> Result<Self, Box<dyn Error>> {
        Ok(match format {
            ImageFormat::Rgba8 => Self::Rgba8(Rgba8Image::load(&paths[0])?),
            ImageFormat::Rgba8x2 => Self::Rgba8x2(Rgba8x2Image::load(&paths[0], &paths[1])?),
            ImageFormat::Rgba8x3 => {
                Self::Rgba8x3(Rgba8x3Image::load(&paths[0], &paths[1], &paths[2])?)
            }
            ImageFormat::Rgba8x4 => Self::Rgba8x4(Rgba8x4Image::load(
                &paths[0], &paths[1], &paths[2], &paths[3],
            )?),
            ImageFormat::Rgba16 => Self::Rgba16(Rgba16Image::load(&paths[0])?),
            ImageFormat::Rgba16x2 => Self::Rgba16x2(Rgba16x2Image::load(&paths[0], &paths[1])?),
            ImageFormat::Rgba16Rgba8 => {
                Self::Rgba16Rgba8(Rgba16Rgba8Image::load(&paths[0], &paths[1])?)
            }
            ImageFormat::Rgba16Rgba8x2 => {
                Self::Rgba16Rgba8x2(Rgba16Rgba8x2Image::load(&paths[0], &paths[1], &paths[2])?)
            }
        })
    }

    pub fn save_add_extension<P: AsRef<Path>>(
        &self,
        paths: &[P],
    ) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        Ok(match self {
            LosslessImage::Rgba8(image) => vec![image.save_add_extension(&paths[0])?],
            LosslessImage::Rgba8x2(image) => {
                let result = image.save_add_extension(&paths[0], &paths[1])?;
                vec![result.0, result.1]
            }
            LosslessImage::Rgba8x3(image) => {
                let result = image.save_add_extension(&paths[0], &paths[1], &paths[2])?;
                vec![result.0, result.1, result.2]
            }
            LosslessImage::Rgba8x4(image) => {
                let result =
                    image.save_add_extension(&paths[0], &paths[1], &paths[2], &paths[3])?;
                vec![result.0, result.1, result.2, result.3]
            }
            LosslessImage::Rgba16(image) => vec![image.save_add_extension(&paths[0])?],
            LosslessImage::Rgba16x2(image) => {
                let result = image.save_add_extension(&paths[0], &paths[1])?;
                vec![result.0, result.1]
            }
            LosslessImage::Rgba16Rgba8(image) => {
                let result = image.save_add_extension(&paths[0], &paths[1])?;
                vec![result.0, result.1]
            }
            LosslessImage::Rgba16Rgba8x2(image) => {
                let result = image.save_add_extension(&paths[0], &paths[1], &paths[2])?;
                vec![result.0, result.1, result.2]
            }
        })
    }

    pub fn format(&self) -> ImageFormat {
        match self {
            LosslessImage::Rgba8(_) => ImageFormat::Rgba8,
            LosslessImage::Rgba8x2(_) => ImageFormat::Rgba8x2,
            LosslessImage::Rgba8x3(_) => ImageFormat::Rgba8x3,
            LosslessImage::Rgba8x4(_) => ImageFormat::Rgba8x4,
            LosslessImage::Rgba16(_) => ImageFormat::Rgba16,
            LosslessImage::Rgba16x2(_) => ImageFormat::Rgba16x2,
            LosslessImage::Rgba16Rgba8(_) => ImageFormat::Rgba16Rgba8,
            LosslessImage::Rgba16Rgba8x2(_) => ImageFormat::Rgba16Rgba8x2,
        }
    }

    pub(crate) fn to_texture_data(&self) -> Vec<u8> {
        match self {
            LosslessImage::Rgba8(image) => image.to_texture_data(),
            LosslessImage::Rgba8x2(image) => image.to_texture_data(),
            LosslessImage::Rgba8x3(image) => image.to_texture_data(),
            LosslessImage::Rgba8x4(image) => image.to_texture_data(),
            LosslessImage::Rgba16(image) => image.to_texture_data(),
            LosslessImage::Rgba16x2(image) => image.to_texture_data(),
            LosslessImage::Rgba16Rgba8(image) => image.to_texture_data(),
            LosslessImage::Rgba16Rgba8x2(image) => image.to_texture_data(),
        }
    }

    pub(crate) fn from_texture_data(
        format: ImageFormat,
        width: u32,
        height: u32,
        data: Vec<u8>,
    ) -> Self {
        match format {
            ImageFormat::Rgba8 => Self::Rgba8(Rgba8Image::from_texture_data(width, height, data)),
            ImageFormat::Rgba8x2 => {
                Self::Rgba8x2(Rgba8x2Image::from_texture_data(width, height, data))
            }
            ImageFormat::Rgba8x3 => {
                Self::Rgba8x3(Rgba8x3Image::from_texture_data(width, height, data))
            }
            ImageFormat::Rgba8x4 => {
                Self::Rgba8x4(Rgba8x4Image::from_texture_data(width, height, data))
            }
            ImageFormat::Rgba16 => {
                Self::Rgba16(Rgba16Image::from_texture_data(width, height, data))
            }
            ImageFormat::Rgba16x2 => {
                Self::Rgba16x2(Rgba16x2Image::from_texture_data(width, height, data))
            }
            ImageFormat::Rgba16Rgba8 => {
                Self::Rgba16Rgba8(Rgba16Rgba8Image::from_texture_data(width, height, data))
            }
            ImageFormat::Rgba16Rgba8x2 => {
                Self::Rgba16Rgba8x2(Rgba16Rgba8x2Image::from_texture_data(width, height, data))
            }
        }
    }
}

impl ImageDimensionsHolder for LosslessImage {
    fn dimensions(&self) -> &ImageDimensions {
        match self {
            LosslessImage::Rgba8(image) => image.dimensions(),
            LosslessImage::Rgba8x2(image) => image.dimensions(),
            LosslessImage::Rgba8x3(image) => image.dimensions(),
            LosslessImage::Rgba8x4(image) => image.dimensions(),
            LosslessImage::Rgba16(image) => image.dimensions(),
            LosslessImage::Rgba16x2(image) => image.dimensions(),
            LosslessImage::Rgba16Rgba8(image) => image.dimensions(),
            LosslessImage::Rgba16Rgba8x2(image) => image.dimensions(),
        }
    }
}

#[cfg(test)]
mod tests;
