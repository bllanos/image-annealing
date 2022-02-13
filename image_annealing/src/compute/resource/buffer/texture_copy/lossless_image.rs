use super::super::super::texture::{
    LosslessImageOutputTexture, LosslessImageTexture, Texture, TextureDatatype,
};
use super::super::map::{MappedBuffer, ReadMappableBuffer};
use super::data::TextureCopyBufferData;
use crate::compute::format::LosslessImageBufferComponent;
use crate::ImageDimensions;
use std::convert::TryInto;

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

    fn output_chunk_mapper(chunk: &[u8]) -> LosslessImageBufferComponent {
        let val = <LosslessImageTexture as TextureDatatype>::Component::from_ne_bytes(
            chunk.try_into().unwrap(),
        );
        val.try_into().unwrap_or(0)
    }
}

impl<'a> ReadMappableBuffer<'a> for LosslessImageOutputBuffer {
    type Element = LosslessImageBufferComponent;

    fn request_map_read(&'a self) -> MappedBuffer<'a, Self::Element> {
        self.0.request_map_read(
            <LosslessImageTexture as TextureDatatype>::COMPONENT_SIZE,
            Self::output_chunk_mapper,
        )
    }
}
