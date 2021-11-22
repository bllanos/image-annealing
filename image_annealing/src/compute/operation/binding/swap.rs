use super::super::super::resource::manager::ResourceManager;
use super::super::super::resource::texture::{
    DisplacementGoalInputTexture, PermutationInputTexture, PermutationOutputTexture, Texture,
};
use super::super::shader::WorkgroupGridDimensions;
use super::{Binding, BindingData};
use image_annealing_shaders::binding::swap as binding_constants;

pub struct SwapBinding(BindingData);

impl SwapBinding {
    pub fn new(device: &wgpu::Device, resources: &ResourceManager) -> Self {
        let displacement_goal_input_texture = resources.displacement_goal_input_texture();
        let permutation_input_texture = resources.permutation_input_texture();
        let permutation_output_texture = resources.permutation_output_texture();

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("swap_bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: binding_constants::DISPLACEMENT_GOAL_INDEX,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: DisplacementGoalInputTexture::binding_description(),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: binding_constants::INPUT_PERMUTATION_INDEX,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: PermutationInputTexture::binding_description(),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: binding_constants::OUTPUT_PERMUTATION_INDEX,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: PermutationOutputTexture::binding_description(),
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("swap_bind_group"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: binding_constants::DISPLACEMENT_GOAL_INDEX,
                    resource: wgpu::BindingResource::TextureView(
                        displacement_goal_input_texture.view(),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: binding_constants::INPUT_PERMUTATION_INDEX,
                    resource: wgpu::BindingResource::TextureView(permutation_input_texture.view()),
                },
                wgpu::BindGroupEntry {
                    binding: binding_constants::OUTPUT_PERMUTATION_INDEX,
                    resource: wgpu::BindingResource::TextureView(permutation_output_texture.view()),
                },
            ],
        });

        Self(BindingData {
            layout,
            bind_group,
            workgroup_grid_dimensions: super::get_workgroup_grid_dimensions(
                permutation_input_texture,
            ),
        })
    }
}

impl Binding for SwapBinding {
    fn layout(&self) -> &wgpu::BindGroupLayout {
        &self.0.layout
    }
    fn workgroup_grid_dimensions(&self) -> &WorkgroupGridDimensions {
        &self.0.workgroup_grid_dimensions
    }
    fn bind<'a: 'b, 'b>(&'a self, index: u32, cpass: &mut wgpu::ComputePass<'b>) {
        self.0.bind(index, cpass)
    }
}
