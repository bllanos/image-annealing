use super::Texture;
use super::TextureDatatype;
use crate::image_utils::validation::ValidatedPermutation;
use crate::image_utils::ImageDimensions;
use core::num::NonZeroU32;
use std::ops::Deref;

pub type PermutationTexture = Texture<super::super::Permutation>;

impl TextureDatatype for PermutationTexture {
    type Component = i16;
    fn n_components() -> usize {
        2
    }

    fn format() -> wgpu::TextureFormat {
        wgpu::TextureFormat::Rgba8Uint
    }
}

pub struct PermutationInputTexture(PermutationTexture);
pub struct PermutationOutputTexture(PermutationTexture);

impl PermutationInputTexture {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        Self(PermutationTexture::create_storage_texture(
            device,
            image_dimensions,
            PermutationTexture::format(),
            true,
            Some("permutation_input_texture"),
            Some("permutation_input_texture_view"),
        ))
    }

    pub fn load(&self, queue: &wgpu::Queue, permutation: &ValidatedPermutation) {
        let (width, height) = permutation.dimensions();
        let own_dimensions = self.dimensions();
        assert!(width == own_dimensions.width && height == own_dimensions.height);

        queue.write_texture(
            self.copy_view(),
            bytemuck::cast_slice(permutation.as_raw().as_slice()),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(
                    <PermutationTexture as TextureDatatype>::pixel_size() as u32 * width,
                ),
                rows_per_image: NonZeroU32::new(height),
            },
            own_dimensions,
        );
    }
}

impl Deref for PermutationInputTexture {
    type Target = PermutationTexture;

    fn deref(&self) -> &PermutationTexture {
        &self.0
    }
}

impl PermutationOutputTexture {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        Self(PermutationTexture::create_storage_texture(
            device,
            image_dimensions,
            PermutationTexture::format(),
            false,
            Some("permutation_output_texture"),
            Some("permutation_output_texture_view"),
        ))
    }
}

impl Deref for PermutationOutputTexture {
    type Target = PermutationTexture;

    fn deref(&self) -> &PermutationTexture {
        &self.0
    }
}
