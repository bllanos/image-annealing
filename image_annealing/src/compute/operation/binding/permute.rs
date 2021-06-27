use super::super::super::resource::manager::ResourceManager;
use super::super::super::resource::texture::{
    LosslessImageTexture, PermutationTexture, Texture, TextureDatatype,
};
use super::super::shader::WorkgroupGridDimensions;
use super::{Binding, BindingData};

pub struct PermuteBinding(BindingData);

impl PermuteBinding {
    pub fn new(device: &wgpu::Device, resources: &ResourceManager) -> Self {
        let permutation_texture = resources.permutation_input_texture();
        let image_input_texture = resources.lossless_image_input_texture();
        let image_output_texture = resources.lossless_image_output_texture();

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("permute_bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::ReadOnly,
                        format: PermutationTexture::format(),
                        view_dimension: PermutationTexture::view_dimension(),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::ReadOnly,
                        format: LosslessImageTexture::format(),
                        view_dimension: LosslessImageTexture::view_dimension(),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStage::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: LosslessImageTexture::format(),
                        view_dimension: LosslessImageTexture::view_dimension(),
                    },
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("permute_bind_group"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(permutation_texture.view()),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(image_input_texture.view()),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(image_output_texture.view()),
                },
            ],
        });

        Self(BindingData {
            layout,
            bind_group,
            workgroup_grid_dimensions: super::get_workgroup_grid_dimensions(permutation_texture),
        })
    }
}

impl Binding for PermuteBinding {
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
