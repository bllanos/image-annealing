use super::format::{VectorFieldImageBuffer, VectorFieldImageBufferPixel};
use crate::ImageDimensions;
use image::Rgba;

pub type VectorFieldEntryComponent = i16;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct VectorFieldEntry(pub VectorFieldEntryComponent, pub VectorFieldEntryComponent);

impl VectorFieldEntry {
    pub fn from_pixel(px: &VectorFieldImageBufferPixel) -> Self {
        VectorFieldEntry(
            VectorFieldEntryComponent::from_be_bytes([px[0], px[1]]),
            VectorFieldEntryComponent::from_be_bytes([px[2], px[3]]),
        )
    }

    pub fn to_pixel(&self) -> VectorFieldImageBufferPixel {
        let VectorFieldEntry(delta_x, delta_y) = self;
        let first = delta_x.to_be_bytes();
        let second = delta_y.to_be_bytes();
        Rgba([first[0], first[1], second[0], second[1]])
    }
}

pub fn to_vec(image: &VectorFieldImageBuffer) -> Vec<VectorFieldEntry> {
    image
        .enumerate_pixels()
        .map(|(.., px)| VectorFieldEntry::from_pixel(px))
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
