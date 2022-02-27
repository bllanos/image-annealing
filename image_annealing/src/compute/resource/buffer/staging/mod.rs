use super::data::BufferData;
use super::dimensions::BufferDimensions;

mod count_swap;

pub use count_swap::CountSwapOutputBuffer;

fn assert_same_dimensions(buffer: &BufferData, dimensions: &BufferDimensions) {
    assert!(buffer.dimensions() == dimensions);
}
