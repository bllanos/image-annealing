use super::super::resource::texture::{PermutationTexture, TextureDatatype};
use super::format::PermutationImageBuffer;

pub type PermutationEntryComponent = <PermutationTexture as TextureDatatype>::Component;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct PermutationEntry(pub PermutationEntryComponent, pub PermutationEntryComponent);

pub fn to_vec(image: &PermutationImageBuffer) -> Vec<PermutationEntry> {
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
