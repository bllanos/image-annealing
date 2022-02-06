use super::texture::{Texture, TEXTURE_ARRAY_LAYERS};
use crate::ImageDimensions;
use core::future::Future;
use core::num::NonZeroU32;
use std::convert::TryInto;
use std::pin::Pin;

mod dimensions;
mod lossless_image;
mod permutation;

use dimensions::BufferDimensions;
pub use lossless_image::LosslessImageOutputBuffer;
pub use permutation::PermutationOutputBuffer;

type BufferFuture = Pin<Box<dyn Future<Output = Result<(), wgpu::BufferAsyncError>> + Send>>;

pub struct MappedBuffer<'a, T> {
    buffer_slice: wgpu::BufferSlice<'a>,
    buffer_future: Option<BufferFuture>,
    buffer_dimensions: &'a BufferDimensions,
    buffer: &'a wgpu::Buffer,
    output_chunk_size: usize,
    output_chunk_mapper: fn(&[u8]) -> T,
}

impl<'a, T> MappedBuffer<'a, T> {
    fn new(
        slice: wgpu::BufferSlice<'a>,
        buffer_future: BufferFuture,
        buffer_dimensions: &'a BufferDimensions,
        buffer: &'a wgpu::Buffer,
        output_chunk_size: usize,
        output_chunk_mapper: fn(&[u8]) -> T,
    ) -> Self {
        Self {
            buffer_slice: slice,
            buffer_future: Some(buffer_future),
            buffer_dimensions,
            buffer,
            output_chunk_size,
            output_chunk_mapper,
        }
    }

    pub fn collect_mapped_buffer(&mut self) -> Vec<T> {
        let fut = self
            .buffer_future
            .take()
            .expect("buffer data has already been collected");
        futures::executor::block_on(async move { fut.await.unwrap() });
        let data = self.buffer_slice.get_mapped_range();
        match self.buffer_dimensions.padding() {
            Some(padding) => data
                .chunks(padding.padded_bytes_per_row())
                .flat_map(|c| {
                    c[..padding.unpadded_bytes_per_row()].chunks_exact(self.output_chunk_size)
                })
                .map(self.output_chunk_mapper)
                .collect::<Vec<T>>(),
            None => data
                .chunks_exact(self.output_chunk_size)
                .map(self.output_chunk_mapper)
                .collect::<Vec<T>>(),
        }
    }

    pub fn width(&self) -> u32 {
        self.buffer_dimensions.width().try_into().unwrap()
    }

    pub fn height(&self) -> u32 {
        self.buffer_dimensions.height().try_into().unwrap()
    }
}

impl<T> Drop for MappedBuffer<'_, T> {
    fn drop(&mut self) {
        self.buffer.unmap(); // Free host memory
    }
}

pub trait ReadMappableBuffer<'a> {
    type Element;

    fn request_map_read(&'a self) -> MappedBuffer<'a, Self::Element>;
}

struct TextureCopyBufferData {
    buffer_dimensions: BufferDimensions,
    buffer: wgpu::Buffer,
}

impl TextureCopyBufferData {
    fn create_buffer(
        device: &wgpu::Device,
        image_dimensions: &ImageDimensions,
        bytes_per_pixel: usize,
        usage: wgpu::BufferUsages,
        label: Option<&str>,
    ) -> Self {
        let buffer_dimensions =
            BufferDimensions::new_texture_copy(image_dimensions, bytes_per_pixel);
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label,
            size: buffer_dimensions.byte_size().try_into().unwrap(),
            usage,
            mapped_at_creation: false,
        });
        Self {
            buffer_dimensions,
            buffer,
        }
    }

    fn create_output_buffer(
        device: &wgpu::Device,
        image_dimensions: &ImageDimensions,
        bytes_per_pixel: usize,
        label: Option<&str>,
    ) -> Self {
        Self::create_buffer(
            device,
            image_dimensions,
            bytes_per_pixel,
            wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            label,
        )
    }

    fn assert_same_dimensions<U: Texture>(buffer: &Self, texture: &U) {
        let dimensions = texture.dimensions();
        assert!(
            buffer.buffer_dimensions.width() == dimensions.width.try_into().unwrap()
                && buffer.buffer_dimensions.height() == dimensions.height.try_into().unwrap()
                && TEXTURE_ARRAY_LAYERS == dimensions.depth_or_array_layers.try_into().unwrap()
        );
    }

    fn copy_view(&self) -> wgpu::ImageCopyBuffer {
        create_buffer_copy_view(&self.buffer, &self.buffer_dimensions)
    }

    fn request_map_read<T>(
        &self,
        output_chunk_size: usize,
        output_chunk_mapper: fn(&[u8]) -> T,
    ) -> MappedBuffer<T> {
        let buffer_slice = self.buffer.slice(..);
        let buffer_future = Box::pin(buffer_slice.map_async(wgpu::MapMode::Read));
        MappedBuffer::new(
            buffer_slice,
            buffer_future,
            &self.buffer_dimensions,
            &self.buffer,
            output_chunk_size,
            output_chunk_mapper,
        )
    }
}

fn create_buffer_copy_view<'a, 'b>(
    buffer: &'a wgpu::Buffer,
    dimensions: &'b BufferDimensions,
) -> wgpu::ImageCopyBuffer<'a> {
    wgpu::ImageCopyBuffer {
        buffer,
        layout: wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(
                NonZeroU32::new(
                    dimensions
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
