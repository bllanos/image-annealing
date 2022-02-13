use super::super::super::texture::{
    PermutationOutputTexture, PermutationTexture, Texture, TextureDatatype,
};
use super::super::map::{MappedBuffer, ReadMappableBuffer};
use super::data::TextureCopyBufferData;
use crate::compute::format::VectorFieldImageBufferComponent;
use crate::ImageDimensions;
use std::convert::TryInto;

pub struct PermutationOutputBuffer(TextureCopyBufferData);

impl PermutationOutputBuffer {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        Self(TextureCopyBufferData::new(
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

    fn output_chunk_mapper(chunk: &[u8]) -> VectorFieldImageBufferComponent {
        VectorFieldImageBufferComponent::from_be_bytes(chunk.try_into().unwrap())
    }
}

impl<'a> ReadMappableBuffer<'a> for PermutationOutputBuffer {
    type Element = VectorFieldImageBufferComponent;

    fn request_map_read(&self) -> MappedBuffer<Self::Element> {
        self.0.request_map_read(
            std::mem::size_of::<Self::Element>(),
            Self::output_chunk_mapper,
        )
    }
}
