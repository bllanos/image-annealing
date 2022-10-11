mod count_swap;
mod create_displacement_goal;
mod create_permutation;
pub mod manager;
mod permute;
mod swap;

trait Binding {
    fn layout(&self) -> &wgpu::BindGroupLayout;
    fn bind<'a: 'b, 'b>(&'a self, index: u32, cpass: &mut wgpu::ComputePass<'b>);
}

struct BindingData {
    layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
}

impl BindingData {
    fn bind<'a: 'b, 'b>(&'a self, index: u32, cpass: &mut wgpu::ComputePass<'b>) {
        cpass.set_bind_group(index, &self.bind_group, &[]);
    }
}
