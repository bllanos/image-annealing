use super::super::super::link::swap::SwapPass;
use super::super::super::resource::manager::ResourceManager;
use super::super::binding::manager::BindingManager;
use super::count_swap::CountSwapPipeline;
use super::create_displacement_goal::CreateDisplacementGoalPipeline;
use super::create_permutation::CreatePermutationPipeline;
use super::permute::PermutePipeline;
use super::swap::SwapPipeline;

pub struct PipelineManager {
    bindings: BindingManager,
    count_swap_pipeline: CountSwapPipeline,
    create_displacement_goal_pipeline: CreateDisplacementGoalPipeline,
    create_permutation_pipeline: CreatePermutationPipeline,
    permute_pipeline: PermutePipeline,
    swap_pipeline: SwapPipeline,
}

impl PipelineManager {
    pub fn new(device: &wgpu::Device, resources: &ResourceManager) -> Self {
        let bindings = BindingManager::new(device, resources);
        let count_swap_pipeline = CountSwapPipeline::new(device, &bindings);
        let create_displacement_goal_pipeline =
            CreateDisplacementGoalPipeline::new(device, &bindings);
        let create_permutation_pipeline = CreatePermutationPipeline::new(device, &bindings);
        let permute_pipeline = PermutePipeline::new(device, &bindings);
        let swap_pipeline = SwapPipeline::new(device, &bindings);
        Self {
            bindings,
            count_swap_pipeline,
            create_displacement_goal_pipeline,
            create_permutation_pipeline,
            permute_pipeline,
            swap_pipeline,
        }
    }

    pub fn count_swap(&self, encoder: &mut wgpu::CommandEncoder) {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("count_swap_compute_pass"),
        });
        self.count_swap_pipeline.set_pipeline(&mut cpass);
        self.bindings.bind_count_swap(&mut cpass);
        self.bindings
            .count_swap_grid_dimensions()
            .dispatch(&mut cpass);
    }

    pub fn create_displacement_goal(&self, encoder: &mut wgpu::CommandEncoder) {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("create_displacement_goal_compute_pass"),
        });
        self.create_displacement_goal_pipeline
            .set_pipeline(&mut cpass);
        self.bindings.bind_create_displacement_goal(&mut cpass);
        self.bindings
            .create_displacement_goal_grid_dimensions()
            .dispatch(&mut cpass);
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

    pub fn swap(&self, encoder: &mut wgpu::CommandEncoder, pass: &SwapPass) {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("swap_compute_pass"),
        });
        self.swap_pipeline.set_pipeline(&mut cpass);
        self.bindings.bind_swap(&mut cpass);
        self.bindings
            .swap_grid_dimensions(pass)
            .dispatch(&mut cpass);
    }
}
