use image::{GenericImageView, ImageBuffer};
use std::error::Error;
use std::path::{Path, PathBuf};

pub type VectorFieldImageBufferComponent = u8;
pub type VectorFieldImageBuffer =
    ImageBuffer<image::Rgba<VectorFieldImageBufferComponent>, Vec<VectorFieldImageBufferComponent>>;
pub type VectorFieldImageBufferPixel = <VectorFieldImageBuffer as GenericImageView>::Pixel;

pub type LosslessImageBufferComponent = u16;
pub type LosslessImageBuffer =
    ImageBuffer<image::Rgba<LosslessImageBufferComponent>, Vec<LosslessImageBufferComponent>>;

pub trait ImageFileWriter {
    const EXTENSION: &'static str;
    fn make_filename<P: AsRef<Path>>(path_no_extension: P) -> PathBuf {
        path_no_extension.as_ref().with_extension(Self::EXTENSION)
    }
    fn save_add_extension<P: AsRef<Path>>(
        &self,
        path_no_extension: P,
    ) -> Result<PathBuf, Box<dyn Error>>;
}

impl ImageFileWriter for ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    const EXTENSION: &'static str = "png";
    fn save_add_extension<P: AsRef<Path>>(
        &self,
        path_no_extension: P,
    ) -> Result<PathBuf, Box<dyn Error>> {
        let output_path = Self::make_filename(path_no_extension);
        self.save(&output_path)?;
        Ok(output_path)
    }
}

impl ImageFileWriter for ImageBuffer<image::Rgba<u16>, Vec<u16>> {
    const EXTENSION: &'static str = "png";
    fn save_add_extension<P: AsRef<Path>>(
        &self,
        path_no_extension: P,
    ) -> Result<PathBuf, Box<dyn Error>> {
        let output_path = Self::make_filename(path_no_extension);
        self.save(&output_path)?;
        Ok(output_path)
    }
}
