use image::Rgba;
use image_annealing::compute::format::LosslessImageBuffer;
use image_annealing::image_utils::ImageDimensions;
use std::convert::TryInto;

pub fn coordinates_to_colors(dimensions: &ImageDimensions) -> LosslessImageBuffer {
    LosslessImageBuffer::from_fn(
        dimensions.width() as u32,
        dimensions.height() as u32,
        |x, y| {
            Rgba([
                x.try_into().unwrap(),
                (x + 1).try_into().unwrap(),
                y.try_into().unwrap(),
                (y + 1).try_into().unwrap(),
            ])
        },
    )
}

pub fn coordinates_to_zero_alpha_colors(dimensions: &ImageDimensions) -> LosslessImageBuffer {
    LosslessImageBuffer::from_fn(
        dimensions.width() as u32,
        dimensions.height() as u32,
        |x, y| {
            Rgba([
                x.try_into().unwrap(),
                (x + 1).try_into().unwrap(),
                y.try_into().unwrap(),
                0,
            ])
        },
    )
}
