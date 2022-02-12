use image_annealing_shaders::constant;

mod data;
mod dimensions;
mod map;
mod staging;
mod storage;
mod texture_copy;
mod uniform;

use dimensions::BufferDimensions;

pub use map::ReadMappableBuffer;
pub use staging::CountSwapOutputBuffer;
pub use storage::{CountSwapInputBuffer, CountSwapOutputStorageBuffer};
pub use texture_copy::LosslessImageOutputBuffer;
pub use texture_copy::PermutationOutputBuffer;
pub use uniform::CountSwapInputLayoutBuffer;

fn count_swap_output_buffer_size() -> BufferDimensions {
    BufferDimensions::new_buffer(constant::count_swap::N_CHANNEL, std::mem::size_of::<f32>())
}
