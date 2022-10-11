mod data;
mod displacement_goal;
mod lossless_image;
mod permutation;

pub(super) use data::TEXTURE_ARRAY_LAYERS;
pub use displacement_goal::{
    DisplacementGoalInputTexture, DisplacementGoalOutputTexture, DisplacementGoalTexture,
};
pub use lossless_image::{
    LosslessImageInputTexture, LosslessImageOutputTexture, LosslessImageTexture,
};
pub use permutation::{PermutationInputTexture, PermutationOutputTexture, PermutationTexture};

pub trait TextureDatatype {
    type Component;
    const N_COMPONENTS: usize;
    const FORMAT: wgpu::TextureFormat;
    const COMPONENT_SIZE: usize = std::mem::size_of::<Self::Component>();
    const PIXEL_SIZE: usize = Self::N_COMPONENTS * Self::COMPONENT_SIZE;
    const VIEW_DIMENSION: wgpu::TextureViewDimension = wgpu::TextureViewDimension::D2;
}

pub trait Texture {
    fn view(&self) -> &wgpu::TextureView;
    fn dimensions(&self) -> wgpu::Extent3d;
    fn copy_view(&self) -> wgpu::ImageCopyTexture;
    fn binding_description() -> wgpu::BindingType;
}
