use super::buffer::PermutationStagingBuffer;
use super::texture::PermutationTexture;
use crate::image_utils::ImageDimensions;

pub struct ResourceManager {
    permutation_staging_buffer: PermutationStagingBuffer,
    permutation_texture: PermutationTexture,
}

impl ResourceManager {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        Self {
            permutation_staging_buffer: PermutationStagingBuffer::new(device, image_dimensions),
            permutation_texture: PermutationTexture::new(device, image_dimensions),
        }
    }

    pub fn permutation_texture(&self) -> &PermutationTexture {
        &self.permutation_texture
    }

    pub fn permutation_staging_buffer(&self) -> &PermutationStagingBuffer {
        &self.permutation_staging_buffer
    }
}
