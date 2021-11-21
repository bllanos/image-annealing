use super::super::texture::{
    PermutationOutputTexture, PermutationTexture, Texture, TextureDatatype,
};
use super::{MappedBuffer, ReadMappableBuffer, TextureCopyBufferData};
use crate::ImageDimensions;

pub struct PermutationOutputBuffer(TextureCopyBufferData);

impl PermutationOutputBuffer {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        Self(TextureCopyBufferData::create_output_buffer(
            device,
            image_dimensions,
            PermutationTexture::PIXEL_SIZE,
            Some("permutation_output_buffer"),
        ))
    }

    pub fn load(&self, encoder: &mut wgpu::CommandEncoder, permutation: &PermutationOutputTexture) {
        TextureCopyBufferData::assert_same_dimensions(&self.0, permutation);

        encoder.copy_texture_to_buffer(
            permutation.copy_view(),
            self.0.copy_view(),
            permutation.dimensions(),
        );
    }
}

impl ReadMappableBuffer for PermutationOutputBuffer {
    fn request_map_read(&self) -> MappedBuffer {
        self.0.request_map_read()
    }
}
