use super::dimensions::BufferDimensions;
use core::future::Future;
use std::convert::TryInto;
use std::pin::Pin;

pub trait MappedBuffer<'a, T> {
    fn collect_mapped_buffer(&mut self) -> Vec<T>;

    fn width(&self) -> u32;

    fn height(&self) -> u32;
}

type BufferFuture = Pin<Box<dyn Future<Output = Result<(), wgpu::BufferAsyncError>> + Send>>;

pub struct ChunkedMappedBuffer<'a, T> {
    buffer_slice: wgpu::BufferSlice<'a>,
    buffer_future: Option<BufferFuture>,
    buffer_dimensions: &'a BufferDimensions,
    buffer: &'a wgpu::Buffer,
    output_chunk_size: usize,
    output_chunk_mapper: fn(&[u8]) -> T,
}

impl<'a, T> ChunkedMappedBuffer<'a, T> {
    pub(super) fn new(
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
}

impl<'a, T> MappedBuffer<'a, T> for ChunkedMappedBuffer<'a, T> {
    fn collect_mapped_buffer(&mut self) -> Vec<T> {
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

    fn width(&self) -> u32 {
        self.buffer_dimensions.width().try_into().unwrap()
    }

    fn height(&self) -> u32 {
        self.buffer_dimensions.height().try_into().unwrap()
    }
}

impl<T> Drop for ChunkedMappedBuffer<'_, T> {
    fn drop(&mut self) {
        self.buffer.unmap(); // Free host memory
    }
}

pub struct PlainMappedBuffer<'a> {
    buffer_slice: wgpu::BufferSlice<'a>,
    buffer_future: Option<BufferFuture>,
    buffer_dimensions: &'a BufferDimensions,
    buffer: &'a wgpu::Buffer,
}

impl<'a> PlainMappedBuffer<'a> {
    pub(super) fn new(
        slice: wgpu::BufferSlice<'a>,
        buffer_future: BufferFuture,
        buffer_dimensions: &'a BufferDimensions,
        buffer: &'a wgpu::Buffer,
    ) -> Self {
        Self {
            buffer_slice: slice,
            buffer_future: Some(buffer_future),
            buffer_dimensions,
            buffer,
        }
    }
}

impl<'a> MappedBuffer<'a, u8> for PlainMappedBuffer<'a> {
    fn collect_mapped_buffer(&mut self) -> Vec<u8> {
        let fut = self
            .buffer_future
            .take()
            .expect("buffer data has already been collected");
        futures::executor::block_on(async move { fut.await.unwrap() });
        let data = self.buffer_slice.get_mapped_range();
        match self.buffer_dimensions.padding() {
            Some(padding) => data
                .chunks(padding.padded_bytes_per_row())
                .flat_map(|c| c[..padding.unpadded_bytes_per_row()].to_vec())
                .collect::<Vec<u8>>(),
            None => data.to_vec(),
        }
    }

    fn width(&self) -> u32 {
        self.buffer_dimensions.width().try_into().unwrap()
    }

    fn height(&self) -> u32 {
        self.buffer_dimensions.height().try_into().unwrap()
    }
}

impl Drop for PlainMappedBuffer<'_> {
    fn drop(&mut self) {
        self.buffer.unmap(); // Free host memory
    }
}

pub trait ChunkedReadMappableBuffer<'a> {
    type Element;

    fn request_map_read(&'a self) -> ChunkedMappedBuffer<'a, Self::Element>;
}

pub trait PlainReadMappableBuffer<'a> {
    fn request_map_read(&'a self) -> PlainMappedBuffer<'a>;
}
