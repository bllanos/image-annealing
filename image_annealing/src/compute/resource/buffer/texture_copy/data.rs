use super::super::super::texture::{Texture, TEXTURE_ARRAY_LAYERS};
use super::super::data::BufferData;
use super::super::dimensions::BufferDimensions;
use crate::compute::device::{DeviceManager, DevicePollType};
use crate::ImageDimensions;
use std::num::NonZeroU32;

pub struct TextureCopyBufferData(BufferData);

impl TextureCopyBufferData {
    pub fn new(
        device: &wgpu::Device,
        image_dimensions: &ImageDimensions,
        bytes_per_pixel: usize,
        label: Option<&str>,
    ) -> Self {
        let buffer_dimensions =
            BufferDimensions::new_texture_copy(image_dimensions, bytes_per_pixel);
        Self(BufferData::create_output_buffer(
            device,
            &buffer_dimensions,
            label,
        ))
    }

    pub fn assert_same_dimensions<U: Texture>(buffer: &Self, texture: &U) {
        let dimensions = texture.dimensions();
        let buffer_dimensions = buffer.0.dimensions();
        assert!(
            buffer_dimensions.width() == dimensions.width.try_into().unwrap()
                && buffer_dimensions.height() == dimensions.height.try_into().unwrap()
                && TEXTURE_ARRAY_LAYERS == dimensions.depth_or_array_layers.try_into().unwrap()
        );
    }

    pub fn copy_view(&self) -> wgpu::ImageCopyBuffer {
        wgpu::ImageCopyBuffer {
            buffer: self.0.buffer(),
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(
                    NonZeroU32::new(
                        self.0
                            .dimensions()
                            .padding()
                            .unwrap()
                            .padded_bytes_per_row()
                            .try_into()
                            .unwrap(),
                    )
                    .unwrap(),
                ),
                rows_per_image: None,
            },
        }
    }

    pub async fn collect_elements<T>(
        &self,
        output_chunk_size: usize,
        output_chunk_mapper: fn(&[u8]) -> T,
        device_manager: &DeviceManager,
        poll_type: DevicePollType,
    ) -> Vec<T> {
        self.0
            .collect_elements(
                output_chunk_size,
                output_chunk_mapper,
                device_manager,
                poll_type,
            )
            .await
    }

    pub async fn collect_raw(
        &self,
        device_manager: &DeviceManager,
        poll_type: DevicePollType,
    ) -> Vec<u8> {
        self.0.collect_raw(device_manager, poll_type).await
    }

    pub fn width(&self) -> usize {
        self.0.dimensions().width()
    }

    pub fn height(&self) -> usize {
        self.0.dimensions().height()
    }
}
