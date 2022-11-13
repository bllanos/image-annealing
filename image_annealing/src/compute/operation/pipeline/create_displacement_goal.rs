use super::super::binding::manager::BindingManager;
use super::super::shader;
use image_annealing_shader::SHADER_ENTRY_POINT;

pub struct CreateDisplacementGoalPipeline {
    pipeline: wgpu::ComputePipeline,
}

impl CreateDisplacementGoalPipeline {
    pub fn new(device: &wgpu::Device, bindings: &BindingManager) -> Self {
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("create_displacement_goal_pipeline_layout"),
            bind_group_layouts: &[bindings.create_displacement_goal_layout()],
            push_constant_ranges: &[],
        });
        let shader = shader::create_displacement_goal_default_shader(device);
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("create_displacement_goal_pipeline"),
            layout: Some(&layout),
            module: shader.shader(),
            entry_point: SHADER_ENTRY_POINT,
        });
        Self { pipeline }
    }

    pub fn set_pipeline<'a: 'b, 'b>(&'a self, cpass: &mut wgpu::ComputePass<'b>) {
        cpass.set_pipeline(&self.pipeline);
        cpass.insert_debug_marker("create_displacement_goal_execution");
    }
}
