use super::pipeline::UnverifiedPipelineOperationConfig;
use image_annealing::compute::{
    CreateDisplacementGoalParameters, CreateDisplacementGoalPipelineOperation,
    CreateDisplacementGoalShaderConfig, PipelineConfig,
};
use image_annealing_shader_cli::config::UnverifiedCreateDisplacementGoalConfig;
use serde::Deserialize;
use std::borrow::Cow;
use std::error::Error;

#[derive(Clone, Deserialize)]
pub struct UnverifiedCreateDisplacementGoalShaderConfig {
    pub content: UnverifiedCreateDisplacementGoalConfig,
    pub entry_point: String,
}

impl TryFrom<UnverifiedCreateDisplacementGoalShaderConfig>
    for CreateDisplacementGoalShaderConfig<'_>
{
    type Error = Box<dyn Error>;

    fn try_from(value: UnverifiedCreateDisplacementGoalShaderConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            content: value.content.try_into()?,
            entry_point: Cow::Owned(value.entry_point),
        })
    }
}

pub type UnverifiedCreateDisplacementGoalPipelineOperationConfig =
    UnverifiedPipelineOperationConfig<UnverifiedCreateDisplacementGoalShaderConfig>;

#[derive(Clone, Default, Deserialize)]
pub struct UnverifiedCreateDisplacementGoalParametersConfig {
    pub pipeline_operation: UnverifiedCreateDisplacementGoalPipelineOperationConfig,
}

impl TryFrom<UnverifiedCreateDisplacementGoalParametersConfig>
    for CreateDisplacementGoalParameters<'_>
{
    type Error = Box<dyn Error>;

    fn try_from(
        value: UnverifiedCreateDisplacementGoalParametersConfig,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            pipeline_operation: match value.pipeline_operation {
                UnverifiedPipelineOperationConfig::Set(inner_value) => {
                    CreateDisplacementGoalPipelineOperation::Set(PipelineConfig {
                        shader_config: inner_value.shader_config.try_into()?,
                        workgroup_grid: inner_value.workgroup_grid.try_into()?,
                    })
                }
                UnverifiedPipelineOperationConfig::SetDefault => {
                    CreateDisplacementGoalPipelineOperation::SetDefault
                }
            },
        })
    }
}
