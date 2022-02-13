use super::super::data::BufferData;
use super::super::dimensions::BufferDimensions;
use super::super::map::{MappedBuffer, ReadMappableBuffer};
use crate::compute::link::swap::CountSwapOutput;
use crate::compute::operation::WorkgroupGridDimensions;
use std::convert::TryInto;

pub struct CountSwapOutputBuffer(BufferData);

impl CountSwapOutputBuffer {
    pub fn new(device: &wgpu::Device) -> Self {
        let buffer_dimensions = BufferDimensions::new_buffer(
            WorkgroupGridDimensions::count_swap().count(),
            <Self as ReadMappableBuffer>::Element::SIZE,
        );
        Self(BufferData::create_output_buffer(
            device,
            &buffer_dimensions,
            Some("count_swap_output_buffer"),
        ))
    }

    fn output_chunk_mapper(chunk: &[u8]) -> <Self as ReadMappableBuffer>::Element {
        <Self as ReadMappableBuffer>::Element::from_ne_bytes(chunk.try_into().unwrap())
    }

    pub(in super::super) fn dimensions(&self) -> &BufferDimensions {
        self.0.dimensions()
    }
}

impl<'a> ReadMappableBuffer<'a> for CountSwapOutputBuffer {
    type Element = CountSwapOutput;

    fn request_map_read(&'a self) -> MappedBuffer<'a, Self::Element> {
        self.0
            .request_map_read(Self::Element::SIZE, Self::output_chunk_mapper)
    }
}
