use super::super::super::texture::{
    LosslessImageOutputTexture, LosslessImageTexture, Texture, TextureDatatype,
};
use super::data::TextureCopyBufferData;
use crate::compute::device::{DeviceManager, DevicePollType};
use crate::ImageDimensions;

pub struct LosslessImageOutputBuffer(TextureCopyBufferData);

impl LosslessImageOutputBuffer {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        Self(TextureCopyBufferData::new(
            device,
            image_dimensions,
            LosslessImageTexture::PIXEL_SIZE,
            Some("lossless_image_output_buffer"),
        ))
    }

    pub fn load(&self, encoder: &mut wgpu::CommandEncoder, image: &LosslessImageOutputTexture) {
        TextureCopyBufferData::assert_same_dimensions(&self.0, image);

        encoder.copy_texture_to_buffer(image.copy_view(), self.0.copy_view(), image.dimensions());
    }

    pub async fn collect(
        &self,
        device_manager: &DeviceManager,
        poll_type: DevicePollType,
    ) -> Vec<u8> {
        self.0.collect_raw(device_manager, poll_type).await
    }

    pub fn width(&self) -> usize {
        self.0.width()
    }

    pub fn height(&self) -> usize {
        self.0.height()
    }
}
