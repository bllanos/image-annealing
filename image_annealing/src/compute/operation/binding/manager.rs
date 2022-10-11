use super::super::super::link::swap::SwapPass;
use super::super::super::resource::manager::ResourceManager;
use super::super::shader::WorkgroupGridDimensions;
use super::count_swap::CountSwapBinding;
use super::create_displacement_goal::CreateDisplacementGoalBinding;
use super::create_permutation::CreatePermutationBinding;
use super::permute::PermuteBinding;
use super::swap::SwapBinding;
use super::Binding;
use image_annealing_shaders::binding as binding_constants;

pub struct BindingManager {
    count_swap_binding: CountSwapBinding,
    create_displacement_goal_binding: CreateDisplacementGoalBinding,
    create_permutation_binding: CreatePermutationBinding,
    permute_binding: PermuteBinding,
    swap_binding: SwapBinding,
}

impl BindingManager {
    pub fn new(device: &wgpu::Device, resources: &ResourceManager) -> Self {
        Self {
            count_swap_binding: CountSwapBinding::new(device, resources),
            create_displacement_goal_binding: CreateDisplacementGoalBinding::new(device, resources),
            create_permutation_binding: CreatePermutationBinding::new(device, resources),
            permute_binding: PermuteBinding::new(device, resources),
            swap_binding: SwapBinding::new(device, resources),
        }
    }

    pub fn bind_count_swap<'a: 'b, 'b>(&'a self, cpass: &mut wgpu::ComputePass<'b>) {
        self.count_swap_binding
            .bind(binding_constants::count_swap::GROUP_INDEX, cpass);
    }

    pub fn count_swap_layout(&self) -> &wgpu::BindGroupLayout {
        self.count_swap_binding.layout()
    }

    pub fn count_swap_grid_dimensions(&self) -> &WorkgroupGridDimensions {
        self.count_swap_binding.workgroup_grid_dimensions()
    }

    pub fn bind_create_displacement_goal<'a: 'b, 'b>(&'a self, cpass: &mut wgpu::ComputePass<'b>) {
        self.create_displacement_goal_binding.bind(
            binding_constants::create_displacement_goal::GROUP_INDEX,
            cpass,
        );
    }

    pub fn create_displacement_goal_layout(&self) -> &wgpu::BindGroupLayout {
        self.create_displacement_goal_binding.layout()
    }

    pub fn create_displacement_goal_grid_dimensions(&self) -> &WorkgroupGridDimensions {
        self.create_displacement_goal_binding
            .workgroup_grid_dimensions()
    }

    pub fn bind_create_permutation<'a: 'b, 'b>(&'a self, cpass: &mut wgpu::ComputePass<'b>) {
        self.create_permutation_binding
            .bind(binding_constants::create_permutation::GROUP_INDEX, cpass);
    }

    pub fn create_permutation_layout(&self) -> &wgpu::BindGroupLayout {
        self.create_permutation_binding.layout()
    }

    pub fn create_permutation_grid_dimensions(&self) -> &WorkgroupGridDimensions {
        self.create_permutation_binding.workgroup_grid_dimensions()
    }

    pub fn bind_permute<'a: 'b, 'b>(&'a self, cpass: &mut wgpu::ComputePass<'b>) {
        self.permute_binding
            .bind(binding_constants::permute::GROUP_INDEX, cpass);
    }

    pub fn permute_layout(&self) -> &wgpu::BindGroupLayout {
        self.permute_binding.layout()
    }

    pub fn permute_grid_dimensions(&self) -> &WorkgroupGridDimensions {
        self.permute_binding.workgroup_grid_dimensions()
    }

    pub fn bind_swap<'a: 'b, 'b>(&'a self, cpass: &mut wgpu::ComputePass<'b>) {
        self.swap_binding
            .bind(binding_constants::swap::GROUP_INDEX, cpass);
    }

    pub fn swap_layout(&self) -> &wgpu::BindGroupLayout {
        self.swap_binding.layout()
    }

    pub fn swap_grid_dimensions(&self, pass: &SwapPass) -> WorkgroupGridDimensions {
        self.swap_binding.workgroup_grid_dimensions(pass)
    }
}
