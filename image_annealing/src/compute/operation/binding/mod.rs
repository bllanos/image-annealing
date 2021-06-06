use super::super::resource::texture::Texture;
use super::shader::WorkgroupDimensions;

mod create_permutation;
pub mod manager;
mod permute;

trait Binding {
    fn layout(&self) -> &wgpu::BindGroupLayout;
    fn workgroup_dimensions(&self) -> &WorkgroupDimensions;
    fn bind<'a: 'b, 'b>(&'a self, index: u32, cpass: &mut wgpu::ComputePass<'b>);
}

struct BindingData {
    layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    workgroup_dimensions: WorkgroupDimensions,
}

impl BindingData {
    fn bind<'a: 'b, 'b>(&'a self, index: u32, cpass: &mut wgpu::ComputePass<'b>) {
        cpass.set_bind_group(index, &self.bind_group, &[]);
    }
}

fn get_workgroup_dimensions<T: Texture>(texture: &T) -> WorkgroupDimensions {
    let dimensions = texture.dimensions();
    WorkgroupDimensions::new(dimensions.width, dimensions.height)
}
