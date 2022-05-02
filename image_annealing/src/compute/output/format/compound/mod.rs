use super::super::super::resource::texture::{LosslessImageTexture, TextureDatatype};
use super::{
    ImageFileReader, ImageFileWriter, Rgba16Image, Rgba16ImageBuffer, Rgba16ImageBufferComponent,
    Rgba8Image, VectorFieldImageBuffer,
};
use crate::image_utils::{
    check_dimensions_match2, check_dimensions_match3, check_dimensions_match4,
};
use crate::{ImageDimensions, ImageDimensionsHolder};
use std::error::Error;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rgba8x2Image(Rgba8Image, Rgba8Image);

impl Rgba8x2Image {
    pub fn new(image1: image::RgbaImage, image2: image::RgbaImage) -> Result<Self, Box<dyn Error>> {
        let wrapped_image1 = Rgba8Image::new(image1)?;
        let wrapped_image2 = Rgba8Image::new(image2)?;
        check_dimensions_match2(&wrapped_image1, &wrapped_image2)?;
        Ok(Self(wrapped_image1, wrapped_image2))
    }

    pub fn load<P1: AsRef<Path>, P2: AsRef<Path>>(
        path1: P1,
        path2: P2,
    ) -> Result<Self, Box<dyn Error>> {
        let image1 = Rgba8Image::load(path1)?;
        let image2 = Rgba8Image::load(path2)?;
        check_dimensions_match2(&image1, &image2)?;
        Ok(Self(image1, image2))
    }

    pub fn save_add_extension<P1: AsRef<Path>, P2: AsRef<Path>>(
        &self,
        path1_no_extension: P1,
        path2_no_extension: P2,
    ) -> Result<(PathBuf, PathBuf), Box<dyn Error>> {
        self.0
            .save_add_extension(path1_no_extension)
            .and_then(
                |path1| match self.1.save_add_extension(path2_no_extension) {
                    Ok(path2) => Ok((path1, path2)),
                    Err(err) => {
                        std::fs::remove_file(path1).unwrap();
                        Err(err)
                    }
                },
            )
    }

    pub(crate) fn to_texture_data(&self) -> Vec<u8> {
        self.0
            .as_raw_iter()
            .zip(self.1.as_raw_iter())
            .flat_map(|(&component1, &component2)| [component1, component2, 0u8, 0u8])
            .collect()
    }

    pub(crate) fn from_texture_data(width: u32, height: u32, data: Vec<u8>) -> Self {
        let image_data: (Vec<u8>, Vec<u8>) = data
            .as_slice()
            .chunks_exact(<LosslessImageTexture as TextureDatatype>::COMPONENT_SIZE)
            .map(|chunk| (chunk[0], chunk[1]))
            .unzip();
        Self::new(
            image::RgbaImage::from_vec(width, height, image_data.0).unwrap(),
            image::RgbaImage::from_vec(width, height, image_data.1).unwrap(),
        )
        .unwrap()
    }

    pub fn first_inner(&self) -> &VectorFieldImageBuffer {
        self.0.as_ref()
    }

    pub fn second_inner(&self) -> &VectorFieldImageBuffer {
        self.1.as_ref()
    }
}

impl ImageDimensionsHolder for Rgba8x2Image {
    fn dimensions(&self) -> &ImageDimensions {
        self.0.dimensions()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rgba8x3Image(Rgba8Image, Rgba8Image, Rgba8Image);

impl Rgba8x3Image {
    pub fn new(
        image1: image::RgbaImage,
        image2: image::RgbaImage,
        image3: image::RgbaImage,
    ) -> Result<Self, Box<dyn Error>> {
        let wrapped_image1 = Rgba8Image::new(image1)?;
        let wrapped_image2 = Rgba8Image::new(image2)?;
        let wrapped_image3 = Rgba8Image::new(image3)?;
        check_dimensions_match3(&wrapped_image1, &wrapped_image2, &wrapped_image3)?;
        Ok(Self(wrapped_image1, wrapped_image2, wrapped_image3))
    }

    pub fn load<P1: AsRef<Path>, P2: AsRef<Path>, P3: AsRef<Path>>(
        path1: P1,
        path2: P2,
        path3: P3,
    ) -> Result<Self, Box<dyn Error>> {
        let image1 = Rgba8Image::load(path1)?;
        let image2 = Rgba8Image::load(path2)?;
        let image3 = Rgba8Image::load(path3)?;
        check_dimensions_match3(&image1, &image2, &image3)?;
        Ok(Self(image1, image2, image3))
    }

    pub fn save_add_extension<P1: AsRef<Path>, P2: AsRef<Path>, P3: AsRef<Path>>(
        &self,
        path1_no_extension: P1,
        path2_no_extension: P2,
        path3_no_extension: P3,
    ) -> Result<(PathBuf, PathBuf, PathBuf), Box<dyn Error>> {
        self.0
            .save_add_extension(path1_no_extension)
            .and_then(
                |path1| match self.1.save_add_extension(path2_no_extension) {
                    Ok(path2) => match self.2.save_add_extension(path3_no_extension) {
                        Ok(path3) => Ok((path1, path2, path3)),
                        Err(err) => {
                            std::fs::remove_file(path1).unwrap();
                            std::fs::remove_file(path2).unwrap();
                            Err(err)
                        }
                    },
                    Err(err) => {
                        std::fs::remove_file(path1).unwrap();
                        Err(err)
                    }
                },
            )
    }

    pub(crate) fn to_texture_data(&self) -> Vec<u8> {
        self.0
            .as_raw_iter()
            .zip(self.1.as_raw_iter())
            .zip(self.2.as_raw_iter())
            .flat_map(|((&component1, &component2), &component3)| {
                [component1, component2, component3, 0u8]
            })
            .collect()
    }

    pub(crate) fn from_texture_data(width: u32, height: u32, data: Vec<u8>) -> Self {
        let image_data: (Vec<u8>, (Vec<u8>, Vec<u8>)) = data
            .as_slice()
            .chunks_exact(<LosslessImageTexture as TextureDatatype>::COMPONENT_SIZE)
            .map(|chunk| (chunk[0], (chunk[1], chunk[2])))
            .unzip();
        Self::new(
            image::RgbaImage::from_vec(width, height, image_data.0).unwrap(),
            image::RgbaImage::from_vec(width, height, image_data.1 .0).unwrap(),
            image::RgbaImage::from_vec(width, height, image_data.1 .1).unwrap(),
        )
        .unwrap()
    }

    pub fn first_inner(&self) -> &VectorFieldImageBuffer {
        self.0.as_ref()
    }

    pub fn second_inner(&self) -> &VectorFieldImageBuffer {
        self.1.as_ref()
    }

    pub fn third_inner(&self) -> &VectorFieldImageBuffer {
        self.2.as_ref()
    }
}

impl ImageDimensionsHolder for Rgba8x3Image {
    fn dimensions(&self) -> &ImageDimensions {
        self.0.dimensions()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rgba8x4Image(Rgba8Image, Rgba8Image, Rgba8Image, Rgba8Image);

impl Rgba8x4Image {
    pub fn new(
        image1: image::RgbaImage,
        image2: image::RgbaImage,
        image3: image::RgbaImage,
        image4: image::RgbaImage,
    ) -> Result<Self, Box<dyn Error>> {
        let wrapped_image1 = Rgba8Image::new(image1)?;
        let wrapped_image2 = Rgba8Image::new(image2)?;
        let wrapped_image3 = Rgba8Image::new(image3)?;
        let wrapped_image4 = Rgba8Image::new(image4)?;
        check_dimensions_match4(
            &wrapped_image1,
            &wrapped_image2,
            &wrapped_image3,
            &wrapped_image4,
        )?;
        Ok(Self(
            wrapped_image1,
            wrapped_image2,
            wrapped_image3,
            wrapped_image4,
        ))
    }

    pub fn load<P1: AsRef<Path>, P2: AsRef<Path>, P3: AsRef<Path>, P4: AsRef<Path>>(
        path1: P1,
        path2: P2,
        path3: P3,
        path4: P4,
    ) -> Result<Self, Box<dyn Error>> {
        let image1 = Rgba8Image::load(path1)?;
        let image2 = Rgba8Image::load(path2)?;
        let image3 = Rgba8Image::load(path3)?;
        let image4 = Rgba8Image::load(path4)?;
        check_dimensions_match4(&image1, &image2, &image3, &image4)?;
        Ok(Self(image1, image2, image3, image4))
    }

    pub fn save_add_extension<
        P1: AsRef<Path>,
        P2: AsRef<Path>,
        P3: AsRef<Path>,
        P4: AsRef<Path>,
    >(
        &self,
        path1_no_extension: P1,
        path2_no_extension: P2,
        path3_no_extension: P3,
        path4_no_extension: P4,
    ) -> Result<(PathBuf, PathBuf, PathBuf, PathBuf), Box<dyn Error>> {
        self.0
            .save_add_extension(path1_no_extension)
            .and_then(
                |path1| match self.1.save_add_extension(path2_no_extension) {
                    Ok(path2) => match self.2.save_add_extension(path3_no_extension) {
                        Ok(path3) => match self.3.save_add_extension(path4_no_extension) {
                            Ok(path4) => Ok((path1, path2, path3, path4)),
                            Err(err) => {
                                std::fs::remove_file(path1).unwrap();
                                std::fs::remove_file(path2).unwrap();
                                std::fs::remove_file(path3).unwrap();
                                Err(err)
                            }
                        },
                        Err(err) => {
                            std::fs::remove_file(path1).unwrap();
                            std::fs::remove_file(path2).unwrap();
                            Err(err)
                        }
                    },
                    Err(err) => {
                        std::fs::remove_file(path1).unwrap();
                        Err(err)
                    }
                },
            )
    }

    pub(crate) fn to_texture_data(&self) -> Vec<u8> {
        self.0
            .as_raw_iter()
            .zip(self.1.as_raw_iter())
            .zip(self.2.as_raw_iter())
            .zip(self.3.as_raw_iter())
            .flat_map(|(((&component1, &component2), &component3), &component4)| {
                [component1, component2, component3, component4]
            })
            .collect()
    }

    pub(crate) fn from_texture_data(width: u32, height: u32, data: Vec<u8>) -> Self {
        #[allow(clippy::type_complexity)]
        let image_data: (Vec<u8>, (Vec<u8>, (Vec<u8>, Vec<u8>))) = data
            .as_slice()
            .chunks_exact(<LosslessImageTexture as TextureDatatype>::COMPONENT_SIZE)
            .map(|chunk| (chunk[0], (chunk[1], (chunk[2], chunk[3]))))
            .unzip();
        Self::new(
            image::RgbaImage::from_vec(width, height, image_data.0).unwrap(),
            image::RgbaImage::from_vec(width, height, image_data.1 .0).unwrap(),
            image::RgbaImage::from_vec(width, height, image_data.1 .1 .0).unwrap(),
            image::RgbaImage::from_vec(width, height, image_data.1 .1 .1).unwrap(),
        )
        .unwrap()
    }

    pub fn first_inner(&self) -> &VectorFieldImageBuffer {
        self.0.as_ref()
    }

    pub fn second_inner(&self) -> &VectorFieldImageBuffer {
        self.1.as_ref()
    }

    pub fn third_inner(&self) -> &VectorFieldImageBuffer {
        self.2.as_ref()
    }

    pub fn fourth_inner(&self) -> &VectorFieldImageBuffer {
        self.3.as_ref()
    }
}

impl ImageDimensionsHolder for Rgba8x4Image {
    fn dimensions(&self) -> &ImageDimensions {
        self.0.dimensions()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rgba16x2Image(Rgba16Image, Rgba16Image);

impl Rgba16x2Image {
    pub fn new(
        image1: Rgba16ImageBuffer,
        image2: Rgba16ImageBuffer,
    ) -> Result<Self, Box<dyn Error>> {
        let wrapped_image1 = Rgba16Image::new(image1)?;
        let wrapped_image2 = Rgba16Image::new(image2)?;
        check_dimensions_match2(&wrapped_image1, &wrapped_image2)?;
        Ok(Self(wrapped_image1, wrapped_image2))
    }

    pub fn load<P1: AsRef<Path>, P2: AsRef<Path>>(
        path1: P1,
        path2: P2,
    ) -> Result<Self, Box<dyn Error>> {
        let image1 = Rgba16Image::load(path1)?;
        let image2 = Rgba16Image::load(path2)?;
        check_dimensions_match2(&image1, &image2)?;
        Ok(Self(image1, image2))
    }

    pub fn save_add_extension<P1: AsRef<Path>, P2: AsRef<Path>>(
        &self,
        path1_no_extension: P1,
        path2_no_extension: P2,
    ) -> Result<(PathBuf, PathBuf), Box<dyn Error>> {
        Ok((
            self.0.save_add_extension(path1_no_extension)?,
            self.1.save_add_extension(path2_no_extension)?,
        ))
    }

    pub(crate) fn to_texture_data(&self) -> Vec<u8> {
        self.0
            .as_raw_iter()
            .zip(self.1.as_raw_iter())
            .flat_map(|(&component1, &component2)| {
                let bytes1 = component1.to_ne_bytes();
                let bytes2 = component2.to_ne_bytes();
                [bytes1[0], bytes1[1], bytes2[0], bytes2[1]]
            })
            .collect()
    }

    pub(crate) fn from_texture_data(width: u32, height: u32, data: Vec<u8>) -> Self {
        let image_data: (
            Vec<Rgba16ImageBufferComponent>,
            Vec<Rgba16ImageBufferComponent>,
        ) = data
            .as_slice()
            .chunks_exact(<LosslessImageTexture as TextureDatatype>::COMPONENT_SIZE)
            .map(|chunk| {
                (
                    Rgba16ImageBufferComponent::from_ne_bytes([chunk[0], chunk[1]]),
                    Rgba16ImageBufferComponent::from_ne_bytes([chunk[2], chunk[3]]),
                )
            })
            .unzip();
        Self::new(
            Rgba16ImageBuffer::from_vec(width, height, image_data.0).unwrap(),
            Rgba16ImageBuffer::from_vec(width, height, image_data.1).unwrap(),
        )
        .unwrap()
    }

    pub fn first_inner(&self) -> &Rgba16ImageBuffer {
        self.0.as_ref()
    }

    pub fn second_inner(&self) -> &Rgba16ImageBuffer {
        self.1.as_ref()
    }
}

impl ImageDimensionsHolder for Rgba16x2Image {
    fn dimensions(&self) -> &ImageDimensions {
        self.0.dimensions()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rgba16Rgba8Image(Rgba16Image, Rgba8Image);

impl Rgba16Rgba8Image {
    pub fn new(
        image1: Rgba16ImageBuffer,
        image2: image::RgbaImage,
    ) -> Result<Self, Box<dyn Error>> {
        let wrapped_image1 = Rgba16Image::new(image1)?;
        let wrapped_image2 = Rgba8Image::new(image2)?;
        check_dimensions_match2(&wrapped_image1, &wrapped_image2)?;
        Ok(Self(wrapped_image1, wrapped_image2))
    }

    pub fn load<P1: AsRef<Path>, P2: AsRef<Path>>(
        path1: P1,
        path2: P2,
    ) -> Result<Self, Box<dyn Error>> {
        let image1 = Rgba16Image::load(path1)?;
        let image2 = Rgba8Image::load(path2)?;
        check_dimensions_match2(&image1, &image2)?;
        Ok(Self(image1, image2))
    }

    pub fn save_add_extension<P1: AsRef<Path>, P2: AsRef<Path>>(
        &self,
        path1_no_extension: P1,
        path2_no_extension: P2,
    ) -> Result<(PathBuf, PathBuf), Box<dyn Error>> {
        Ok((
            self.0.save_add_extension(path1_no_extension)?,
            self.1.save_add_extension(path2_no_extension)?,
        ))
    }

    pub(crate) fn to_texture_data(&self) -> Vec<u8> {
        self.0
            .as_raw_iter()
            .zip(self.1.as_raw_iter())
            .flat_map(|(&component1, &component2)| {
                let bytes1 = component1.to_ne_bytes();
                [bytes1[0], bytes1[1], component2, 0u8]
            })
            .collect()
    }

    pub(crate) fn from_texture_data(width: u32, height: u32, data: Vec<u8>) -> Self {
        let image_data: (Vec<Rgba16ImageBufferComponent>, Vec<u8>) = data
            .as_slice()
            .chunks_exact(<LosslessImageTexture as TextureDatatype>::COMPONENT_SIZE)
            .map(|chunk| {
                (
                    Rgba16ImageBufferComponent::from_ne_bytes([chunk[0], chunk[1]]),
                    chunk[2],
                )
            })
            .unzip();
        Self::new(
            Rgba16ImageBuffer::from_vec(width, height, image_data.0).unwrap(),
            image::RgbaImage::from_vec(width, height, image_data.1).unwrap(),
        )
        .unwrap()
    }

    pub fn first_inner(&self) -> &Rgba16ImageBuffer {
        self.0.as_ref()
    }

    pub fn second_inner(&self) -> &VectorFieldImageBuffer {
        self.1.as_ref()
    }
}

impl ImageDimensionsHolder for Rgba16Rgba8Image {
    fn dimensions(&self) -> &ImageDimensions {
        self.0.dimensions()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rgba16Rgba8x2Image(Rgba16Image, Rgba8Image, Rgba8Image);

impl Rgba16Rgba8x2Image {
    pub fn new(
        image1: Rgba16ImageBuffer,
        image2: image::RgbaImage,
        image3: image::RgbaImage,
    ) -> Result<Self, Box<dyn Error>> {
        let wrapped_image1 = Rgba16Image::new(image1)?;
        let wrapped_image2 = Rgba8Image::new(image2)?;
        let wrapped_image3 = Rgba8Image::new(image3)?;
        check_dimensions_match3(&wrapped_image1, &wrapped_image2, &wrapped_image3)?;
        Ok(Self(wrapped_image1, wrapped_image2, wrapped_image3))
    }

    pub fn load<P1: AsRef<Path>, P2: AsRef<Path>, P3: AsRef<Path>>(
        path1: P1,
        path2: P2,
        path3: P3,
    ) -> Result<Self, Box<dyn Error>> {
        let image1 = Rgba16Image::load(path1)?;
        let image2 = Rgba8Image::load(path2)?;
        let image3 = Rgba8Image::load(path3)?;
        check_dimensions_match3(&image1, &image2, &image3)?;
        Ok(Self(image1, image2, image3))
    }

    pub fn save_add_extension<P1: AsRef<Path>, P2: AsRef<Path>, P3: AsRef<Path>>(
        &self,
        path1_no_extension: P1,
        path2_no_extension: P2,
        path3_no_extension: P3,
    ) -> Result<(PathBuf, PathBuf, PathBuf), Box<dyn Error>> {
        Ok((
            self.0.save_add_extension(path1_no_extension)?,
            self.1.save_add_extension(path2_no_extension)?,
            self.2.save_add_extension(path3_no_extension)?,
        ))
    }

    pub(crate) fn to_texture_data(&self) -> Vec<u8> {
        self.0
            .as_raw_iter()
            .zip(self.1.as_raw_iter())
            .zip(self.2.as_raw_iter())
            .flat_map(|((&component1, &component2), &component3)| {
                let bytes1 = component1.to_ne_bytes();
                [bytes1[0], bytes1[1], component2, component3]
            })
            .collect()
    }

    pub(crate) fn from_texture_data(width: u32, height: u32, data: Vec<u8>) -> Self {
        let image_data: (Vec<Rgba16ImageBufferComponent>, (Vec<u8>, Vec<u8>)) = data
            .as_slice()
            .chunks_exact(<LosslessImageTexture as TextureDatatype>::COMPONENT_SIZE)
            .map(|chunk| {
                (
                    Rgba16ImageBufferComponent::from_ne_bytes([chunk[0], chunk[1]]),
                    (chunk[2], chunk[3]),
                )
            })
            .unzip();
        Self::new(
            Rgba16ImageBuffer::from_vec(width, height, image_data.0).unwrap(),
            image::RgbaImage::from_vec(width, height, image_data.1 .0).unwrap(),
            image::RgbaImage::from_vec(width, height, image_data.1 .1).unwrap(),
        )
        .unwrap()
    }

    pub fn first_inner(&self) -> &Rgba16ImageBuffer {
        self.0.as_ref()
    }

    pub fn second_inner(&self) -> &VectorFieldImageBuffer {
        self.1.as_ref()
    }

    pub fn third_inner(&self) -> &VectorFieldImageBuffer {
        self.2.as_ref()
    }
}

impl ImageDimensionsHolder for Rgba16Rgba8x2Image {
    fn dimensions(&self) -> &ImageDimensions {
        self.0.dimensions()
    }
}

#[cfg(test)]
mod tests;
