use super::super::binding::manager::BindingManager;
use super::super::shader;
use image_annealing_shaders::SHADER_ENTRY_POINT;

pub struct PermutePipeline {
    pipeline: wgpu::ComputePipeline,
}

impl PermutePipeline {
    pub fn new(device: &wgpu::Device, bindings: &BindingManager) -> Self {
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("permute_pipeline_layout"),
            bind_group_layouts: &[bindings.permute_layout()],
            push_constant_ranges: &[],
        });
        let shader = shader::permute_shader(device);
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("permute_pipeline"),
            layout: Some(&layout),
            module: shader.shader(),
            entry_point: SHADER_ENTRY_POINT,
        });
        Self { pipeline }
    }

    pub fn set_pipeline<'a: 'b, 'b>(&'a self, cpass: &mut wgpu::ComputePass<'b>) {
        cpass.set_pipeline(&self.pipeline);
        cpass.insert_debug_marker("permute_execution");
    }
}
