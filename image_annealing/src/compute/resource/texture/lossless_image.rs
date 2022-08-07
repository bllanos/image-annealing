use super::super::super::output::format::LosslessImage;
use super::data::TextureData;
use super::{Texture, TextureDatatype};
use crate::{ImageDimensions, ImageDimensionsHolder};
use std::num::NonZeroU32;

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

    pub fn load(&self, queue: &wgpu::Queue, image: &LosslessImage) {
        let image_dimensions = image.dimensions();
        let own_dimensions = self.dimensions();
        assert_eq!(image_dimensions, &own_dimensions);

        queue.write_texture(
            self.copy_view(),
            image.to_texture_data().as_slice(),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(
                    (<LosslessImageTexture as TextureDatatype>::PIXEL_SIZE
                        * image_dimensions.width())
                    .try_into()
                    .unwrap(),
                ),
                rows_per_image: NonZeroU32::new(own_dimensions.height),
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
