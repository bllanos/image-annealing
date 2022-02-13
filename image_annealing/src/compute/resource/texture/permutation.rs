use super::super::super::output::conversion::VectorFieldEntryComponent;
use super::data::TextureData;
use super::{Texture, TextureDatatype};
use crate::{ImageDimensions, ValidatedPermutation};
use core::num::NonZeroU32;
use std::convert::{TryFrom, TryInto};

pub struct PermutationTexture {}

impl TextureDatatype for PermutationTexture {
    type Component = VectorFieldEntryComponent;
    const N_COMPONENTS: usize = 2;

    const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Uint;
}

pub struct PermutationInputTexture(TextureData);
pub struct PermutationOutputTexture(TextureData);

impl Texture for PermutationInputTexture {
    fn view(&self) -> &wgpu::TextureView {
        self.0.view()
    }
    fn dimensions(&self) -> wgpu::Extent3d {
        self.0.dimensions()
    }
    fn copy_view(&self) -> wgpu::ImageCopyTexture {
        self.0.copy_view()
    }
    fn binding_description() -> wgpu::BindingType {
        super::data::make_read_texture_binding_description::<PermutationTexture>(
            wgpu::TextureSampleType::Uint,
        )
    }
}

impl Texture for PermutationOutputTexture {
    fn view(&self) -> &wgpu::TextureView {
        self.0.view()
    }
    fn dimensions(&self) -> wgpu::Extent3d {
        self.0.dimensions()
    }
    fn copy_view(&self) -> wgpu::ImageCopyTexture {
        self.0.copy_view()
    }
    fn binding_description() -> wgpu::BindingType {
        super::data::make_write_texture_binding_description::<PermutationTexture>()
    }
}

impl PermutationInputTexture {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        Self(TextureData::create_read_texture(
            device,
            image_dimensions,
            PermutationTexture::FORMAT,
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
                    (<PermutationTexture as TextureDatatype>::PIXEL_SIZE
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
        Self(TextureData::create_write_texture(
            device,
            image_dimensions,
            PermutationTexture::FORMAT,
            Some("permutation_output_texture"),
            Some("permutation_output_texture_view"),
        ))
    }
}
