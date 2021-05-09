use super::super::device::DeviceManager;
use super::super::resource::manager::ResourceManager;
use super::pipeline::manager::PipelineManager;
use crate::image_utils::validation::ValidatedPermutation;

pub struct PermuteInput<'a> {
    pub permutation: Option<&'a ValidatedPermutation>,
    pub image: Option<&'a image::DynamicImage>,
}

struct PastOperationState {
    valid_lossless_image_input_texture: bool,
    valid_output_permutation_texture: bool,
}

impl PastOperationState {
    fn new() -> Self {
        Self {
            valid_lossless_image_input_texture: false,
            valid_output_permutation_texture: false,
        }
    }
}

pub struct OperationManager {
    pipelines: PipelineManager,
    history: PastOperationState,
}

impl OperationManager {
    pub fn new(device: &wgpu::Device, resources: &ResourceManager) -> Self {
        let pipelines = PipelineManager::new(device, resources);
        OperationManager {
            pipelines,
            history: PastOperationState::new(),
        }
    }

    pub fn create_permutation(&mut self, resources: &ResourceManager, device: &DeviceManager) {
        let mut encoder = device
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("create_permutation_command_encoder"),
            });
        self.pipelines.create_permutation(&mut encoder);
        resources
            .permutation_output_buffer()
            .load(&mut encoder, resources.permutation_output_texture());
        device.queue().submit(Some(encoder.finish()));
        self.history.valid_output_permutation_texture = true;
    }

    pub fn permute(
        &mut self,
        resources: &ResourceManager,
        device: &DeviceManager,
        input: &PermuteInput,
    ) {
        let mut encoder = device
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("permute_command_encoder"),
            });
        let queue = device.queue();
        match input.permutation {
            Some(permutation) => resources
                .permutation_input_texture()
                .load(queue, permutation),
            None => {
                assert!(self.history.valid_output_permutation_texture);
                resources
                    .permutation_input_texture()
                    .copy(&mut encoder, resources.permutation_output_texture())
            }
        }
        match input.image {
            Some(image) => {
                resources.lossless_image_input_texture().load(queue, image);
                self.history.valid_lossless_image_input_texture = true;
            }
            None => assert!(self.history.valid_lossless_image_input_texture),
        }

        self.pipelines.forward_permute(&mut encoder);
        resources
            .lossless_image_output_buffer()
            .load(&mut encoder, resources.lossless_image_output_texture());
        queue.submit(Some(encoder.finish()));
    }
}
