use super::texture::{Texture, TEXTURE_ARRAY_LAYERS};
use crate::image_utils::ImageDimensions;
use core::future::Future;
use core::num::NonZeroU32;
use std::convert::TryInto;
use std::pin::Pin;

mod lossless_image;
mod permutation;

pub use lossless_image::LosslessImageOutputBuffer;
pub use permutation::PermutationOutputBuffer;

/// From https://github.com/gfx-rs/wgpu-rs/blob/master/examples/capture/main.rs
#[derive(Copy, Clone)]
pub struct TextureCopyBufferDimensions {
    pub width: usize,
    pub height: usize,
    pub unpadded_bytes_per_row: usize,
    pub padded_bytes_per_row: usize,
}

impl TextureCopyBufferDimensions {
    fn new(sz: &ImageDimensions, bytes_per_pixel: usize) -> Self {
        let width = sz.width();
        let unpadded_bytes_per_row = width * bytes_per_pixel;
        let align: usize = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT.try_into().unwrap();
        let padded_bytes_per_row_padding = (align - unpadded_bytes_per_row % align) % align;
        let padded_bytes_per_row = unpadded_bytes_per_row + padded_bytes_per_row_padding;
        Self {
            width,
            height: sz.height(),
            unpadded_bytes_per_row,
            padded_bytes_per_row,
        }
    }
}

type BufferFuture = Pin<Box<dyn Future<Output = Result<(), wgpu::BufferAsyncError>> + Send>>;

pub struct MappedBuffer<'a> {
    buffer_slice: wgpu::BufferSlice<'a>,
    buffer_future: Option<BufferFuture>,
    buffer_dimensions: &'a TextureCopyBufferDimensions,
    buffer: &'a wgpu::Buffer,
}

impl<'a> MappedBuffer<'a> {
    fn new(
        slice: wgpu::BufferSlice<'a>,
        buffer_future: BufferFuture,
        buffer_dimensions: &'a TextureCopyBufferDimensions,
        buffer: &'a wgpu::Buffer,
    ) -> Self {
        Self {
            buffer_slice: slice,
            buffer_future: Some(buffer_future),
            buffer_dimensions,
            buffer,
        }
    }

    pub fn collect_mapped_buffer(&mut self) -> wgpu::BufferView {
        let fut = self
            .buffer_future
            .take()
            .expect("buffer data has already been collected");
        futures::executor::block_on(async move { fut.await.unwrap() });
        self.buffer_slice.get_mapped_range()
    }

    pub fn buffer_dimensions(&self) -> TextureCopyBufferDimensions {
        *(self.buffer_dimensions)
    }
}

impl Drop for MappedBuffer<'_> {
    fn drop(&mut self) {
        self.buffer.unmap(); // Free host memory
    }
}

pub trait ReadMappableBuffer {
    fn request_map_read(&self) -> MappedBuffer;
}

struct TextureCopyBufferData {
    buffer_dimensions: TextureCopyBufferDimensions,
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
        let buffer_dimensions = TextureCopyBufferDimensions::new(image_dimensions, bytes_per_pixel);
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label,
            size: (buffer_dimensions.padded_bytes_per_row * buffer_dimensions.height)
                .try_into()
                .unwrap(),
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
            buffer.buffer_dimensions.width == dimensions.width.try_into().unwrap()
                && buffer.buffer_dimensions.height == dimensions.height.try_into().unwrap()
                && TEXTURE_ARRAY_LAYERS == dimensions.depth_or_array_layers.try_into().unwrap()
        );
    }

    fn copy_view(&self) -> wgpu::ImageCopyBuffer {
        create_buffer_copy_view(&self.buffer, &self.buffer_dimensions)
    }
}

impl ReadMappableBuffer for TextureCopyBufferData {
    fn request_map_read(&self) -> MappedBuffer {
        let buffer_slice = self.buffer.slice(..);
        let buffer_future = Box::pin(buffer_slice.map_async(wgpu::MapMode::Read));
        MappedBuffer::new(
            buffer_slice,
            buffer_future,
            &self.buffer_dimensions,
            &self.buffer,
        )
    }
}

fn create_buffer_copy_view<'a, 'b>(
    buffer: &'a wgpu::Buffer,
    dimensions: &'b TextureCopyBufferDimensions,
) -> wgpu::ImageCopyBuffer<'a> {
    wgpu::ImageCopyBuffer {
        buffer,
        layout: wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(
                NonZeroU32::new(dimensions.padded_bytes_per_row.try_into().unwrap()).unwrap(),
            ),
            rows_per_image: None,
        },
    }
}
