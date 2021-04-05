use super::super::resource::manager::ResourceManager;
use super::super::resource::texture::{PermutationTexture, Texture, TextureDatatype};
use super::shader::WorkgroupDimensions;
use std::marker::PhantomData;

pub mod manager;

struct CreatePermutation {}

struct Binding<T> {
    layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    workgroup_dimensions: WorkgroupDimensions,
    phantom: PhantomData<T>,
}
type CreatePermutationBinding = Binding<CreatePermutation>;

impl<T> Binding<T> {
    pub fn layout(&self) -> &wgpu::BindGroupLayout {
        &self.layout
    }

    pub fn workgroup_dimensions(&self) -> &WorkgroupDimensions {
        &self.workgroup_dimensions
    }

    pub fn bind<'a: 'b, 'b>(&'a self, index: u32, cpass: &mut wgpu::ComputePass<'b>) {
        cpass.set_bind_group(index, &self.bind_group, &[]);
    }
}

impl CreatePermutationBinding {
    pub fn new(device: &wgpu::Device, resources: &ResourceManager) -> Self {
        let texture = resources.permutation_texture();

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
            workgroup_dimensions: get_workgroup_dimensions(texture),
            phantom: PhantomData,
        }
    }
}

fn get_workgroup_dimensions<T>(texture: &Texture<T>) -> WorkgroupDimensions {
    let dimensions = texture.dimensions();
    WorkgroupDimensions::new(dimensions.width, dimensions.height)
}
