use super::UniformBufferData;
use crate::compute::link::swap::CountSwapInputLayout;

pub struct CountSwapInputLayoutBuffer(UniformBufferData<CountSwapInputLayout>);

impl CountSwapInputLayoutBuffer {
    pub fn new(device: &wgpu::Device) -> Self {
        Self(UniformBufferData::<CountSwapInputLayout>::new(
            device,
            Some("count_swap_input_layout_buffer"),
        ))
    }

    pub fn load(&self, queue: &wgpu::Queue, data: &CountSwapInputLayout) {
        self.0.load(queue, data);
    }
}
