use super::super::super::texture::{Texture, TEXTURE_ARRAY_LAYERS};
use super::super::data::BufferData;
use super::super::dimensions::BufferDimensions;
use super::super::map::{ChunkedMappedBuffer, PlainMappedBuffer};
use crate::ImageDimensions;
use std::num::NonZeroU32;

pub struct TextureCopyBufferData(BufferData);

impl TextureCopyBufferData {
    pub fn new(
        device: &wgpu::Device,
        image_dimensions: &ImageDimensions,
        bytes_per_pixel: usize,
        label: Option<&str>,
    ) -> Self {
        let buffer_dimensions =
            BufferDimensions::new_texture_copy(image_dimensions, bytes_per_pixel);
        Self(BufferData::create_output_buffer(
            device,
            &buffer_dimensions,
            label,
        ))
    }

    pub fn assert_same_dimensions<U: Texture>(buffer: &Self, texture: &U) {
        let dimensions = texture.dimensions();
        let buffer_dimensions = buffer.0.dimensions();
        assert!(
            buffer_dimensions.width() == dimensions.width.try_into().unwrap()
                && buffer_dimensions.height() == dimensions.height.try_into().unwrap()
                && TEXTURE_ARRAY_LAYERS == dimensions.depth_or_array_layers.try_into().unwrap()
        );
    }

    pub fn copy_view(&self) -> wgpu::ImageCopyBuffer {
        wgpu::ImageCopyBuffer {
            buffer: self.0.buffer(),
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(
                    NonZeroU32::new(
                        self.0
                            .dimensions()
                            .padding()
                            .unwrap()
                            .padded_bytes_per_row()
                            .try_into()
                            .unwrap(),
                    )
                    .unwrap(),
                ),
                rows_per_image: None,
            },
        }
    }

    pub fn request_chunked_map_read<T>(
        &self,
        output_chunk_size: usize,
        output_chunk_mapper: fn(&[u8]) -> T,
    ) -> ChunkedMappedBuffer<T> {
        self.0
            .request_chunked_map_read(output_chunk_size, output_chunk_mapper)
    }

    pub fn request_plain_map_read(&self) -> PlainMappedBuffer {
        self.0.request_plain_map_read()
    }
}
