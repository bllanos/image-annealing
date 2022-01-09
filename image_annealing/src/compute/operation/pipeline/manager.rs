use super::super::super::resource::manager::ResourceManager;
use super::super::binding::manager::BindingManager;
use super::create_permutation::CreatePermutationPipeline;
use super::permute::PermutePipeline;
use super::swap::SwapPipeline;
use std::num::NonZeroU32;

pub struct PipelineManager {
    bindings: BindingManager,
    create_permutation_pipeline: CreatePermutationPipeline,
    permute_pipeline: PermutePipeline,
    swap_pipeline: SwapPipeline,
}

impl PipelineManager {
    pub fn new(device: &wgpu::Device, resources: &ResourceManager) -> Self {
        let bindings = BindingManager::new(device, resources);
        let create_permutation_pipeline = CreatePermutationPipeline::new(device, &bindings);
        let permute_pipeline = PermutePipeline::new(device, &bindings);
        let swap_pipeline = SwapPipeline::new(device, &bindings);
        Self {
            bindings,
            create_permutation_pipeline,
            permute_pipeline,
            swap_pipeline,
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

    pub fn swap(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        x_stride: NonZeroU32,
        y_stride: NonZeroU32,
    ) {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("swap_compute_pass"),
        });
        self.swap_pipeline.set_pipeline(&mut cpass);
        self.bindings.bind_swap(&mut cpass);
        self.bindings
            .swap_grid_dimensions(x_stride, y_stride)
            .dispatch(&mut cpass);
    }
}
