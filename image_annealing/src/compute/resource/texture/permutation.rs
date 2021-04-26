use super::Texture;
use super::TextureDatatype;
use crate::image_utils::ImageDimensions;
use std::ops::Deref;

pub type PermutationTexture = Texture<super::super::Permutation>;

impl TextureDatatype for PermutationTexture {
    type Component = i16;
    fn n_components() -> usize {
        2
    }

    fn format() -> wgpu::TextureFormat {
        wgpu::TextureFormat::Rgba8Uint
    }
}

pub struct PermutationInputTexture(PermutationTexture);
pub struct PermutationOutputTexture(PermutationTexture);

impl PermutationInputTexture {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        Self(PermutationTexture::create_storage_texture(
            device,
            image_dimensions,
            PermutationTexture::format(),
            true,
            Some("permutation_input_texture"),
            Some("permutation_input_texture_view"),
        ))
    }
}

impl Deref for PermutationInputTexture {
    type Target = PermutationTexture;

    fn deref(&self) -> &PermutationTexture {
        &self.0
    }
}

impl PermutationOutputTexture {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        Self(PermutationTexture::create_storage_texture(
            device,
            image_dimensions,
            PermutationTexture::format(),
            false,
            Some("permutation_output_texture"),
            Some("permutation_output_texture_view"),
        ))
    }
}

impl Deref for PermutationOutputTexture {
    type Target = PermutationTexture;

    fn deref(&self) -> &PermutationTexture {
        &self.0
    }
}
