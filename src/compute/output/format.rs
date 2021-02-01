use image::ImageBuffer;
use std::error::Error;
use std::path::{Path, PathBuf};

pub type PermutationImageBufferComponent = u8;
pub type PermutationImageBuffer =
    ImageBuffer<image::Rgba<PermutationImageBufferComponent>, Vec<PermutationImageBufferComponent>>;

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
