use super::super::resource::texture::Texture;
use super::shader::WorkgroupDimensions;
use std::marker::PhantomData;

mod create_permutation;
pub mod manager;
mod permute;

struct Binding<T> {
    layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    workgroup_dimensions: WorkgroupDimensions,
    phantom: PhantomData<T>,
}

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

fn get_workgroup_dimensions<T: Texture>(texture: &T) -> WorkgroupDimensions {
    let dimensions = texture.dimensions();
    WorkgroupDimensions::new(dimensions.width, dimensions.height)
}
