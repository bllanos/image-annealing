use super::dimensions::BufferDimensions;
use core::future::Future;
use std::convert::TryInto;
use std::pin::Pin;

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
