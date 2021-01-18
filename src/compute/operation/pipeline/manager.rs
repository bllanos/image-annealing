use super::super::super::resource::manager::ResourceManager;
use super::super::binding::manager::BindingManager;
use super::super::shader::WorkgroupDimensions;
use super::create_permutation::CreatePermutationPipeline;

pub struct PipelineManager {
    bindings: BindingManager,
    create_permutation_pipeline: CreatePermutationPipeline,
}

impl PipelineManager {
    pub fn new(device: &wgpu::Device, resources: &ResourceManager) -> Self {
        let bindings = BindingManager::new(device, resources);
        let create_permutation_pipeline = CreatePermutationPipeline::new(device, &bindings);
        Self {
            bindings,
            create_permutation_pipeline,
        }
    }

    pub fn create_permutation(&self, encoder: &mut wgpu::CommandEncoder) {
        let mut cpass = encoder.begin_compute_pass();
        self.create_permutation_pipeline.set_pipeline(&mut cpass);
        self.bindings.bind_create_permutation(&mut cpass);
        let &WorkgroupDimensions(x, y, z) = self.bindings.create_permutation_dimensions();
        cpass.dispatch(x, y, z);
    }
}
