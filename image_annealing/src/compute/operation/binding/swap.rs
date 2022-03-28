use super::super::super::link::swap::SwapPass;
use super::super::super::resource::buffer::{BindableBuffer, InputBuffer, OutputBuffer};
use super::super::super::resource::manager::ResourceManager;
use super::super::super::resource::texture::{
    DisplacementGoalInputTexture, PermutationInputTexture, PermutationOutputTexture, Texture,
};
use super::super::shader::WorkgroupGridDimensions;
use super::{Binding, BindingData};
use crate::ImageDimensions;
use image_annealing_shaders::binding::swap as binding_constants;
use std::convert::TryFrom;

pub struct SwapBinding {
    binding_data: BindingData,
    image_dimensions: ImageDimensions,
}

impl SwapBinding {
    pub fn new(device: &wgpu::Device, resources: &ResourceManager) -> Self {
        let swap_parameters_buffer = resources.swap_parameters_buffer();
        let displacement_goal_input_texture = resources.displacement_goal_input_texture();
        let permutation_input_texture = resources.permutation_input_texture();
        let permutation_output_texture = resources.permutation_output_texture();
        let count_swap_input_buffer = resources.count_swap_input_buffer();

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("swap_bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: binding_constants::PARAMETERS_INDEX,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: swap_parameters_buffer.input_binding_description(),
                    count: None,
                },
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
                wgpu::BindGroupLayoutEntry {
                    binding: binding_constants::OUTPUT_COUNT_BUFFER_INDEX,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: count_swap_input_buffer.output_binding_description(),
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("swap_bind_group"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: binding_constants::PARAMETERS_INDEX,
                    resource: swap_parameters_buffer.binding_resource(),
                },
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
                wgpu::BindGroupEntry {
                    binding: binding_constants::OUTPUT_COUNT_BUFFER_INDEX,
                    resource: count_swap_input_buffer.binding_resource(),
                },
            ],
        });

        Self {
            binding_data: BindingData { layout, bind_group },
            image_dimensions: ImageDimensions::try_from(permutation_input_texture.dimensions())
                .unwrap(),
        }
    }

    pub fn workgroup_grid_dimensions(&self, pass: &SwapPass) -> WorkgroupGridDimensions {
        pass.swap_workgroup_grid_dimensions(&self.image_dimensions)
    }
}

impl Binding for SwapBinding {
    fn layout(&self) -> &wgpu::BindGroupLayout {
        &self.binding_data.layout
    }
    fn bind<'a: 'b, 'b>(&'a self, index: u32, cpass: &mut wgpu::ComputePass<'b>) {
        self.binding_data.bind(index, cpass)
    }
}
