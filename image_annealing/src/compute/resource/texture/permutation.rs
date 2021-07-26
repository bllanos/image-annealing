use super::{Texture, TextureData, TextureDatatype};
use crate::image_utils::validation::ValidatedPermutation;
use crate::image_utils::ImageDimensions;
use core::num::NonZeroU32;
use std::convert::{TryFrom, TryInto};

pub struct PermutationTexture {}

impl TextureDatatype for PermutationTexture {
    type Component = i16;
    fn n_components() -> usize {
        2
    }

    fn format() -> wgpu::TextureFormat {
        wgpu::TextureFormat::Rgba8Uint
    }
}

pub struct PermutationInputTexture(TextureData);
pub struct PermutationOutputTexture(TextureData);

impl Texture for PermutationInputTexture {
    fn view(&self) -> &wgpu::TextureView {
        &self.0.view
    }
    fn dimensions(&self) -> wgpu::Extent3d {
        self.0.dimensions
    }
    fn copy_view(&self) -> wgpu::ImageCopyTexture {
        self.0.copy_view()
    }
}

impl Texture for PermutationOutputTexture {
    fn view(&self) -> &wgpu::TextureView {
        &self.0.view
    }
    fn dimensions(&self) -> wgpu::Extent3d {
        self.0.dimensions
    }
    fn copy_view(&self) -> wgpu::ImageCopyTexture {
        self.0.copy_view()
    }
}

impl PermutationInputTexture {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        Self(TextureData::create_storage_texture(
            device,
            image_dimensions,
            PermutationTexture::format(),
            true,
            Some("permutation_input_texture"),
            Some("permutation_input_texture_view"),
        ))
    }

    pub fn load(&self, queue: &wgpu::Queue, permutation: &ValidatedPermutation) {
        let dimensions = self.dimensions();
        TextureData::assert_same_dimensions(&self.0, &permutation.dimensions());

        queue.write_texture(
            self.copy_view(),
            bytemuck::cast_slice(permutation.as_raw_slice()),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(
                    (<PermutationTexture as TextureDatatype>::pixel_size()
                        * <usize as TryFrom<u32>>::try_from(dimensions.width).unwrap())
                    .try_into()
                    .unwrap(),
                ),
                rows_per_image: NonZeroU32::new(dimensions.height),
            },
            dimensions,
        );
    }

    pub fn copy(&self, encoder: &mut wgpu::CommandEncoder, permutation: &PermutationOutputTexture) {
        let own_dimensions = self.dimensions();
        assert!(own_dimensions == permutation.dimensions());

        encoder.copy_texture_to_texture(permutation.copy_view(), self.copy_view(), own_dimensions);
    }
}

impl PermutationOutputTexture {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        Self(TextureData::create_storage_texture(
            device,
            image_dimensions,
            PermutationTexture::format(),
            false,
            Some("permutation_output_texture"),
            Some("permutation_output_texture_view"),
        ))
    }
}
