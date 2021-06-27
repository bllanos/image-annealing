use super::super::super::resource::manager::ResourceManager;
use super::super::binding::manager::BindingManager;
use super::super::shader::WorkgroupGridDimensions;
use super::create_permutation::CreatePermutationPipeline;
use super::forward_permute::ForwardPermutePipeline;

pub struct PipelineManager {
    bindings: BindingManager,
    create_permutation_pipeline: CreatePermutationPipeline,
    forward_permute_pipeline: ForwardPermutePipeline,
}

impl PipelineManager {
    pub fn new(device: &wgpu::Device, resources: &ResourceManager) -> Self {
        let bindings = BindingManager::new(device, resources);
        let create_permutation_pipeline = CreatePermutationPipeline::new(device, &bindings);
        let forward_permute_pipeline = ForwardPermutePipeline::new(device, &bindings);
        Self {
            bindings,
            create_permutation_pipeline,
            forward_permute_pipeline,
        }
    }

    pub fn create_permutation(&self, encoder: &mut wgpu::CommandEncoder) {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("create_permutation_compute_pass"),
        });
        self.create_permutation_pipeline.set_pipeline(&mut cpass);
        self.bindings.bind_create_permutation(&mut cpass);
        let &WorkgroupGridDimensions(x, y, z) = self.bindings.create_permutation_grid_dimensions();
        cpass.dispatch(x, y, z);
    }

    pub fn forward_permute(&self, encoder: &mut wgpu::CommandEncoder) {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("forward_permute_compute_pass"),
        });
        self.forward_permute_pipeline.set_pipeline(&mut cpass);
        self.bindings.bind_permute(&mut cpass);
        let &WorkgroupGridDimensions(x, y, z) = self.bindings.permute_grid_dimensions();
        cpass.dispatch(x, y, z);
    }
}
