use super::super::device::DeviceManager;
use super::super::format::{LosslessImageBuffer, VectorFieldImageBuffer};
use super::super::link::swap::SwapPass;
use super::super::resource::buffer::ReadMappableBuffer;
use super::super::resource::manager::ResourceManager;
use super::pipeline::manager::PipelineManager;
use crate::image_utils::validation::{self};
use crate::{DisplacementGoal, ImageDimensions, ValidatedPermutation};
use std::default::Default;
use std::error::Error;

mod state;
use state::ResourceStateManager;

#[derive(Default)]
pub struct PermuteOperationInput<'a> {
    pub permutation: Option<&'a ValidatedPermutation>,
    pub image: Option<&'a image::DynamicImage>,
}

#[derive(Default)]
pub struct SwapOperationInput<'a> {
    pub permutation: Option<&'a ValidatedPermutation>,
    pub displacement_goal: Option<&'a DisplacementGoal>,
}

pub struct OperationManager {
    resources: ResourceManager,
    state: ResourceStateManager,
    pipelines: PipelineManager,
}

impl OperationManager {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        let resources = ResourceManager::new(device, image_dimensions);
        let pipelines = PipelineManager::new(device, &resources);
        OperationManager {
            resources,
            state: ResourceStateManager::new(),
            pipelines,
        }
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
        self.pipelines.swap(&mut encoder, &SwapPass::Horizontal);
        queue.submit(Some(encoder.finish()));
        transaction.set_commit();
        Ok(())
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
        Ok(validation::vector_field_into_validated_permutation(
            VectorFieldImageBuffer::from_vec(mapped_buffer.width(), mapped_buffer.height(), result)
                .unwrap(),
        ))
    }

    pub fn output_permuted_image(
        &mut self,
        device: &DeviceManager,
    ) -> Result<LosslessImageBuffer, Box<dyn Error>> {
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
        Ok(
            LosslessImageBuffer::from_vec(mapped_buffer.width(), mapped_buffer.height(), result)
                .unwrap(),
        )
    }
}
