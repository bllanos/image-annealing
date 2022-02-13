use super::super::data::BufferData;
use super::super::dimensions::BufferDimensions;
use super::super::CountSwapOutputBuffer;
use super::super::{InputBuffer, OutputBuffer};
use crate::compute::link::swap::{CountSwapOutputDataElement, SwapPass};
use crate::ImageDimensions;

pub struct CountSwapInputBuffer(BufferData);

impl CountSwapInputBuffer {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        let buffer_dimensions = BufferDimensions::new_buffer(
            SwapPass::total_workgroups(image_dimensions),
            std::mem::size_of::<CountSwapOutputDataElement>(),
        );
        Self(BufferData::create_storage_buffer(
            device,
            &buffer_dimensions,
            Some("count_swap_input_buffer"),
        ))
    }
}

impl InputBuffer for CountSwapInputBuffer {
    fn input_binding_description(&self) -> wgpu::BindingType {
        super::make_storage_buffer_binding_description(true, self.0.dimensions())
    }
}

impl OutputBuffer for CountSwapInputBuffer {
    fn output_binding_description(&self) -> wgpu::BindingType {
        super::make_storage_buffer_binding_description(false, self.0.dimensions())
    }
}

pub struct CountSwapOutputStorageBuffer(BufferData);

impl CountSwapOutputStorageBuffer {
    pub fn new(device: &wgpu::Device, size_reference: &CountSwapOutputBuffer) -> Self {
        Self(BufferData::create_output_storage_buffer(
            device,
            size_reference.dimensions(),
            Some("count_swap_output_storage_buffer"),
        ))
    }
}

impl OutputBuffer for CountSwapOutputStorageBuffer {
    fn output_binding_description(&self) -> wgpu::BindingType {
        super::make_storage_buffer_binding_description(false, self.0.dimensions())
    }
}
