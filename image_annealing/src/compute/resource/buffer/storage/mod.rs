use super::data::BufferData;
use super::dimensions::BufferDimensions;
use crate::compute::link::swap::SwapPass;
use crate::ImageDimensions;

pub struct CountSwapInputBuffer(BufferData);

impl CountSwapInputBuffer {
    fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        let buffer_dimensions =
            BufferDimensions::new_buffer(1, SwapPass::total_workgroups(image_dimensions));
        Self(BufferData::create_storage_buffer(
            device,
            &buffer_dimensions,
            Some("count_swap_input_buffer"),
        ))
    }
}

pub struct CountSwapOutputStorageBuffer(BufferData);

impl CountSwapOutputStorageBuffer {
    fn new(device: &wgpu::Device) -> Self {
        let buffer_dimensions = super::count_swap_output_buffer_size();
        Self(BufferData::create_output_storage_buffer(
            device,
            &buffer_dimensions,
            Some("count_swap_output_storage_buffer"),
        ))
    }
}
