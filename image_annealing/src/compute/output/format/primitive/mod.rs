use super::super::super::resource::texture::{LosslessImageTexture, TextureDatatype};
use super::{
    ImageFileReader, ImageFileWriter, ImageFileWriterSaveResult, ImageFormat, ImageFormatError,
};
use crate::compute::conversion::VectorFieldEntry;
use crate::{ImageDimensions, ImageDimensionsHolder};
use image::{io::Reader as ImageReader, GenericImageView, ImageBuffer};
use std::error::Error;
use std::path::Path;

pub type VectorFieldImageBufferComponent = u8;
pub type VectorFieldImageBuffer =
    ImageBuffer<image::Rgba<VectorFieldImageBufferComponent>, Vec<VectorFieldImageBufferComponent>>;
pub type VectorFieldImageBufferPixel = <VectorFieldImageBuffer as GenericImageView>::Pixel;

pub type Rgba16ImageBufferComponent = u16;
pub type Rgba16ImageBuffer =
    ImageBuffer<image::Rgba<Rgba16ImageBufferComponent>, Vec<Rgba16ImageBufferComponent>>;

impl ImageFileReader for ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let image = ImageReader::open(&path)?.decode()?;
        image.as_rgba8().ok_or(ImageFormatError::Unexpected {
            image_name: format!("{}", path.as_ref().display()),
            expected_format: ImageFormat::Rgba8,
        })?;
        Ok(image.into_rgba8())
    }
}

impl ImageFileReader
    for ImageBuffer<image::Rgba<Rgba16ImageBufferComponent>, Vec<Rgba16ImageBufferComponent>>
{
    fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let image = ImageReader::open(&path)?.decode()?;
        image.as_rgba16().ok_or(ImageFormatError::Unexpected {
            image_name: format!("{}", path.as_ref().display()),
            expected_format: ImageFormat::Rgba16,
        })?;
        Ok(image.into_rgba16())
    }
}

impl ImageFileWriter for ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    const EXTENSION: &'static str = "png";
    fn save_add_extension<P: AsRef<Path>>(
        &self,
        path_no_extension: P,
    ) -> ImageFileWriterSaveResult {
        let output_path = Self::make_filename(path_no_extension);
        self.save(&output_path)?;
        Ok(output_path)
    }
}

impl ImageFileWriter
    for ImageBuffer<image::Rgba<Rgba16ImageBufferComponent>, Vec<Rgba16ImageBufferComponent>>
{
    const EXTENSION: &'static str = "png";
    fn save_add_extension<P: AsRef<Path>>(
        &self,
        path_no_extension: P,
    ) -> ImageFileWriterSaveResult {
        let output_path = Self::make_filename(path_no_extension);
        self.save(&output_path)?;
        Ok(output_path)
    }
}

pub fn identity(dimensions: &ImageDimensions) -> VectorFieldImageBuffer {
    VectorFieldImageBuffer::from_pixel(
        dimensions.width().try_into().unwrap(),
        dimensions.height().try_into().unwrap(),
        VectorFieldEntry::identity().to_pixel(),
    )
}

pub fn is_identity(image: &VectorFieldImageBuffer) -> bool {
    let identity_pixel = VectorFieldEntry::identity().to_pixel();
    image.pixels().all(|px| *px == identity_pixel)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rgba8Image {
    dimensions: ImageDimensions,
    image: image::RgbaImage,
}

impl Rgba8Image {
    pub fn new(image: image::RgbaImage) -> Result<Self, Box<dyn Error>> {
        let dimensions = ImageDimensions::from_image(&image)?;
        Ok(Self { dimensions, image })
    }

    pub fn into_inner(self) -> VectorFieldImageBuffer {
        self.image
    }

    pub(crate) fn to_texture_data(&self) -> Vec<u8> {
        self.as_raw_iter()
            .flat_map(|&component| [component, 0u8, 0u8, 0u8])
            .collect()
    }

    pub(crate) fn from_texture_data(width: u32, height: u32, data: Vec<u8>) -> Self {
        let image_data: Vec<u8> = data
            .as_slice()
            .chunks_exact(<LosslessImageTexture as TextureDatatype>::COMPONENT_SIZE)
            .map(|chunk| chunk[0])
            .collect();
        let dimensions = ImageDimensions::try_new(width, height).unwrap();
        Self {
            dimensions,
            image: image::RgbaImage::from_vec(width, height, image_data).unwrap(),
        }
    }

    pub(crate) fn as_raw_iter(&self) -> impl Iterator<Item = &VectorFieldImageBufferComponent> {
        self.image.as_raw().iter()
    }
}

impl AsRef<VectorFieldImageBuffer> for Rgba8Image {
    fn as_ref(&self) -> &VectorFieldImageBuffer {
        &self.image
    }
}

impl PartialEq<VectorFieldImageBuffer> for Rgba8Image {
    fn eq(&self, other: &VectorFieldImageBuffer) -> bool {
        self.as_ref() == other
    }
}

impl ImageDimensionsHolder for Rgba8Image {
    fn dimensions(&self) -> &ImageDimensions {
        &self.dimensions
    }
}

impl ImageFileReader for Rgba8Image {
    fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        Self::new(<image::RgbaImage as ImageFileReader>::load(path)?)
    }
}

impl ImageFileWriter for Rgba8Image {
    const EXTENSION: &'static str = <image::RgbaImage as ImageFileWriter>::EXTENSION;

    fn save_add_extension<P: AsRef<Path>>(
        &self,
        path_no_extension: P,
    ) -> ImageFileWriterSaveResult {
        self.image.save_add_extension(path_no_extension)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rgba16Image {
    dimensions: ImageDimensions,
    image: Rgba16ImageBuffer,
}

impl Rgba16Image {
    pub fn new(image: Rgba16ImageBuffer) -> Result<Self, Box<dyn Error>> {
        let dimensions = ImageDimensions::from_image(&image)?;
        Ok(Self { dimensions, image })
    }

    pub fn into_inner(self) -> Rgba16ImageBuffer {
        self.image
    }

    pub(crate) fn to_texture_data(&self) -> Vec<u8> {
        self.as_raw_iter()
            .flat_map(|&component| {
                let bytes = component.to_ne_bytes();
                [bytes[0], bytes[1], 0u8, 0u8]
            })
            .collect()
    }

    pub(crate) fn from_texture_data(width: u32, height: u32, data: Vec<u8>) -> Self {
        let image_data: Vec<Rgba16ImageBufferComponent> = data
            .as_slice()
            .chunks_exact(<LosslessImageTexture as TextureDatatype>::COMPONENT_SIZE)
            .map(|chunk| Rgba16ImageBufferComponent::from_ne_bytes([chunk[0], chunk[1]]))
            .collect();
        let dimensions = ImageDimensions::try_new(width, height).unwrap();
        Self {
            dimensions,
            image: Rgba16ImageBuffer::from_vec(width, height, image_data).unwrap(),
        }
    }

    pub(crate) fn as_raw_iter(&self) -> impl Iterator<Item = &Rgba16ImageBufferComponent> {
        self.image.as_raw().iter()
    }
}

impl AsRef<Rgba16ImageBuffer> for Rgba16Image {
    fn as_ref(&self) -> &Rgba16ImageBuffer {
        &self.image
    }
}

impl PartialEq<Rgba16ImageBuffer> for Rgba16Image {
    fn eq(&self, other: &Rgba16ImageBuffer) -> bool {
        self.as_ref() == other
    }
}

impl ImageDimensionsHolder for Rgba16Image {
    fn dimensions(&self) -> &ImageDimensions {
        &self.dimensions
    }
}

impl ImageFileReader for Rgba16Image {
    fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        Self::new(<Rgba16ImageBuffer as ImageFileReader>::load(path)?)
    }
}

impl ImageFileWriter for Rgba16Image {
    const EXTENSION: &'static str = <Rgba16ImageBuffer as ImageFileWriter>::EXTENSION;

    fn save_add_extension<P: AsRef<Path>>(
        &self,
        path_no_extension: P,
    ) -> ImageFileWriterSaveResult {
        self.image.save_add_extension(path_no_extension)
    }
}

#[cfg(test)]
mod tests;
