use super::super::binding::manager::BindingManager;
use super::super::shader;

pub struct CreatePermutationPipeline {
    pipeline: wgpu::ComputePipeline,
}

impl CreatePermutationPipeline {
    pub fn new(device: &wgpu::Device, bindings: &BindingManager) -> Self {
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("create_permutation_pipeline_layout"),
            bind_group_layouts: &[bindings.create_permutation_layout()],
            push_constant_ranges: &[],
        });
        let shader = shader::create_permutation_shader(device);
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("create_permutation_pipeline"),
            layout: Some(&layout),
            module: shader.shader(),
            entry_point: image_annealing_build_utils::SHADER_ENTRY_POINT,
        });
        Self { pipeline }
    }

    pub fn set_pipeline<'a: 'b, 'b>(&'a self, cpass: &mut wgpu::ComputePass<'b>) {
        cpass.set_pipeline(&self.pipeline);
        cpass.insert_debug_marker("create_permutation_execution");
    }
}
