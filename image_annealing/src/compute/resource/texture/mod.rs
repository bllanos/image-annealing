use crate::image_utils::ImageDimensions;
use std::convert::TryInto;

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

pub trait Texture {
    fn view(&self) -> &wgpu::TextureView;
    fn dimensions(&self) -> wgpu::Extent3d;
    fn copy_view(&self) -> wgpu::ImageCopyTexture;
}

struct TextureData {
    dimensions: wgpu::Extent3d,
    texture: wgpu::Texture,
    view: wgpu::TextureView,
}

const TEXTURE_DIMENSION: wgpu::TextureDimension = wgpu::TextureDimension::D2;
pub(super) const TEXTURE_ARRAY_LAYERS: usize = 1;

impl TextureData {
    fn create_texture(
        device: &wgpu::Device,
        image_dimensions: &ImageDimensions,
        format: wgpu::TextureFormat,
        usage: wgpu::TextureUsage,
        label: Option<&str>,
        view_label: Option<&str>,
    ) -> Self {
        let dimensions = wgpu::Extent3d {
            width: image_dimensions.width().try_into().unwrap(),
            height: image_dimensions.height().try_into().unwrap(),
            depth_or_array_layers: TEXTURE_ARRAY_LAYERS.try_into().unwrap(),
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

    fn assert_same_dimensions(texture: &Self, dimensions: &ImageDimensions) {
        assert!(
            texture.dimensions.width == dimensions.width().try_into().unwrap()
                && texture.dimensions.height == dimensions.height().try_into().unwrap()
        );
    }

    fn copy_view(&self) -> wgpu::ImageCopyTexture {
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
