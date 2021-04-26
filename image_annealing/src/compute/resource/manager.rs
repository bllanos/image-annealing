use super::buffer::PermutationOutputBuffer;
use super::texture::PermutationOutputTexture;
use crate::image_utils::ImageDimensions;

pub struct ResourceManager {
    permutation_output_buffer: PermutationOutputBuffer,
    permutation_output_texture: PermutationOutputTexture,
}

impl ResourceManager {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        Self {
            permutation_output_buffer: PermutationOutputBuffer::new(device, image_dimensions),
            permutation_output_texture: PermutationOutputTexture::new(device, image_dimensions),
        }
    }

    pub fn permutation_output_texture(&self) -> &PermutationOutputTexture {
        &self.permutation_output_texture
    }

    pub fn permutation_output_buffer(&self) -> &PermutationOutputBuffer {
        &self.permutation_output_buffer
    }
}
