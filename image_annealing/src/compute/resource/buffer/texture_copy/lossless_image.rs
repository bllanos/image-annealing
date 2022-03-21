use super::super::super::texture::{
    LosslessImageOutputTexture, LosslessImageTexture, Texture, TextureDatatype,
};
use super::super::map::{PlainMappedBuffer, PlainReadMappableBuffer};
use super::data::TextureCopyBufferData;
use crate::ImageDimensions;

pub struct LosslessImageOutputBuffer(TextureCopyBufferData);

impl LosslessImageOutputBuffer {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        Self(TextureCopyBufferData::new(
            device,
            image_dimensions,
            LosslessImageTexture::PIXEL_SIZE,
            Some("lossless_image_output_buffer"),
        ))
    }

    pub fn load(&self, encoder: &mut wgpu::CommandEncoder, image: &LosslessImageOutputTexture) {
        TextureCopyBufferData::assert_same_dimensions(&self.0, image);

        encoder.copy_texture_to_buffer(image.copy_view(), self.0.copy_view(), image.dimensions());
    }
}

impl<'a> PlainReadMappableBuffer<'a> for LosslessImageOutputBuffer {
    fn request_map_read(&'a self) -> PlainMappedBuffer<'a> {
        self.0.request_plain_map_read()
    }
}
