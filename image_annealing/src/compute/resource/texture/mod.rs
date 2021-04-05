use crate::image_utils::ImageDimensions;
use std::marker::PhantomData;

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
pub type PermutationTexture = Texture<super::Permutation>;

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

impl PermutationTexture {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        Self::create_texture(
            device,
            image_dimensions,
            Self::format(),
            wgpu::TextureUsage::STORAGE | wgpu::TextureUsage::COPY_SRC,
            Some("permutation_texture"),
            Some("permutation_texture_view"),
        )
    }
}

impl TextureDatatype for PermutationTexture {
    type Component = i16;
    fn n_components() -> usize {
        2
    }

    /// Matches `PERMUTATION_FORMAT` in src/compute/operation/shader/glsl/defs.glsl
    fn format() -> wgpu::TextureFormat {
        wgpu::TextureFormat::Rgba8Uint
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
