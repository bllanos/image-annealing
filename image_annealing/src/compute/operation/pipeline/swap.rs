use super::super::binding::manager::BindingManager;
use super::super::shader;
use image_annealing_shader::SHADER_ENTRY_POINT;

pub struct SwapPipeline {
    pipeline: wgpu::ComputePipeline,
}

impl SwapPipeline {
    pub fn new(device: &wgpu::Device, bindings: &BindingManager) -> Self {
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("swap_pipeline_layout"),
            bind_group_layouts: &[bindings.swap_layout()],
            push_constant_ranges: &[],
        });
        let shader = shader::swap_shader(device);
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("swap_pipeline"),
            layout: Some(&layout),
            module: shader.shader(),
            entry_point: SHADER_ENTRY_POINT,
        });
        Self { pipeline }
    }

    pub fn set_pipeline<'a: 'b, 'b>(&'a self, cpass: &mut wgpu::ComputePass<'b>) {
        cpass.set_pipeline(&self.pipeline);
        cpass.insert_debug_marker("swap_execution");
    }
}
