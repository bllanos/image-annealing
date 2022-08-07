use super::dimensions::BufferDimensions;
use super::map::BufferSliceMapFuture;
use crate::compute::device::{DeviceManager, DevicePollType};

pub struct BufferData {
    dimensions: BufferDimensions,
    buffer: wgpu::Buffer,
}

impl BufferData {
    fn create_buffer(
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

    pub fn create_storage_buffer(
        device: &wgpu::Device,
        dimensions: &BufferDimensions,
        label: Option<&str>,
    ) -> Self {
        Self::create_buffer(device, dimensions, wgpu::BufferUsages::STORAGE, label)
    }

    pub fn create_output_storage_buffer(
        device: &wgpu::Device,
        dimensions: &BufferDimensions,
        label: Option<&str>,
    ) -> Self {
        Self::create_buffer(
            device,
            dimensions,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            label,
        )
    }

    pub fn create_uniform_buffer(
        device: &wgpu::Device,
        dimensions: &BufferDimensions,
        label: Option<&str>,
    ) -> Self {
        Self::create_buffer(
            device,
            dimensions,
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            label,
        )
    }

    pub async fn collect_elements<T>(
        &self,
        output_chunk_size: usize,
        output_chunk_mapper: fn(&[u8]) -> T,
        device_manager: &DeviceManager,
        poll_type: DevicePollType,
    ) -> Vec<T> {
        let buffer_slice = self.buffer.slice(..);
        let fut = BufferSliceMapFuture::new(&buffer_slice, device_manager, poll_type);
        fut.await.unwrap();
        let data = buffer_slice.get_mapped_range();
        let output = match self.dimensions.padding() {
            Some(padding) => data
                .chunks(padding.padded_bytes_per_row())
                .flat_map(|c| c[..padding.unpadded_bytes_per_row()].chunks_exact(output_chunk_size))
                .map(output_chunk_mapper)
                .collect::<Vec<T>>(),
            None => data
                .chunks_exact(output_chunk_size)
                .map(output_chunk_mapper)
                .collect::<Vec<T>>(),
        };
        drop(data);
        self.buffer.unmap(); // Free host memory
        output
    }

    pub async fn collect_raw(
        &self,
        device_manager: &DeviceManager,
        poll_type: DevicePollType,
    ) -> Vec<u8> {
        let buffer_slice = self.buffer.slice(..);
        let fut = BufferSliceMapFuture::new(&buffer_slice, device_manager, poll_type);
        fut.await.unwrap();
        let data = buffer_slice.get_mapped_range();
        let output = match self.dimensions.padding() {
            Some(padding) => data
                .chunks(padding.padded_bytes_per_row())
                .flat_map(|c| c[..padding.unpadded_bytes_per_row()].to_vec())
                .collect::<Vec<u8>>(),
            None => data.to_vec(),
        };
        drop(data);
        self.buffer.unmap(); // Free host memory
        output
    }

    pub fn dimensions(&self) -> &BufferDimensions {
        &self.dimensions
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }
}
