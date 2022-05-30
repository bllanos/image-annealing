use super::super::super::texture::{
    PermutationOutputTexture, PermutationTexture, Texture, TextureDatatype,
};
use super::super::map::{ChunkedMappedBuffer, ChunkedReadMappableBuffer};
use super::data::TextureCopyBufferData;
use crate::compute::format::VectorFieldImageBufferComponent;
use crate::ImageDimensions;

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

    fn output_chunk_mapper(chunk: &[u8]) -> <Self as ChunkedReadMappableBuffer>::Element {
        <Self as ChunkedReadMappableBuffer>::Element::from_be_bytes(chunk.try_into().unwrap())
    }
}

impl<'a> ChunkedReadMappableBuffer<'a> for PermutationOutputBuffer {
    type Element = VectorFieldImageBufferComponent;

    fn request_map_read(&self) -> ChunkedMappedBuffer<Self::Element> {
        self.0.request_chunked_map_read(
            std::mem::size_of::<Self::Element>(),
            Self::output_chunk_mapper,
        )
    }
}
