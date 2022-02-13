use super::data::BufferData;
use super::map::{MappedBuffer, ReadMappableBuffer};
use std::convert::TryInto;

pub struct CountSwapOutputBuffer(BufferData);

impl CountSwapOutputBuffer {
    pub fn new(device: &wgpu::Device) -> Self {
        let buffer_dimensions = super::count_swap_output_buffer_size();
        Self(BufferData::create_output_buffer(
            device,
            &buffer_dimensions,
            Some("count_swap_output_buffer"),
        ))
    }

    fn output_chunk_mapper(chunk: &[u8]) -> <CountSwapOutputBuffer as ReadMappableBuffer>::Element {
        <CountSwapOutputBuffer as ReadMappableBuffer>::Element::from_ne_bytes(
            chunk.try_into().unwrap(),
        )
    }
}

impl<'a> ReadMappableBuffer<'a> for CountSwapOutputBuffer {
    type Element = f32;

    fn request_map_read(&'a self) -> MappedBuffer<'a, Self::Element> {
        self.0.request_map_read(
            std::mem::size_of::<Self::Element>(),
            Self::output_chunk_mapper,
        )
    }
}
