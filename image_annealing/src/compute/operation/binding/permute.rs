use super::super::super::resource::manager::ResourceManager;
use super::super::super::resource::texture::{
    LosslessImageInputTexture, LosslessImageOutputTexture, PermutationInputTexture, Texture,
};
use super::super::shader::WorkgroupGridDimensions;
use super::{Binding, BindingData};
use image_annealing_shader::binding::permute as binding_constants;
use image_annealing_shader::WorkgroupDimensions;

pub struct PermuteBinding {
    binding_data: BindingData,
    workgroup_grid_dimensions: WorkgroupGridDimensions,
}

impl PermuteBinding {
    pub fn new(device: &wgpu::Device, resources: &ResourceManager) -> Self {
        let permutation_texture = resources.permutation_input_texture();
        let image_input_texture = resources.lossless_image_input_texture();
        let image_output_texture = resources.lossless_image_output_texture();

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("permute_bind_group_layout"),
            entries: &[
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
                    binding: binding_constants::OUTPUT_IMAGE_INDEX,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: LosslessImageOutputTexture::binding_description(),
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("permute_bind_group"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: binding_constants::INPUT_PERMUTATION_INDEX,
                    resource: wgpu::BindingResource::TextureView(permutation_texture.view()),
                },
                wgpu::BindGroupEntry {
                    binding: binding_constants::INPUT_IMAGE_INDEX,
                    resource: wgpu::BindingResource::TextureView(image_input_texture.view()),
                },
                wgpu::BindGroupEntry {
                    binding: binding_constants::OUTPUT_IMAGE_INDEX,
                    resource: wgpu::BindingResource::TextureView(image_output_texture.view()),
                },
            ],
        });

        Self {
            binding_data: BindingData { layout, bind_group },
            workgroup_grid_dimensions: WorkgroupGridDimensions::from_extent(
                &WorkgroupDimensions::permute(),
                permutation_texture.dimensions(),
            ),
        }
    }

    pub fn workgroup_grid_dimensions(&self) -> &WorkgroupGridDimensions {
        &self.workgroup_grid_dimensions
    }
}

impl Binding for PermuteBinding {
    fn layout(&self) -> &wgpu::BindGroupLayout {
        &self.binding_data.layout
    }
    fn bind<'a: 'b, 'b>(&'a self, index: u32, cpass: &mut wgpu::ComputePass<'b>) {
        self.binding_data.bind(index, cpass)
    }
}
