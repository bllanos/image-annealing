use super::super::data::BufferData;
use super::super::dimensions::BufferDimensions;
use super::super::BindableBuffer;
use std::convert::TryInto;
use std::marker::PhantomData;

pub struct UniformBufferData<T>(BufferData, PhantomData<T>);

impl<T: bytemuck::Pod> UniformBufferData<T> {
    const SIZE: usize = std::mem::size_of::<T>();

    pub fn new(device: &wgpu::Device, label: Option<&str>) -> Self {
        let buffer_dimensions = BufferDimensions::new_buffer(1, Self::SIZE);
        Self(
            BufferData::create_uniform_buffer(device, &buffer_dimensions, label),
            PhantomData,
        )
    }

    pub fn load(&self, queue: &wgpu::Queue, data: &T) {
        queue.write_buffer(self.0.buffer(), 0, bytemuck::cast_slice(&[*data]));
    }

    pub fn binding_description(&self) -> wgpu::BindingType {
        wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: Some(wgpu::BufferSize::new(Self::SIZE.try_into().unwrap()).unwrap()),
        }
    }
}

impl<T> BindableBuffer for UniformBufferData<T> {
    fn binding_resource(&self) -> wgpu::BindingResource {
        self.0.buffer().as_entire_binding()
    }
}
