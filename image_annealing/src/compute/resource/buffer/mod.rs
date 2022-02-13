mod data;
mod dimensions;
mod map;
mod staging;
mod storage;
mod texture_copy;
mod uniform;

pub use map::ReadMappableBuffer;
pub use staging::CountSwapOutputBuffer;
pub use storage::{CountSwapInputBuffer, CountSwapOutputStorageBuffer};
pub use texture_copy::LosslessImageOutputBuffer;
pub use texture_copy::PermutationOutputBuffer;
pub use uniform::CountSwapInputLayoutBuffer;

pub trait InputBuffer {
    fn input_binding_description(&self) -> wgpu::BindingType;
}

pub trait OutputBuffer {
    fn output_binding_description(&self) -> wgpu::BindingType;
}
