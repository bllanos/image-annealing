use super::super::texture::{LosslessImageTexture, TextureDatatype};
use super::{OutputBuffer, TextureCopyBuffer};
use crate::image_utils::ImageDimensions;

pub struct LosslessImageOutput {}
impl OutputBuffer for LosslessImageOutput {}

pub type LosslessImageOutputBuffer = TextureCopyBuffer<LosslessImageOutput>;

impl LosslessImageOutputBuffer {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        Self::create_output_buffer(
            device,
            image_dimensions,
            LosslessImageTexture::pixel_size(),
            Some("lossless_image_output_buffer"),
        )
    }
}
