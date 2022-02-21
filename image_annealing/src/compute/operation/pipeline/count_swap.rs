use super::super::binding::manager::BindingManager;
use super::super::shader;
use image_annealing_shaders::SHADER_ENTRY_POINT;

pub struct CountSwapPipeline {
    pipeline: wgpu::ComputePipeline,
}

impl CountSwapPipeline {
    pub fn new(device: &wgpu::Device, bindings: &BindingManager) -> Self {
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("count_swap_pipeline_layout"),
            bind_group_layouts: &[bindings.count_swap_layout()],
            push_constant_ranges: &[],
        });
        let shader = shader::count_swap_shader(device);
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("count_swap_pipeline"),
            layout: Some(&layout),
            module: shader.shader(),
            entry_point: SHADER_ENTRY_POINT,
        });
        Self { pipeline }
    }

    pub fn set_pipeline<'a: 'b, 'b>(&'a self, cpass: &mut wgpu::ComputePass<'b>) {
        cpass.set_pipeline(&self.pipeline);
        cpass.insert_debug_marker("count_swap_execution");
    }
}
