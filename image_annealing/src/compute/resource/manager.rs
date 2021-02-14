use super::buffer::PermutationOutputBuffer;
use super::texture::PermutationTexture;
use crate::image_utils::ImageDimensions;

pub struct ResourceManager {
    permutation_output_buffer: PermutationOutputBuffer,
    permutation_texture: PermutationTexture,
}

impl ResourceManager {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        Self {
            permutation_output_buffer: PermutationOutputBuffer::new(device, image_dimensions),
            permutation_texture: PermutationTexture::new(device, image_dimensions),
        }
    }

    pub fn permutation_texture(&self) -> &PermutationTexture {
        &self.permutation_texture
    }

    pub fn permutation_output_buffer(&self) -> &PermutationOutputBuffer {
        &self.permutation_output_buffer
    }
}
