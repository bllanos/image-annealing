use crate::image_utils::ImageDimensions;
use std::marker::PhantomData;

mod lossless_image;
mod permutation;

pub use lossless_image::LosslessImageInputTexture;
pub use lossless_image::LosslessImageOutputTexture;
pub use lossless_image::LosslessImageTexture;
pub use permutation::PermutationInputTexture;
pub use permutation::PermutationOutputTexture;
pub use permutation::PermutationTexture;

pub trait TextureDatatype {
    type Component;
    fn n_components() -> usize;
    fn format() -> wgpu::TextureFormat;
    fn component_size() -> usize {
        std::mem::size_of::<Self::Component>()
    }
    fn pixel_size() -> usize {
        Self::n_components() * Self::component_size()
    }
    fn view_dimension() -> wgpu::TextureViewDimension {
        wgpu::TextureViewDimension::D2
    }
}

pub struct Texture<T> {
    dimensions: wgpu::Extent3d,
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    phantom: PhantomData<T>,
}

const TEXTURE_DIMENSION: wgpu::TextureDimension = wgpu::TextureDimension::D2;
const TEXTURE_ARRAY_LAYERS: usize = 1;

impl<T> Texture<T> {
    fn create_texture(
        device: &wgpu::Device,
        image_dimensions: &ImageDimensions,
        format: wgpu::TextureFormat,
        usage: wgpu::TextureUsage,
        label: Option<&str>,
        view_label: Option<&str>,
    ) -> Self {
        let dimensions = wgpu::Extent3d {
            width: image_dimensions.width() as u32,
            height: image_dimensions.height() as u32,
            depth_or_array_layers: TEXTURE_ARRAY_LAYERS as u32,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size: dimensions,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TEXTURE_DIMENSION,
            format,
            usage,
        });
        let view = create_texture_view(&texture, view_label);
        Self {
            dimensions,
            texture,
            view,
            phantom: PhantomData,
        }
    }

    fn create_storage_texture(
        device: &wgpu::Device,
        image_dimensions: &ImageDimensions,
        format: wgpu::TextureFormat,
        is_input: bool,
        label: Option<&str>,
        view_label: Option<&str>,
    ) -> Self {
        let copy_usage = if is_input {
            wgpu::TextureUsage::COPY_DST
        } else {
            wgpu::TextureUsage::COPY_SRC
        };
        Self::create_texture(
            device,
            image_dimensions,
            format,
            wgpu::TextureUsage::STORAGE | copy_usage,
            label,
            view_label,
        )
    }

    pub fn view(&self) -> &wgpu::TextureView {
        &self.view
    }

    pub fn dimensions(&self) -> wgpu::Extent3d {
        self.dimensions
    }

    pub fn copy_view(&self) -> wgpu::ImageCopyTexture {
        create_texture_copy_view(&self.texture)
    }
}

fn create_texture_view(texture: &wgpu::Texture, label: Option<&str>) -> wgpu::TextureView {
    texture.create_view(&wgpu::TextureViewDescriptor {
        label,
        ..Default::default()
    })
}

fn create_texture_copy_view(texture: &wgpu::Texture) -> wgpu::ImageCopyTexture {
    wgpu::ImageCopyTexture {
        texture: &texture,
        mip_level: 0,
        origin: wgpu::Origin3d::ZERO,
    }
}
