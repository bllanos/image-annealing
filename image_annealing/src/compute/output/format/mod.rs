use std::error::Error;
use std::path::{Path, PathBuf};

mod compound;
mod dynamic;
mod primitive;

pub use compound::{
    Rgba16Rgba8Image, Rgba16Rgba8x2Image, Rgba16x2Image, Rgba8x2Image, Rgba8x3Image, Rgba8x4Image,
};
pub use dynamic::{ImageFormat, ImageFormatError, LosslessImage};
pub use primitive::{
    Rgba16Image, Rgba16ImageBuffer, Rgba16ImageBufferComponent, Rgba8Image, VectorFieldImageBuffer,
    VectorFieldImageBufferComponent, VectorFieldImageBufferPixel,
};

pub trait ImageFileReader {
    fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
}

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