use super::super::super::resource::manager::ResourceManager;
use super::super::shader::WorkgroupDimensions;
use super::create_permutation::CreatePermutation;
use super::permute::Permute;
use super::Binding;

type CreatePermutationBinding = Binding<CreatePermutation>;
type PermuteBinding = Binding<Permute>;

pub struct BindingManager {
    create_permutation_binding: CreatePermutationBinding,
    permute_binding: PermuteBinding,
}

impl BindingManager {
    pub fn new(device: &wgpu::Device, resources: &ResourceManager) -> Self {
        Self {
            create_permutation_binding: CreatePermutationBinding::new(device, resources),
            permute_binding: PermuteBinding::new(device, resources),
        }
    }

    pub fn bind_create_permutation<'a: 'b, 'b>(&'a self, cpass: &mut wgpu::ComputePass<'b>) {
        self.create_permutation_binding.bind(0, cpass);
    }

    pub fn create_permutation_layout(&self) -> &wgpu::BindGroupLayout {
        self.create_permutation_binding.layout()
    }

    pub fn create_permutation_dimensions(&self) -> &WorkgroupDimensions {
        self.create_permutation_binding.workgroup_dimensions()
    }

    pub fn bind_permute<'a: 'b, 'b>(&'a self, cpass: &mut wgpu::ComputePass<'b>) {
        self.permute_binding.bind(0, cpass);
    }

    pub fn permute_layout(&self) -> &wgpu::BindGroupLayout {
        self.permute_binding.layout()
    }

    pub fn permute_dimensions(&self) -> &WorkgroupDimensions {
        self.permute_binding.workgroup_dimensions()
    }
}
