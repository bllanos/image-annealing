use super::super::super::resource::buffer::{BindableBuffer, InputBuffer, OutputBuffer};
use super::super::super::resource::manager::ResourceManager;
use super::super::shader::WorkgroupGridDimensions;
use super::{Binding, BindingData};
use image_annealing_shaders::binding::count_swap as binding_constants;

pub struct CountSwapBinding {
    binding_data: BindingData,
    workgroup_grid_dimensions: WorkgroupGridDimensions,
}

impl CountSwapBinding {
    pub fn new(device: &wgpu::Device, resources: &ResourceManager) -> Self {
        let count_swap_input_buffer = resources.count_swap_input_buffer();
        let count_swap_input_layout_buffer = resources.count_swap_input_layout_buffer();
        let count_swap_output_storage_buffer = resources.count_swap_output_storage_buffer();

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("count_swap_bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: binding_constants::PARAMETERS_INDEX,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: count_swap_input_layout_buffer.input_binding_description(),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: binding_constants::INPUT_BUFFER_INDEX,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: count_swap_input_buffer.input_binding_description(),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: binding_constants::OUTPUT_BUFFER_INDEX,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: count_swap_output_storage_buffer.output_binding_description(),
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("count_swap_bind_group"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: binding_constants::PARAMETERS_INDEX,
                    resource: count_swap_input_layout_buffer.binding_resource(),
                },
                wgpu::BindGroupEntry {
                    binding: binding_constants::INPUT_BUFFER_INDEX,
                    resource: count_swap_input_buffer.binding_resource(),
                },
                wgpu::BindGroupEntry {
                    binding: binding_constants::OUTPUT_BUFFER_INDEX,
                    resource: count_swap_output_storage_buffer.binding_resource(),
                },
            ],
        });

        Self {
            binding_data: BindingData { layout, bind_group },
            workgroup_grid_dimensions: WorkgroupGridDimensions::count_swap(),
        }
    }

    pub fn workgroup_grid_dimensions(&self) -> &WorkgroupGridDimensions {
        &self.workgroup_grid_dimensions
    }
}

impl Binding for CountSwapBinding {
    fn layout(&self) -> &wgpu::BindGroupLayout {
        &self.binding_data.layout
    }
    fn bind<'a: 'b, 'b>(&'a self, index: u32, cpass: &mut wgpu::ComputePass<'b>) {
        self.binding_data.bind(index, cpass)
    }
}
