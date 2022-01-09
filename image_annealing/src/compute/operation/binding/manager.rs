use super::super::super::resource::manager::ResourceManager;
use super::super::shader::WorkgroupGridDimensions;
use super::create_permutation::CreatePermutationBinding;
use super::permute::PermuteBinding;
use super::swap::SwapBinding;
use super::Binding;
use image_annealing_shaders::binding as binding_constants;
use std::num::NonZeroU32;

pub struct BindingManager {
    create_permutation_binding: CreatePermutationBinding,
    permute_binding: PermuteBinding,
    swap_binding: SwapBinding,
}

impl BindingManager {
    pub fn new(device: &wgpu::Device, resources: &ResourceManager) -> Self {
        Self {
            create_permutation_binding: CreatePermutationBinding::new(device, resources),
            permute_binding: PermuteBinding::new(device, resources),
            swap_binding: SwapBinding::new(device, resources),
        }
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

    pub fn swap_grid_dimensions(
        &self,
        x_stride: NonZeroU32,
        y_stride: NonZeroU32,
    ) -> WorkgroupGridDimensions {
        self.swap_binding
            .workgroup_grid_dimensions(x_stride, y_stride)
    }
}
