use super::texture::{PermutationTexture, TextureDatatype};
use crate::image_utils::ImageDimensions;
use core::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;

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
        let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as usize;
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

pub struct MappedBuffer<'a> {
    buffer_slice: wgpu::BufferSlice<'a>,
    buffer_future: Option<Pin<Box<dyn Future<Output = Result<(), wgpu::BufferAsyncError>> + Send>>>,
    buffer_dimensions: &'a TextureCopyBufferDimensions,
    buffer: &'a wgpu::Buffer,
}

impl<'a> MappedBuffer<'a> {
    pub fn new(
        slice: wgpu::BufferSlice<'a>,
        buffer_future: Pin<Box<dyn Future<Output = Result<(), wgpu::BufferAsyncError>> + Send>>,
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
        match self.buffer_future.take() {
            Some(fut) => {
                futures::executor::block_on(async move { fut.await.unwrap() });
                ()
            }
            None => panic!("The buffer data has already been collected."),
        }
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

pub trait MappableBuffer {
    fn request_map(&self) -> MappedBuffer;
}

pub struct TextureCopyBuffer<T> {
    buffer_dimensions: TextureCopyBufferDimensions,
    buffer: wgpu::Buffer,
    phantom: PhantomData<T>,
}
pub type PermutationStagingBuffer = TextureCopyBuffer<super::Permutation>;

impl<T> TextureCopyBuffer<T> {
    fn create_buffer(
        device: &wgpu::Device,
        image_dimensions: &ImageDimensions,
        bytes_per_pixel: usize,
        usage: wgpu::BufferUsage,
        label: Option<&str>,
    ) -> Self {
        let buffer_dimensions =
            TextureCopyBufferDimensions::new(&image_dimensions, bytes_per_pixel);
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label,
            size: (buffer_dimensions.padded_bytes_per_row * buffer_dimensions.height) as u64,
            usage,
            mapped_at_creation: false,
        });
        Self {
            buffer_dimensions,
            buffer,
            phantom: PhantomData,
        }
    }

    pub fn copy_view(&self) -> wgpu::BufferCopyView {
        create_buffer_copy_view(&self.buffer, &self.buffer_dimensions)
    }
}

impl<T> MappableBuffer for TextureCopyBuffer<T> {
    fn request_map(&self) -> MappedBuffer {
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

impl PermutationStagingBuffer {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        Self::create_buffer(
            device,
            image_dimensions,
            PermutationTexture::pixel_size(),
            wgpu::BufferUsage::MAP_READ
                | wgpu::BufferUsage::MAP_WRITE
                | wgpu::BufferUsage::COPY_SRC
                | wgpu::BufferUsage::COPY_DST,
            Some("permutation_staging_buffer"),
        )
    }
}

fn create_buffer_copy_view<'a, 'b>(
    buffer: &'a wgpu::Buffer,
    dimensions: &'b TextureCopyBufferDimensions,
) -> wgpu::BufferCopyView<'a> {
    wgpu::BufferCopyView {
        buffer,
        layout: wgpu::TextureDataLayout {
            offset: 0,
            bytes_per_row: dimensions.padded_bytes_per_row as u32,
            rows_per_image: 0,
        },
    }
}
