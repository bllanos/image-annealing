use super::super::texture::{
    LosslessImageOutputTexture, LosslessImageTexture, Texture, TextureDatatype,
};
use super::{MappedBuffer, ReadMappableBuffer, TextureCopyBufferData};
use crate::image_utils::ImageDimensions;

pub struct LosslessImageOutputBuffer(TextureCopyBufferData);

impl LosslessImageOutputBuffer {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        Self(TextureCopyBufferData::create_output_buffer(
            device,
            image_dimensions,
            LosslessImageTexture::pixel_size(),
            Some("lossless_image_output_buffer"),
        ))
    }

    pub fn load(&self, encoder: &mut wgpu::CommandEncoder, image: &LosslessImageOutputTexture) {
        TextureCopyBufferData::assert_same_dimensions(&self.0, image);

        encoder.copy_texture_to_buffer(image.copy_view(), self.0.copy_view(), image.dimensions());
    }
}

impl ReadMappableBuffer for LosslessImageOutputBuffer {
    fn request_map_read(&self) -> MappedBuffer {
        self.0.request_map_read()
    }
}
