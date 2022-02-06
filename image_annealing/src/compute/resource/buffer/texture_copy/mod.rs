use super::super::texture::{Texture, TEXTURE_ARRAY_LAYERS};
use super::data::BufferData;
use super::dimensions::BufferDimensions;
use super::map::MappedBuffer;
use crate::ImageDimensions;
use core::num::NonZeroU32;
use std::convert::TryInto;

mod lossless_image;
mod permutation;

pub use lossless_image::LosslessImageOutputBuffer;
pub use permutation::PermutationOutputBuffer;

struct TextureCopyBufferData(BufferData);

impl TextureCopyBufferData {
    fn new(
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

    fn assert_same_dimensions<U: Texture>(buffer: &Self, texture: &U) {
        let dimensions = texture.dimensions();
        let buffer_dimensions = buffer.0.dimensions();
        assert!(
            buffer_dimensions.width() == dimensions.width.try_into().unwrap()
                && buffer_dimensions.height() == dimensions.height.try_into().unwrap()
                && TEXTURE_ARRAY_LAYERS == dimensions.depth_or_array_layers.try_into().unwrap()
        );
    }

    fn copy_view(&self) -> wgpu::ImageCopyBuffer {
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

    fn request_map_read<T>(
        &self,
        output_chunk_size: usize,
        output_chunk_mapper: fn(&[u8]) -> T,
    ) -> MappedBuffer<T> {
        self.0
            .request_map_read(output_chunk_size, output_chunk_mapper)
    }
}
