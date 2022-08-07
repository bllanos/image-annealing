mod data;
mod dimensions;
mod map;
mod staging;
mod storage;
mod texture_copy;
mod uniform;

pub use staging::CountSwapOutputBuffer;
pub use storage::{CountSwapInputBuffer, CountSwapOutputStorageBuffer};
pub use texture_copy::LosslessImageOutputBuffer;
pub use texture_copy::PermutationOutputBuffer;
pub use uniform::{CountSwapInputLayoutBuffer, SwapParametersBuffer};

pub trait BindableBuffer {
    fn binding_resource(&self) -> wgpu::BindingResource;
}

pub trait InputBuffer: BindableBuffer {
    fn input_binding_description(&self) -> wgpu::BindingType;
}

pub trait OutputBuffer: BindableBuffer {
    fn output_binding_description(&self) -> wgpu::BindingType;
}
