use super::super::device::DeviceManager;
use super::super::format::{ImageFormat, LosslessImage, VectorFieldImageBuffer};
use super::super::link::swap::SwapPassSequence;
use super::super::resource::buffer::{
    ChunkedReadMappableBuffer, MappedBuffer, PlainReadMappableBuffer,
};
use super::super::resource::manager::ResourceManager;
use super::pipeline::manager::PipelineManager;
use crate::image_utils::validation::{self};
use crate::{ImageDimensions, ValidatedPermutation};
use std::error::Error;

mod input;
mod output;
mod state;

pub use input::{PermuteOperationInput, SwapOperationInput};
pub use output::CountSwapOperationOutput;
use state::ResourceStateManager;

pub struct OperationManager {
    resources: ResourceManager,
    state: ResourceStateManager,
    pipelines: PipelineManager,
    image_dimensions: ImageDimensions,
}

impl OperationManager {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        let resources = ResourceManager::new(device, image_dimensions);
        let pipelines = PipelineManager::new(device, &resources);
        OperationManager {
            resources,
            state: ResourceStateManager::new(image_dimensions),
            pipelines,
            image_dimensions: *image_dimensions,
        }
    }

    pub fn count_swap(
        &mut self,
        device: &DeviceManager,
        sequence: SwapPassSequence,
    ) -> Result<(), Box<dyn Error>> {
        let mut encoder = device
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("count_swap_command_encoder"),
            });
        let queue = device.queue();
        let mut transaction = self.state.count_swap(&self.resources, queue, sequence)?;
        self.pipelines.count_swap(&mut encoder);
        queue.submit(Some(encoder.finish()));
        transaction.set_commit();
        Ok(())
    }

    pub fn create_permutation(&mut self, device: &DeviceManager) -> Result<(), Box<dyn Error>> {
        let mut encoder = device
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("create_permutation_command_encoder"),
            });
        let mut transaction = self.state.create_permutation()?;
        self.pipelines.create_permutation(&mut encoder);
        device.queue().submit(Some(encoder.finish()));
        transaction.set_commit();
        Ok(())
    }

    pub fn permute(
        &mut self,
        device: &DeviceManager,
        input: &PermuteOperationInput,
    ) -> Result<(), Box<dyn Error>> {
        let mut encoder = device
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("permute_command_encoder"),
            });
        let queue = device.queue();
        let mut transaction = self
            .state
            .permute(&self.resources, queue, &mut encoder, input)?;
        self.pipelines.permute(&mut encoder);
        queue.submit(Some(encoder.finish()));
        transaction.set_commit();
        Ok(())
    }

    pub fn swap(
        &mut self,
        device: &DeviceManager,
        input: &SwapOperationInput,
    ) -> Result<(), Box<dyn Error>> {
        let mut encoder = device
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("swap_command_encoder"),
            });
        let queue = device.queue();
        let mut transaction = self
            .state
            .swap(&self.resources, queue, &mut encoder, input)?;
        self.pipelines.swap(&mut encoder, &input.pass);
        queue.submit(Some(encoder.finish()));
        transaction.set_commit();
        Ok(())
    }

    pub fn output_count_swap(
        &mut self,
        device: &DeviceManager,
        sequence: &SwapPassSequence,
    ) -> Result<CountSwapOperationOutput, Box<dyn Error>> {
        let mut encoder = device
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("output_count_swap_command_encoder"),
            });
        let mut transaction =
            self.state
                .output_count_swap(&self.resources, &mut encoder, sequence)?;
        device.queue().submit(Some(encoder.finish()));

        let mut mapped_buffer = self.resources.count_swap_output_buffer().request_map_read();

        device.wait_for_device();

        let result = mapped_buffer.collect_mapped_buffer();
        transaction.set_commit();

        assert_eq!(result.len(), 1);
        Ok(CountSwapOperationOutput::new(
            &result[0],
            sequence,
            &self.image_dimensions,
        ))
    }

    pub fn output_permutation(
        &mut self,
        device: &DeviceManager,
    ) -> Result<ValidatedPermutation, Box<dyn Error>> {
        let mut encoder = device
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("output_permutation_command_encoder"),
            });
        let mut transaction = self
            .state
            .output_permutation(&self.resources, &mut encoder)?;
        device.queue().submit(Some(encoder.finish()));

        let mut mapped_buffer = self
            .resources
            .permutation_output_buffer()
            .request_map_read();

        device.wait_for_device();

        let result = mapped_buffer.collect_mapped_buffer();

        transaction.set_commit();
        Ok(unsafe {
            validation::vector_field_into_validated_permutation_unchecked(
                VectorFieldImageBuffer::from_vec(
                    mapped_buffer.width(),
                    mapped_buffer.height(),
                    result,
                )
                .unwrap(),
            )
        })
    }

    pub fn output_permuted_image(
        &mut self,
        device: &DeviceManager,
        format: ImageFormat,
    ) -> Result<LosslessImage, Box<dyn Error>> {
        let mut encoder = device
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("output_permuted_image_command_encoder"),
            });
        let mut transaction = self
            .state
            .output_permuted_image(&self.resources, &mut encoder)?;
        device.queue().submit(Some(encoder.finish()));

        let mut mapped_buffer = self
            .resources
            .lossless_image_output_buffer()
            .request_map_read();

        device.wait_for_device();

        let result = mapped_buffer.collect_mapped_buffer();

        transaction.set_commit();
        Ok(LosslessImage::from_texture_data(
            format,
            mapped_buffer.width(),
            mapped_buffer.height(),
            result,
        ))
    }
}
