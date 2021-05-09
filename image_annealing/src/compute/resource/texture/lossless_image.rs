use super::Texture;
use super::TextureDatatype;
use crate::image_utils::ImageDimensions;
use core::num::NonZeroU32;
use image::DynamicImage;
use std::ops::Deref;

pub type LosslessImageTexture = Texture<super::super::LosslessImage>;

impl TextureDatatype for LosslessImageTexture {
    type Component = u32;
    fn n_components() -> usize {
        4
    }

    fn format() -> wgpu::TextureFormat {
        wgpu::TextureFormat::Rgba32Uint
    }
}

pub struct LosslessImageInputTexture(LosslessImageTexture);
pub struct LosslessImageOutputTexture(LosslessImageTexture);

impl LosslessImageInputTexture {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        Self(LosslessImageTexture::create_storage_texture(
            device,
            image_dimensions,
            LosslessImageTexture::format(),
            true,
            Some("lossless_image_input_texture"),
            Some("lossless_image_input_texture_view"),
        ))
    }

    pub fn load(&self, queue: &wgpu::Queue, image: &DynamicImage) {
        let image_data = image.to_rgba16();
        let (width, height) = image_data.dimensions();
        let own_dimensions = self.dimensions();
        assert!(width == own_dimensions.width && height == own_dimensions.height);

        let mut texture_data = Vec::with_capacity((width * height) as usize);
        image_data.enumerate_pixels().for_each(|(.., px)| {
            texture_data.push(px[0] as <LosslessImageTexture as TextureDatatype>::Component);
            texture_data.push(px[1] as <LosslessImageTexture as TextureDatatype>::Component);
            texture_data.push(px[2] as <LosslessImageTexture as TextureDatatype>::Component);
            texture_data.push(px[3] as <LosslessImageTexture as TextureDatatype>::Component);
        });

        queue.write_texture(
            self.copy_view(),
            bytemuck::cast_slice(texture_data.as_slice()),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(
                    <LosslessImageTexture as TextureDatatype>::pixel_size() as u32 * width,
                ),
                rows_per_image: NonZeroU32::new(height),
            },
            own_dimensions,
        );
    }
}

impl Deref for LosslessImageInputTexture {
    type Target = LosslessImageTexture;

    fn deref(&self) -> &LosslessImageTexture {
        &self.0
    }
}

impl LosslessImageOutputTexture {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        Self(LosslessImageTexture::create_storage_texture(
            device,
            image_dimensions,
            LosslessImageTexture::format(),
            false,
            Some("lossless_image_output_texture"),
            Some("lossless_image_output_texture_view"),
        ))
    }
}

impl Deref for LosslessImageOutputTexture {
    type Target = LosslessImageTexture;

    fn deref(&self) -> &LosslessImageTexture {
        &self.0
    }
}
