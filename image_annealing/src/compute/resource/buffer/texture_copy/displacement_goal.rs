use super::super::super::texture::{
    DisplacementGoalOutputTexture, DisplacementGoalTexture, Texture, TextureDatatype,
};
use super::data::TextureCopyBufferData;
use crate::compute::device::{DeviceManager, DevicePollType};
use crate::compute::format::VectorFieldImageBufferComponent;
use crate::ImageDimensions;

type BufferElement = VectorFieldImageBufferComponent;

pub struct DisplacementGoalOutputBuffer(TextureCopyBufferData);

impl DisplacementGoalOutputBuffer {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        Self(TextureCopyBufferData::new(
            device,
            image_dimensions,
            DisplacementGoalTexture::PIXEL_SIZE,
            Some("displacement_goal_output_buffer"),
        ))
    }

    pub fn load(&self, encoder: &mut wgpu::CommandEncoder, displacement_goal: &DisplacementGoalOutputTexture) {
        TextureCopyBufferData::assert_same_dimensions(&self.0, displacement_goal);

        encoder.copy_texture_to_buffer(
            displacement_goal.copy_view(),
            self.0.copy_view(),
            displacement_goal.dimensions(),
        );
    }

    fn output_chunk_mapper(chunk: &[u8]) -> BufferElement {
        BufferElement::from_be_bytes(chunk.try_into().unwrap())
    }

    pub async fn collect(
        &self,
        device_manager: &DeviceManager,
        poll_type: DevicePollType,
    ) -> Vec<BufferElement> {
        self.0
            .collect_elements(
                std::mem::size_of::<BufferElement>(),
                Self::output_chunk_mapper,
                device_manager,
                poll_type,
            )
            .await
    }

    pub fn width(&self) -> usize {
        self.0.width()
    }

    pub fn height(&self) -> usize {
        self.0.height()
    }
}
