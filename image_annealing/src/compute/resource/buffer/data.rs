use super::dimensions::BufferDimensions;
use super::map::BufferSliceMapFuture;
use crate::compute::device::{DeviceManager, DevicePollType};

pub trait BufferChunkMapper {
    type Value;
    const CHUNK_SIZE: usize = std::mem::size_of::<Self::Value>();
    fn chunk_to_value(chunk: &[u8]) -> Self::Value;
}

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

    pub async fn collect_elements<T: BufferChunkMapper>(
        &self,
        device_manager: &DeviceManager,
        poll_type: DevicePollType,
    ) -> Vec<T::Value> {
        let buffer_slice = self.buffer.slice(..);
        BufferSliceMapFuture::new(&buffer_slice, device_manager, poll_type).await;
        let data = buffer_slice.get_mapped_range();
        let output = match self.dimensions.padding() {
            Some(padding) => data
                .chunks(padding.padded_bytes_per_row())
                .flat_map(|c| c[..padding.unpadded_bytes_per_row()].chunks_exact(T::CHUNK_SIZE))
                .map(T::chunk_to_value)
                .collect::<Vec<T::Value>>(),
            None => data
                .chunks_exact(T::CHUNK_SIZE)
                .map(T::chunk_to_value)
                .collect::<Vec<T::Value>>(),
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
        BufferSliceMapFuture::new(&buffer_slice, device_manager, poll_type).await;
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
