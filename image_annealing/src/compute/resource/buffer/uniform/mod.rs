use super::data::BufferData;
use super::dimensions::BufferDimensions;
use std::convert::TryInto;
use std::marker::PhantomData;

mod count_swap_input_layout;

pub use count_swap_input_layout::CountSwapInputLayoutBuffer;

struct UniformBufferData<T>(BufferData, PhantomData<T>);

impl<T: bytemuck::Pod> UniformBufferData<T> {
    const SIZE: usize = std::mem::size_of::<T>();

    fn new(device: &wgpu::Device, label: Option<&str>) -> Self {
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
