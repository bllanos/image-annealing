use super::format::VectorFieldImageBuffer;
use crate::image_utils::ImageDimensions;
use std::convert::TryInto;

pub type VectorFieldEntryComponent = i16;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct VectorFieldEntry(pub VectorFieldEntryComponent, pub VectorFieldEntryComponent);

pub fn to_vec(image: &VectorFieldImageBuffer) -> Vec<VectorFieldEntry> {
    image
        .enumerate_pixels()
        .map(|(.., px)| -> VectorFieldEntry {
            VectorFieldEntry(
                VectorFieldEntryComponent::from_be_bytes([px[0], px[1]]),
                VectorFieldEntryComponent::from_be_bytes([px[2], px[3]]),
            )
        })
        .collect()
}

pub fn to_image(dimensions: &ImageDimensions, v: &[VectorFieldEntry]) -> VectorFieldImageBuffer {
    if v.len() == dimensions.count() {
        let image_vec = v
            .iter()
            .flat_map(|VectorFieldEntry(delta_x, delta_y)| {
                let first = delta_x.to_be_bytes();
                let second = delta_y.to_be_bytes();
                [first, second].concat()
            })
            .collect();
        VectorFieldImageBuffer::from_vec(
            dimensions.width().try_into().unwrap(),
            dimensions.height().try_into().unwrap(),
            image_vec,
        )
        .unwrap()
    } else {
        panic!("vector length and image dimensions are incompatible.");
    }
}

#[cfg(test)]
mod tests;
