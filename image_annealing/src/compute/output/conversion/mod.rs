use super::super::resource::texture::{PermutationTexture, TextureDatatype};
use super::format::PermutationImageBuffer;
use crate::image_utils::ImageDimensions;
use std::convert::TryInto;

pub type PermutationEntryComponent = <PermutationTexture as TextureDatatype>::Component;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct PermutationEntry(pub PermutationEntryComponent, pub PermutationEntryComponent);

pub fn as_vec(image: &PermutationImageBuffer) -> Vec<PermutationEntry> {
    image
        .enumerate_pixels()
        .map(|(.., px)| -> PermutationEntry {
            PermutationEntry(
                PermutationEntryComponent::from_ne_bytes([px[0], px[1]]),
                PermutationEntryComponent::from_ne_bytes([px[2], px[3]]),
            )
        })
        .collect()
}

pub fn as_image(dimensions: &ImageDimensions, v: &Vec<PermutationEntry>) -> PermutationImageBuffer {
    if v.len() == dimensions.count() {
        let image_vec = v
            .into_iter()
            .flat_map(|PermutationEntry(delta_x, delta_y)| {
                let first = delta_x.to_ne_bytes();
                let second = delta_y.to_ne_bytes();
                [first, second].concat()
            })
            .collect();
        PermutationImageBuffer::from_vec(
            dimensions.width().try_into().unwrap(),
            dimensions.height().try_into().unwrap(),
            image_vec,
        )
        .unwrap()
    } else {
        panic!(format!(
            "A vector of length {} cannot be converted to an image of dimensions {}.",
            v.len(),
            dimensions
        ))
    }
}

#[cfg(test)]
mod tests;
