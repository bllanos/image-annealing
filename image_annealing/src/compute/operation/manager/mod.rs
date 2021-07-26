use super::super::device::DeviceManager;
use super::super::resource::manager::ResourceManager;
use super::pipeline::manager::PipelineManager;
use crate::image_utils::validation::ValidatedPermutation;
use std::default::Default;
use std::error::Error;

mod state;
use state::ResourceStateManager;

#[derive(Default)]
pub struct PermuteOperationInput<'a> {
    pub permutation: Option<&'a ValidatedPermutation>,
    pub image: Option<&'a image::DynamicImage>,
}

pub struct OperationManager {
    pipelines: PipelineManager,
    state: ResourceStateManager,
}

impl OperationManager {
    pub fn new(device: &wgpu::Device, resources: &ResourceManager) -> Self {
        let pipelines = PipelineManager::new(device, resources);
        OperationManager {
            pipelines,
            state: ResourceStateManager::new(),
        }
    }

    pub fn create_permutation(
        &mut self,
        resources: &ResourceManager,
        device: &DeviceManager,
    ) -> Result<(), Box<dyn Error>> {
        let mut encoder = device
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("create_permutation_command_encoder"),
            });
        self.state.prepare_create_permutation()?;
        self.pipelines.create_permutation(&mut encoder);
        self.state
            .finish_create_permutation(resources, &mut encoder)?;
        device.queue().submit(Some(encoder.finish()));
        Ok(())
    }

    pub fn permute(
        &mut self,
        resources: &ResourceManager,
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
            .prepare_permute(resources, queue, &mut encoder, input)?;
        self.pipelines.permute(&mut encoder);
        self.state.finish_permute(resources, &mut encoder)?;
        queue.submit(Some(encoder.finish()));
        Ok(())
    }
}
