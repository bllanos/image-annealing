use super::super::super::resource::manager::ResourceManager;
use super::super::super::resource::texture::{PermutationOutputTexture, Texture};
use super::super::shader::WorkgroupGridDimensions;
use super::{Binding, BindingData};
use image_annealing_shader::binding::create_permutation as binding_constants;
use image_annealing_shader::WorkgroupDimensions;

pub struct CreatePermutationBinding {
    binding_data: BindingData,
    workgroup_grid_dimensions: WorkgroupGridDimensions,
}

impl CreatePermutationBinding {
    pub fn new(device: &wgpu::Device, resources: &ResourceManager) -> Self {
        let texture = resources.permutation_output_texture();

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("create_permutation_bind_group_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: binding_constants::OUTPUT_PERMUTATION_INDEX,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: PermutationOutputTexture::binding_description(),
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("create_permutation_bind_group"),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: binding_constants::OUTPUT_PERMUTATION_INDEX,
                resource: wgpu::BindingResource::TextureView(texture.view()),
            }],
        });

        Self {
            binding_data: BindingData { layout, bind_group },
            workgroup_grid_dimensions: WorkgroupGridDimensions::from_extent(
                &WorkgroupDimensions::create_permutation(),
                texture.dimensions(),
            ),
        }
    }

    pub fn workgroup_grid_dimensions(&self) -> &WorkgroupGridDimensions {
        &self.workgroup_grid_dimensions
    }
}

impl Binding for CreatePermutationBinding {
    fn layout(&self) -> &wgpu::BindGroupLayout {
        &self.binding_data.layout
    }
    fn bind<'a: 'b, 'b>(&'a self, index: u32, cpass: &mut wgpu::ComputePass<'b>) {
        self.binding_data.bind(index, cpass)
    }
}
