use super::super::super::resource::manager::ResourceManager;
use super::super::shader::WorkgroupDimensions;
use super::create_permutation::CreatePermutation;
use super::Binding;

type CreatePermutationBinding = Binding<CreatePermutation>;

pub struct BindingManager {
    create_permutation_binding: CreatePermutationBinding,
}

impl BindingManager {
    pub fn new(device: &wgpu::Device, resources: &ResourceManager) -> Self {
        Self {
            create_permutation_binding: CreatePermutationBinding::new(device, resources),
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
}
