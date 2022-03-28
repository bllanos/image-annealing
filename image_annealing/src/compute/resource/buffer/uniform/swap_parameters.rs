use super::super::{BindableBuffer, InputBuffer};
use super::data::UniformBufferData;
use crate::compute::link::swap::SwapShaderParameters;

pub struct SwapParametersBuffer(UniformBufferData<SwapShaderParameters>);

impl SwapParametersBuffer {
    pub fn new(device: &wgpu::Device) -> Self {
        Self(UniformBufferData::<SwapShaderParameters>::new(
            device,
            Some("swap_parameters_buffer"),
        ))
    }

    pub fn load(&self, queue: &wgpu::Queue, data: &SwapShaderParameters) {
        self.0.load(queue, data);
    }
}

impl BindableBuffer for SwapParametersBuffer {
    fn binding_resource(&self) -> wgpu::BindingResource {
        self.0.binding_resource()
    }
}

impl InputBuffer for SwapParametersBuffer {
    fn input_binding_description(&self) -> wgpu::BindingType {
        self.0.binding_description()
    }
}
