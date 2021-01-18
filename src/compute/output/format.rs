use image::ImageBuffer;

pub type PermutationImageBufferComponent = u8;
pub type PermutationImageBuffer =
    ImageBuffer<image::Rgba<PermutationImageBufferComponent>, Vec<PermutationImageBufferComponent>>;
