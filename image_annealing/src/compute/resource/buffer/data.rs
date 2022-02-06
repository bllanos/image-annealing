use std::convert::TryInto;

use super::dimensions::BufferDimensions;
use super::map::MappedBuffer;

pub struct BufferData {
    dimensions: BufferDimensions,
    buffer: wgpu::Buffer,
}

impl BufferData {
    pub fn create_buffer(
        device: &wgpu::Device,
        dimensions: &BufferDimensions,
        usage: wgpu::BufferUsages,
        label: Option<&str>,
    ) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label,
            size: dimensions.byte_size().try_into().unwrap(),
            usage,
            mapped_at_creation: false,
        });
        Self {
            dimensions: *dimensions,
            buffer,
        }
    }

    pub fn create_output_buffer(
        device: &wgpu::Device,
        dimensions: &BufferDimensions,
        label: Option<&str>,
    ) -> Self {
        Self::create_buffer(
            device,
            dimensions,
            wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            label,
        )
    }

    pub fn request_map_read<T>(
        &self,
        output_chunk_size: usize,
        output_chunk_mapper: fn(&[u8]) -> T,
    ) -> MappedBuffer<T> {
        let buffer_slice = self.buffer.slice(..);
        let buffer_future = Box::pin(buffer_slice.map_async(wgpu::MapMode::Read));
        MappedBuffer::new(
            buffer_slice,
            buffer_future,
            &self.dimensions,
            &self.buffer,
            output_chunk_size,
            output_chunk_mapper,
        )
    }

    pub fn dimensions(&self) -> &BufferDimensions {
        &self.dimensions
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }
}
