use super::super::super::texture::{
    PermutationOutputTexture, PermutationTexture, Texture, TextureDatatype,
};
use super::super::data::BufferChunkMapper;
use super::data::TextureCopyBufferData;
use crate::compute::device::{DeviceManager, DevicePollType};
use crate::compute::format::VectorFieldImageBufferComponent;
use crate::ImageDimensions;

pub struct PermutationOutputBuffer(TextureCopyBufferData);

impl PermutationOutputBuffer {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        Self(TextureCopyBufferData::new(
            device,
            image_dimensions,
            PermutationTexture::PIXEL_SIZE,
            Some("permutation_output_buffer"),
        ))
    }

    pub fn load(&self, encoder: &mut wgpu::CommandEncoder, permutation: &PermutationOutputTexture) {
        TextureCopyBufferData::assert_same_dimensions(&self.0, permutation);

        encoder.copy_texture_to_buffer(
            permutation.copy_view(),
            self.0.copy_view(),
            permutation.dimensions(),
        );
    }

    pub async fn collect(
        &self,
        device_manager: &DeviceManager,
        poll_type: DevicePollType,
    ) -> Vec<<Self as BufferChunkMapper>::Value> {
        self.0
            .collect_elements::<Self>(device_manager, poll_type)
            .await
    }

    pub fn width(&self) -> usize {
        self.0.width()
    }

    pub fn height(&self) -> usize {
        self.0.height()
    }
}

impl BufferChunkMapper for PermutationOutputBuffer {
    type Value = VectorFieldImageBufferComponent;

    fn chunk_to_value(chunk: &[u8]) -> Self::Value {
        Self::Value::from_be_bytes(chunk.try_into().unwrap())
    }
}
