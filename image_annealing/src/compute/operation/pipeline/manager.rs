use super::super::super::resource::manager::ResourceManager;
use super::super::binding::manager::BindingManager;
use super::create_permutation::CreatePermutationPipeline;
use super::permute::PermutePipeline;

pub struct PipelineManager {
    bindings: BindingManager,
    create_permutation_pipeline: CreatePermutationPipeline,
    permute_pipeline: PermutePipeline,
}

impl PipelineManager {
    pub fn new(device: &wgpu::Device, resources: &ResourceManager) -> Self {
        let bindings = BindingManager::new(device, resources);
        let create_permutation_pipeline = CreatePermutationPipeline::new(device, &bindings);
        let permute_pipeline = PermutePipeline::new(device, &bindings);
        Self {
            bindings,
            create_permutation_pipeline,
            permute_pipeline,
        }
    }

    pub fn create_permutation(&self, encoder: &mut wgpu::CommandEncoder) {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("create_permutation_compute_pass"),
        });
        self.create_permutation_pipeline.set_pipeline(&mut cpass);
        self.bindings.bind_create_permutation(&mut cpass);
        self.bindings
            .create_permutation_grid_dimensions()
            .dispatch(&mut cpass);
    }

    pub fn permute(&self, encoder: &mut wgpu::CommandEncoder) {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("permute_compute_pass"),
        });
        self.permute_pipeline.set_pipeline(&mut cpass);
        self.bindings.bind_permute(&mut cpass);
        self.bindings.permute_grid_dimensions().dispatch(&mut cpass);
    }
}
