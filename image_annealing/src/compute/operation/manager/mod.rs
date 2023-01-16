use super::super::device::{DeviceManager, DevicePollType};
use super::super::format::{ImageFormat, LosslessImage, VectorFieldImageBuffer};
use super::super::link::swap::SwapPassSequence;
use super::super::resource::manager::ResourceManager;
use super::pipeline::manager::PipelineManager;
use crate::image_utils::validation::{self};
use crate::{DisplacementGoal, ImageDimensions, ValidatedPermutation};
use std::error::Error;
use std::fmt;

mod input;
mod output;
mod state;

pub use super::pipeline::manager::{
    CreateDisplacementGoalPipelineConfig, CreateDisplacementGoalShaderConfig,
};
pub use input::{CreateDisplacementGoalOperationInput, PermuteOperationInput, SwapOperationInput};
pub use output::CountSwapOperationOutput;
use state::ResourceStateManager;

#[derive(Debug, Clone)]
pub struct NoCreateDisplacementGoalPipelineError;

impl fmt::Display for NoCreateDisplacementGoalPipelineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "no displacement goal generation compute shader pipeline has been set"
        )
    }
}

impl Error for NoCreateDisplacementGoalPipelineError {}

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

    pub fn configure_create_displacement_goal_pipeline(
        &mut self,
        device: &DeviceManager,
        config: Option<CreateDisplacementGoalPipelineConfig<'static>>,
    ) {
        self.pipelines
            .configure_create_displacement_goal_pipeline(device.device(), config);
    }

    pub fn count_swap(
        &mut self,
        device: &DeviceManager,
        sequence: SwapPassSequence,
    ) -> Result<(), Box<dyn Error>> {
        let queue = device.queue();
        self.state.count_swap(&self.resources, queue, sequence)?;
        let mut encoder = device
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("count_swap_command_encoder"),
            });
        self.pipelines.count_swap(&mut encoder);
        queue.submit(Some(encoder.finish()));
        Ok(())
    }

    pub fn create_displacement_goal(
        &mut self,
        device: &DeviceManager,
        input: &CreateDisplacementGoalOperationInput,
    ) -> Result<(), Box<dyn Error>> {
        if self.pipelines.has_create_displacement_goal_pipeline() {
            let mut encoder =
                device
                    .device()
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("create_displacement_goal_command_encoder"),
                    });
            let queue = device.queue();
            self.state
                .create_displacement_goal(&self.resources, queue, &mut encoder, input)?;
            self.pipelines.create_displacement_goal(&mut encoder);
            queue.submit(Some(encoder.finish()));
            Ok(())
        } else {
            Err(Box::new(NoCreateDisplacementGoalPipelineError))
        }
    }

    pub fn create_permutation(&mut self, device: &DeviceManager) -> Result<(), Box<dyn Error>> {
        if !self.state.can_skip_create_permutation() {
            self.state.create_permutation()?;
            let mut encoder =
                device
                    .device()
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("create_permutation_command_encoder"),
                    });
            self.pipelines.create_permutation(&mut encoder);
            device.queue().submit(Some(encoder.finish()));
        }
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
        self.state
            .permute(&self.resources, queue, &mut encoder, input)?;
        self.pipelines.permute(&mut encoder);
        queue.submit(Some(encoder.finish()));
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
        self.state
            .swap(&self.resources, queue, &mut encoder, input)?;
        self.pipelines.swap(&mut encoder, &input.pass);
        queue.submit(Some(encoder.finish()));
        Ok(())
    }

    pub async fn output_count_swap(
        &mut self,
        device: &DeviceManager,
        poll_type: DevicePollType,
        sequence: &SwapPassSequence,
    ) -> Result<CountSwapOperationOutput, Box<dyn Error>> {
        let mut encoder = device
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("output_count_swap_command_encoder"),
            });
        self.state
            .output_count_swap(&self.resources, &mut encoder, sequence)?;
        device.queue().submit(Some(encoder.finish()));

        let result = self
            .resources
            .count_swap_output_buffer()
            .collect(device, poll_type)
            .await;

        assert_eq!(result.len(), 1);
        Ok(CountSwapOperationOutput::new(
            &result[0],
            sequence,
            &self.image_dimensions,
        ))
    }

    pub async fn output_displacement_goal(
        &mut self,
        device: &DeviceManager,
        poll_type: DevicePollType,
    ) -> Result<DisplacementGoal, Box<dyn Error>> {
        let mut encoder = device
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("output_displacement_goal_command_encoder"),
            });
        self.state
            .output_displacement_goal(&self.resources, &mut encoder)?;
        device.queue().submit(Some(encoder.finish()));

        let buffer = self.resources.displacement_goal_output_buffer();
        let result = buffer.collect(device, poll_type).await;

        Ok(DisplacementGoal::from_vector_field(
            VectorFieldImageBuffer::from_vec(
                buffer.width().try_into().unwrap(),
                buffer.height().try_into().unwrap(),
                result,
            )
            .unwrap(),
        )
        .unwrap())
    }

    pub async fn output_permutation(
        &mut self,
        device: &DeviceManager,
        poll_type: DevicePollType,
    ) -> Result<ValidatedPermutation, Box<dyn Error>> {
        let mut encoder = device
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("output_permutation_command_encoder"),
            });
        self.state
            .output_permutation(&self.resources, &mut encoder)?;
        device.queue().submit(Some(encoder.finish()));

        let buffer = self.resources.permutation_output_buffer();
        let result = buffer.collect(device, poll_type).await;

        Ok(unsafe {
            validation::vector_field_into_validated_permutation_unchecked(
                VectorFieldImageBuffer::from_vec(
                    buffer.width().try_into().unwrap(),
                    buffer.height().try_into().unwrap(),
                    result,
                )
                .unwrap(),
            )
        })
    }

    pub async fn output_permuted_image(
        &mut self,
        device: &DeviceManager,
        poll_type: DevicePollType,
        format: ImageFormat,
    ) -> Result<LosslessImage, Box<dyn Error>> {
        let mut encoder = device
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("output_permuted_image_command_encoder"),
            });
        self.state
            .output_permuted_image(&self.resources, &mut encoder)?;
        device.queue().submit(Some(encoder.finish()));

        let buffer = self.resources.lossless_image_output_buffer();
        let result = buffer.collect(device, poll_type).await;

        Ok(LosslessImage::from_texture_data(
            format,
            buffer.width().try_into().unwrap(),
            buffer.height().try_into().unwrap(),
            result,
        ))
    }
}
