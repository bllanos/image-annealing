use super::super::device::DeviceManager;
use super::super::resource::manager::ResourceManager;
use super::pipeline::manager::PipelineManager;

pub struct OperationManager {
    pipelines: PipelineManager,
}

impl OperationManager {
    pub fn new(device: &wgpu::Device, resources: &ResourceManager) -> Self {
        let pipelines = PipelineManager::new(device, resources);
        OperationManager { pipelines }
    }

    pub fn create_permutation(&self, resources: &ResourceManager, device: &DeviceManager) {
        let mut encoder = device
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("create_permutation_command_encoder"),
            });
        self.pipelines.create_permutation(&mut encoder);
        encoder.copy_texture_to_buffer(
            resources.permutation_texture().copy_view(),
            resources.permutation_output_buffer().copy_view(),
            resources.permutation_texture().dimensions(),
        );
        device.queue().submit(Some(encoder.finish()));
    }
}
