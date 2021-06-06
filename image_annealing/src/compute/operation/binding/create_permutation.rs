use super::super::super::resource::manager::ResourceManager;
use super::super::super::resource::texture::{PermutationTexture, Texture, TextureDatatype};
use super::Binding;
use std::marker::PhantomData;

pub struct CreatePermutation {}

impl Binding<CreatePermutation> {
    pub fn new(device: &wgpu::Device, resources: &ResourceManager) -> Self {
        let texture = resources.permutation_output_texture();

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("create_permutation_bind_group_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::COMPUTE,
                ty: wgpu::BindingType::StorageTexture {
                    access: wgpu::StorageTextureAccess::WriteOnly,
                    format: PermutationTexture::format(),
                    view_dimension: PermutationTexture::view_dimension(),
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("create_permutation_bind_group"),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(texture.view()),
            }],
        });

        Self {
            layout,
            bind_group,
            workgroup_dimensions: super::get_workgroup_dimensions(texture),
            phantom: PhantomData,
        }
    }
}
