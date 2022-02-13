use super::dimensions::BufferDimensions;
use std::convert::TryInto;

mod count_swap;

pub use count_swap::CountSwapInputBuffer;
pub use count_swap::CountSwapOutputStorageBuffer;

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
