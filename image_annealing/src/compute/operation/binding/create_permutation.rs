use super::super::super::resource::manager::ResourceManager;
use super::super::super::resource::texture::{PermutationTexture, Texture, TextureDatatype};
use super::super::shader::WorkgroupGridDimensions;
use super::{Binding, BindingData};
use image_annealing_shaders::binding::create_permutation as binding_constants;

pub struct CreatePermutationBinding(BindingData);

impl CreatePermutationBinding {
    pub fn new(device: &wgpu::Device, resources: &ResourceManager) -> Self {
        let texture = resources.permutation_output_texture();

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("create_permutation_bind_group_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: binding_constants::OUTPUT_PERMUTATION_INDEX,
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
                binding: binding_constants::OUTPUT_PERMUTATION_INDEX,
                resource: wgpu::BindingResource::TextureView(texture.view()),
            }],
        });

        Self(BindingData {
            layout,
            bind_group,
            workgroup_grid_dimensions: super::get_workgroup_grid_dimensions(texture),
        })
    }
}

impl Binding for CreatePermutationBinding {
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
