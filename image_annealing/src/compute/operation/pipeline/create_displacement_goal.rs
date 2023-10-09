use super::super::binding::manager::BindingManager;
use super::super::shader;
use image_annealing_shader::{CreateDisplacementGoalShaderContent, SHADER_ENTRY_POINT};
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateDisplacementGoalShaderConfig<'a> {
    pub content: CreateDisplacementGoalShaderContent<'a>,
    pub entry_point: Cow<'a, str>,
}

impl CreateDisplacementGoalShaderConfig<'_> {
    // Ideally, this would be a method, but it may not be possible to return a `Self` type parameterized with a lifetime.
    // (See https://stackoverflow.com/questions/57701914/trait-method-which-returns-self-type-with-a-different-type-and-or-lifetime-par)
    pub fn to_owned(
        instance: &CreateDisplacementGoalShaderConfig,
    ) -> CreateDisplacementGoalShaderConfig<'static> {
        CreateDisplacementGoalShaderConfig {
            content: CreateDisplacementGoalShaderContent::to_owned(&instance.content),
            entry_point: Cow::Owned(instance.entry_point.clone().into_owned()),
        }
    }
}

pub struct CreateDisplacementGoalPipeline {
    pipeline_layout: wgpu::PipelineLayout,
    pipeline: Option<wgpu::ComputePipeline>,
    shader_config: Option<CreateDisplacementGoalShaderConfig<'static>>,
}

impl CreateDisplacementGoalPipeline {
    pub fn new(device: &wgpu::Device, bindings: &BindingManager) -> Self {
        Self {
            pipeline_layout: device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("create_displacement_goal_pipeline_layout"),
                bind_group_layouts: &[bindings.create_displacement_goal_layout()],
                push_constant_ranges: &[],
            }),
            pipeline: None,
            shader_config: None,
        }
    }

    pub fn has_pipeline(&self) -> bool {
        debug_assert!(self.pipeline.is_some() || self.shader_config.is_none());
        self.pipeline.is_some()
    }

    pub fn set_shader(
        &mut self,
        device: &wgpu::Device,
        config: Option<CreateDisplacementGoalShaderConfig<'static>>,
    ) {
        let pipeline_has_changed = match self.pipeline {
            Some(_) => self.shader_config != config,
            None => {
                debug_assert!(self.shader_config.is_none());
                true
            }
        };
        if pipeline_has_changed {
            let (shader, entry_point) = match config {
                Some(CreateDisplacementGoalShaderConfig {
                    ref content,
                    ref entry_point,
                }) => (
                    shader::create_displacement_goal_custom_shader(device, content),
                    entry_point.as_ref(),
                ),
                None => (
                    shader::create_displacement_goal_default_shader(device),
                    SHADER_ENTRY_POINT,
                ),
            };
            self.pipeline = Some(device.create_compute_pipeline(
                &wgpu::ComputePipelineDescriptor {
                    label: Some("create_displacement_goal_pipeline"),
                    layout: Some(&self.pipeline_layout),
                    module: shader.shader(),
                    entry_point,
                },
            ));
            self.shader_config = config;
        }
    }

    pub fn set_pipeline<'a: 'b, 'b>(&'a self, cpass: &mut wgpu::ComputePass<'b>) {
        cpass.set_pipeline(self.pipeline.as_ref().unwrap());
        cpass.insert_debug_marker("create_displacement_goal_execution");
    }
}
