use super::super::texture::{PermutationTexture, TextureDatatype};
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
}
