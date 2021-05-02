use super::buffer::{LosslessImageOutputBuffer, PermutationOutputBuffer};
use super::texture::{
    LosslessImageInputTexture, LosslessImageOutputTexture, PermutationInputTexture,
    PermutationOutputTexture,
};
use crate::image_utils::ImageDimensions;

pub struct ResourceManager {
    permutation_input_texture: PermutationInputTexture,
    permutation_output_texture: PermutationOutputTexture,
    permutation_output_buffer: PermutationOutputBuffer,
    lossless_image_input_texture: LosslessImageInputTexture,
    lossless_image_output_texture: LosslessImageOutputTexture,
    lossless_image_output_buffer: LosslessImageOutputBuffer,
}

impl ResourceManager {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        Self {
            permutation_input_texture: PermutationInputTexture::new(device, image_dimensions),
            permutation_output_texture: PermutationOutputTexture::new(device, image_dimensions),
            permutation_output_buffer: PermutationOutputBuffer::new(device, image_dimensions),
            lossless_image_input_texture: LosslessImageInputTexture::new(device, image_dimensions),
            lossless_image_output_texture: LosslessImageOutputTexture::new(
                device,
                image_dimensions,
            ),
            lossless_image_output_buffer: LosslessImageOutputBuffer::new(device, image_dimensions),
        }
    }

    pub fn permutation_input_texture(&self) -> &PermutationInputTexture {
        &self.permutation_input_texture
    }

    pub fn permutation_output_texture(&self) -> &PermutationOutputTexture {
        &self.permutation_output_texture
    }

    pub fn permutation_output_buffer(&self) -> &PermutationOutputBuffer {
        &self.permutation_output_buffer
    }

    pub fn lossless_image_input_texture(&self) -> &LosslessImageInputTexture {
        &self.lossless_image_input_texture
    }

    pub fn lossless_image_output_texture(&self) -> &LosslessImageOutputTexture {
        &self.lossless_image_output_texture
    }

    pub fn lossless_image_output_buffer(&self) -> &LosslessImageOutputBuffer {
        &self.lossless_image_output_buffer
    }
}
