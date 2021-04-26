use super::Texture;
use super::TextureDatatype;
use crate::image_utils::ImageDimensions;
use std::ops::Deref;

pub type LosslessImageTexture = Texture<super::super::LosslessImage>;

impl TextureDatatype for LosslessImageTexture {
    type Component = u32;
    fn n_components() -> usize {
        4
    }

    fn format() -> wgpu::TextureFormat {
        wgpu::TextureFormat::Rgba32Uint
    }
}

pub struct LosslessImageInputTexture(LosslessImageTexture);
pub struct LosslessImageOutputTexture(LosslessImageTexture);

impl LosslessImageInputTexture {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        Self(LosslessImageTexture::create_storage_texture(
            device,
            image_dimensions,
            LosslessImageTexture::format(),
            true,
            Some("lossless_image_input_texture"),
            Some("lossless_image_input_texture_view"),
        ))
    }
}

impl Deref for LosslessImageInputTexture {
    type Target = LosslessImageTexture;

    fn deref(&self) -> &LosslessImageTexture {
        &self.0
    }
}

impl LosslessImageOutputTexture {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        Self(LosslessImageTexture::create_storage_texture(
            device,
            image_dimensions,
            LosslessImageTexture::format(),
            false,
            Some("lossless_image_output_texture"),
            Some("lossless_image_output_texture_view"),
        ))
    }
}

impl Deref for LosslessImageOutputTexture {
    type Target = LosslessImageTexture;

    fn deref(&self) -> &LosslessImageTexture {
        &self.0
    }
}
