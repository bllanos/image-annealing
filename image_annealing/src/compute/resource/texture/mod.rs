use crate::ImageDimensions;
use image::GenericImageView;
use std::convert::TryInto;

mod displacement_goal;
mod lossless_image;
mod permutation;

pub use displacement_goal::DisplacementGoalInputTexture;
pub use lossless_image::LosslessImageInputTexture;
pub use lossless_image::LosslessImageOutputTexture;
pub use lossless_image::LosslessImageTexture;
pub use permutation::PermutationInputTexture;
pub use permutation::PermutationOutputTexture;
pub use permutation::PermutationTexture;

pub trait TextureDatatype {
    type Component;
    const N_COMPONENTS: usize;
    const FORMAT: wgpu::TextureFormat;
    const COMPONENT_SIZE: usize = std::mem::size_of::<Self::Component>();
    const PIXEL_SIZE: usize = Self::N_COMPONENTS * Self::COMPONENT_SIZE;
    const VIEW_DIMENSION: wgpu::TextureViewDimension = wgpu::TextureViewDimension::D2;
}

fn make_write_texture_binding_description<T: TextureDatatype>() -> wgpu::BindingType {
    wgpu::BindingType::StorageTexture {
        access: wgpu::StorageTextureAccess::WriteOnly,
        format: <T as TextureDatatype>::FORMAT,
        view_dimension: <T as TextureDatatype>::VIEW_DIMENSION,
    }
}

fn make_read_texture_binding_description<T: TextureDatatype>(
    sample_type: wgpu::TextureSampleType,
) -> wgpu::BindingType {
    wgpu::BindingType::Texture {
        sample_type,
        view_dimension: <T as TextureDatatype>::VIEW_DIMENSION,
        multisampled: false,
    }
}

pub trait Texture {
    fn view(&self) -> &wgpu::TextureView;
    fn dimensions(&self) -> wgpu::Extent3d;
    fn copy_view(&self) -> wgpu::ImageCopyTexture;
    fn binding_description() -> wgpu::BindingType;
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
        usage: wgpu::TextureUsages,
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

    fn create_write_texture(
        device: &wgpu::Device,
        image_dimensions: &ImageDimensions,
        format: wgpu::TextureFormat,
        label: Option<&str>,
        view_label: Option<&str>,
    ) -> Self {
        Self::create_texture(
            device,
            image_dimensions,
            format,
            wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
            label,
            view_label,
        )
    }

    fn create_read_texture(
        device: &wgpu::Device,
        image_dimensions: &ImageDimensions,
        format: wgpu::TextureFormat,
        label: Option<&str>,
        view_label: Option<&str>,
    ) -> Self {
        Self::create_texture(
            device,
            image_dimensions,
            format,
            wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
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

    fn assert_same_image_dimensions<T>(texture: &Self, image: &T)
    where
        T: GenericImageView,
    {
        Self::assert_same_dimensions(texture, &ImageDimensions::from_image(image).unwrap());
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
        texture,
        mip_level: 0,
        origin: wgpu::Origin3d::ZERO,
        aspect: wgpu::TextureAspect::All,
    }
}
