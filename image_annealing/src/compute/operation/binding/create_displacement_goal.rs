use super::super::super::resource::manager::ResourceManager;
use super::super::super::resource::texture::{
    DisplacementGoalInputTexture, DisplacementGoalOutputTexture, LosslessImageInputTexture,
    PermutationInputTexture, Texture,
};
use super::super::shader::WorkgroupGridDimensions;
use super::{Binding, BindingData};
use image_annealing_shader::binding::create_displacement_goal as binding_constants;
use image_annealing_shader::WorkgroupDimensions;

pub struct CreateDisplacementGoalBinding {
    binding_data: BindingData,
    workgroup_grid_dimensions: WorkgroupGridDimensions,
}

impl CreateDisplacementGoalBinding {
    pub fn new(device: &wgpu::Device, resources: &ResourceManager) -> Self {
        let displacement_goal_input_texture = resources.displacement_goal_input_texture();
        let permutation_texture = resources.permutation_input_texture();
        let image_texture = resources.lossless_image_input_texture();
        let displacement_goal_output_texture = resources.displacement_goal_output_texture();

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("create_displacement_goal_bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: binding_constants::INPUT_DISPLACEMENT_GOAL_INDEX,
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
                    binding: binding_constants::INPUT_IMAGE_INDEX,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: LosslessImageInputTexture::binding_description(),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: binding_constants::OUTPUT_DISPLACEMENT_GOAL_INDEX,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: DisplacementGoalOutputTexture::binding_description(),
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("create_displacement_goal_bind_group"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: binding_constants::INPUT_DISPLACEMENT_GOAL_INDEX,
                    resource: wgpu::BindingResource::TextureView(
                        displacement_goal_input_texture.view(),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: binding_constants::INPUT_PERMUTATION_INDEX,
                    resource: wgpu::BindingResource::TextureView(permutation_texture.view()),
                },
                wgpu::BindGroupEntry {
                    binding: binding_constants::INPUT_IMAGE_INDEX,
                    resource: wgpu::BindingResource::TextureView(image_texture.view()),
                },
                wgpu::BindGroupEntry {
                    binding: binding_constants::OUTPUT_DISPLACEMENT_GOAL_INDEX,
                    resource: wgpu::BindingResource::TextureView(
                        displacement_goal_output_texture.view(),
                    ),
                },
            ],
        });

        Self {
            binding_data: BindingData { layout, bind_group },
            workgroup_grid_dimensions: WorkgroupGridDimensions::from_extent(
                &WorkgroupDimensions::create_displacement_goal_default(),
                displacement_goal_input_texture.dimensions(),
            ),
        }
    }

    pub fn default_workgroup_grid_dimensions(&self) -> &WorkgroupGridDimensions {
        &self.workgroup_grid_dimensions
    }
}

impl Binding for CreateDisplacementGoalBinding {
    fn layout(&self) -> &wgpu::BindGroupLayout {
        &self.binding_data.layout
    }
    fn bind<'a: 'b, 'b>(&'a self, index: u32, cpass: &mut wgpu::ComputePass<'b>) {
        self.binding_data.bind(index, cpass)
    }
}
