use super::super::super::output::conversion::VectorFieldEntryComponent;
use super::{Texture, TextureData, TextureDatatype};
use crate::{DisplacementGoal, ImageDimensions};
use core::num::NonZeroU32;
use std::convert::{TryFrom, TryInto};

pub struct DisplacementGoalTexture {}

impl TextureDatatype for DisplacementGoalTexture {
    type Component = VectorFieldEntryComponent;
    const N_COMPONENTS: usize = 2;

    const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Uint;
}

pub struct DisplacementGoalInputTexture(TextureData);

impl Texture for DisplacementGoalInputTexture {
    fn view(&self) -> &wgpu::TextureView {
        &self.0.view
    }
    fn dimensions(&self) -> wgpu::Extent3d {
        self.0.dimensions
    }
    fn copy_view(&self) -> wgpu::ImageCopyTexture {
        self.0.copy_view()
    }
    fn binding_description() -> wgpu::BindingType {
        super::make_read_texture_binding_description::<DisplacementGoalTexture>(
            wgpu::TextureSampleType::Uint,
        )
    }
}

impl DisplacementGoalInputTexture {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        Self(TextureData::create_read_texture(
            device,
            image_dimensions,
            DisplacementGoalTexture::FORMAT,
            Some("displacement_goal_input_texture"),
            Some("displacement_goal_input_texture_view"),
        ))
    }

    pub fn load(&self, queue: &wgpu::Queue, displacement_goal: &DisplacementGoal) {
        let dimensions = self.dimensions();
        TextureData::assert_same_image_dimensions(&self.0, displacement_goal.as_ref());

        queue.write_texture(
            self.copy_view(),
            bytemuck::cast_slice(displacement_goal.as_raw_slice()),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(
                    (<DisplacementGoalTexture as TextureDatatype>::PIXEL_SIZE
                        * <usize as TryFrom<u32>>::try_from(dimensions.width).unwrap())
                    .try_into()
                    .unwrap(),
                ),
                rows_per_image: NonZeroU32::new(dimensions.height),
            },
            dimensions,
        );
    }
}
