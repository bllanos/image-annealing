use super::data::TextureData;
use super::{Texture, TextureDatatype};
use crate::ImageDimensions;
use core::num::NonZeroU32;
use image::DynamicImage;
use std::convert::{TryFrom, TryInto};

pub struct LosslessImageTexture {}

impl TextureDatatype for LosslessImageTexture {
    type Component = u32;
    const N_COMPONENTS: usize = 4;
    const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba32Uint;
}

pub struct LosslessImageInputTexture(TextureData);
pub struct LosslessImageOutputTexture(TextureData);

impl Texture for LosslessImageInputTexture {
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
        super::data::make_read_texture_binding_description::<LosslessImageTexture>(
            wgpu::TextureSampleType::Uint,
        )
    }
}

impl Texture for LosslessImageOutputTexture {
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
        super::data::make_write_texture_binding_description::<LosslessImageTexture>()
    }
}

impl LosslessImageInputTexture {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        Self(TextureData::create_read_texture(
            device,
            image_dimensions,
            LosslessImageTexture::FORMAT,
            Some("lossless_image_input_texture"),
            Some("lossless_image_input_texture_view"),
        ))
    }

    pub fn load(&self, queue: &wgpu::Queue, image: &DynamicImage) {
        let image_data = image.to_rgba16();
        let (width, height) = image_data.dimensions();
        let own_dimensions = self.dimensions();
        assert!(width == own_dimensions.width && height == own_dimensions.height);

        let mut texture_data = Vec::with_capacity((width * height).try_into().unwrap());
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
                    (<LosslessImageTexture as TextureDatatype>::PIXEL_SIZE
                        * <usize as TryFrom<u32>>::try_from(width).unwrap())
                    .try_into()
                    .unwrap(),
                ),
                rows_per_image: NonZeroU32::new(height),
            },
            own_dimensions,
        );
    }
}

impl LosslessImageOutputTexture {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        Self(TextureData::create_write_texture(
            device,
            image_dimensions,
            LosslessImageTexture::FORMAT,
            Some("lossless_image_output_texture"),
            Some("lossless_image_output_texture_view"),
        ))
    }
}
