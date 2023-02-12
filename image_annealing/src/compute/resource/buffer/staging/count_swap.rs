use super::super::data::{BufferChunkMapper, BufferData};
use super::super::dimensions::BufferDimensions;
use super::super::storage::CountSwapOutputStorageBuffer;
use crate::compute::device::{DeviceManager, DevicePollType};
use crate::compute::link::swap::CountSwapOutput;
use crate::compute::operation::WorkgroupGridDimensions;

pub struct CountSwapOutputBuffer(BufferData);

impl CountSwapOutputBuffer {
    pub fn new(device: &wgpu::Device) -> Self {
        let buffer_dimensions = BufferDimensions::new_buffer(
            WorkgroupGridDimensions::count_swap().count(),
            <Self as BufferChunkMapper>::CHUNK_SIZE,
        );
        Self(BufferData::create_output_buffer(
            device,
            &buffer_dimensions,
            Some("count_swap_output_buffer"),
        ))
    }

    pub fn load(&self, encoder: &mut wgpu::CommandEncoder, buffer: &CountSwapOutputStorageBuffer) {
        super::assert_same_dimensions(&self.0, buffer.dimensions());

        encoder.copy_buffer_to_buffer(
            buffer.buffer(),
            0,
            self.0.buffer(),
            0,
            self.dimensions().byte_size().try_into().unwrap(),
        );
    }

    pub async fn collect(
        &self,
        device_manager: &DeviceManager,
        poll_type: DevicePollType,
    ) -> Vec<<Self as BufferChunkMapper>::Value> {
        self.0
            .collect_elements::<Self>(device_manager, poll_type)
            .await
    }

    pub(in super::super) fn dimensions(&self) -> &BufferDimensions {
        self.0.dimensions()
    }
}

impl BufferChunkMapper for CountSwapOutputBuffer {
    type Value = CountSwapOutput;

    const CHUNK_SIZE: usize = Self::Value::SIZE;

    fn chunk_to_value(chunk: &[u8]) -> Self::Value {
        Self::Value::from_ne_bytes(chunk.try_into().unwrap())
    }
}
