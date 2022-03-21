use super::super::data::BufferData;
use super::super::dimensions::BufferDimensions;
use super::super::map::{ChunkedMappedBuffer, ChunkedReadMappableBuffer};
use super::super::storage::CountSwapOutputStorageBuffer;
use crate::compute::link::swap::CountSwapOutput;
use crate::compute::operation::WorkgroupGridDimensions;
use std::convert::TryInto;

pub struct CountSwapOutputBuffer(BufferData);

impl CountSwapOutputBuffer {
    pub fn new(device: &wgpu::Device) -> Self {
        let buffer_dimensions = BufferDimensions::new_buffer(
            WorkgroupGridDimensions::count_swap().count(),
            <Self as ChunkedReadMappableBuffer>::Element::SIZE,
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

    fn output_chunk_mapper(chunk: &[u8]) -> <Self as ChunkedReadMappableBuffer>::Element {
        <Self as ChunkedReadMappableBuffer>::Element::from_ne_bytes(chunk.try_into().unwrap())
    }

    pub(in super::super) fn dimensions(&self) -> &BufferDimensions {
        self.0.dimensions()
    }
}

impl<'a> ChunkedReadMappableBuffer<'a> for CountSwapOutputBuffer {
    type Element = CountSwapOutput;

    fn request_map_read(&'a self) -> ChunkedMappedBuffer<'a, Self::Element> {
        self.0
            .request_chunked_map_read(Self::Element::SIZE, Self::output_chunk_mapper)
    }
}
