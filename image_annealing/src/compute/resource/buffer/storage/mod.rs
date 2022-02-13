use super::data::BufferData;
use super::dimensions::BufferDimensions;
use super::{InputBuffer, OutputBuffer};
use crate::compute::link::swap::SwapPass;
use crate::ImageDimensions;
use std::convert::TryInto;

fn make_storage_buffer_binding_description(
    read_only: bool,
    dimensions: &BufferDimensions,
) -> wgpu::BindingType {
    wgpu::BindingType::Buffer {
        ty: wgpu::BufferBindingType::Storage { read_only },
        has_dynamic_offset: false,
        min_binding_size: Some(
            wgpu::BufferSize::new(dimensions.byte_size().try_into().unwrap()).unwrap(),
        ),
    }
}

pub struct CountSwapInputBuffer(BufferData);

impl CountSwapInputBuffer {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        let buffer_dimensions =
            BufferDimensions::new_buffer(1, SwapPass::total_workgroups(image_dimensions));
        Self(BufferData::create_storage_buffer(
            device,
            &buffer_dimensions,
            Some("count_swap_input_buffer"),
        ))
    }
}

impl InputBuffer for CountSwapInputBuffer {
    fn input_binding_description(&self) -> wgpu::BindingType {
        make_storage_buffer_binding_description(true, self.0.dimensions())
    }
}

impl OutputBuffer for CountSwapInputBuffer {
    fn output_binding_description(&self) -> wgpu::BindingType {
        make_storage_buffer_binding_description(false, self.0.dimensions())
    }
}

pub struct CountSwapOutputStorageBuffer(BufferData);

impl CountSwapOutputStorageBuffer {
    pub fn new(device: &wgpu::Device) -> Self {
        let buffer_dimensions = super::count_swap_output_buffer_size();
        Self(BufferData::create_output_storage_buffer(
            device,
            &buffer_dimensions,
            Some("count_swap_output_storage_buffer"),
        ))
    }
}

impl OutputBuffer for CountSwapOutputStorageBuffer {
    fn output_binding_description(&self) -> wgpu::BindingType {
        make_storage_buffer_binding_description(false, self.0.dimensions())
    }
}
