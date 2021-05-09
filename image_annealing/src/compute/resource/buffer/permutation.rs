use super::super::texture::{PermutationOutputTexture, PermutationTexture, TextureDatatype};
use super::{OutputBuffer, TextureCopyBuffer};
use crate::image_utils::ImageDimensions;

pub struct PermutationOutput {}
impl OutputBuffer for PermutationOutput {}

pub type PermutationOutputBuffer = TextureCopyBuffer<PermutationOutput>;

impl PermutationOutputBuffer {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        Self::create_output_buffer(
            device,
            image_dimensions,
            PermutationTexture::pixel_size(),
            Some("permutation_output_buffer"),
        )
    }

    pub fn load(&self, encoder: &mut wgpu::CommandEncoder, permutation: &PermutationOutputTexture) {
        Self::assert_same_dimensions(self, permutation);

        encoder.copy_texture_to_buffer(
            permutation.copy_view(),
            self.copy_view(),
            permutation.dimensions(),
        );
    }
}
