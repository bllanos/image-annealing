use super::pipeline::{UnverifiedPipelineConfig, UnverifiedPipelineOperationConfig};
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

impl TryFrom<UnverifiedPipelineConfig<UnverifiedCreateDisplacementGoalShaderConfig>>
    for PipelineConfig<CreateDisplacementGoalShaderConfig<'_>>
{
    type Error = Box<dyn Error>;

    fn try_from(
        value: UnverifiedPipelineConfig<UnverifiedCreateDisplacementGoalShaderConfig>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            shader_config: value.shader_config.try_into()?,
            workgroup_grid: value.workgroup_grid.try_into()?,
        })
    }
}

pub type UnverifiedCreateDisplacementGoalPipelineOperationConfig =
    UnverifiedPipelineOperationConfig<UnverifiedCreateDisplacementGoalShaderConfig>;

impl TryFrom<UnverifiedCreateDisplacementGoalPipelineOperationConfig>
    for CreateDisplacementGoalPipelineOperation<'_>
{
    type Error = Box<dyn Error>;

    fn try_from(
        value: UnverifiedCreateDisplacementGoalPipelineOperationConfig,
    ) -> Result<Self, Self::Error> {
        Ok(match value {
            UnverifiedPipelineOperationConfig::Set(inner_value) => {
                Self::Set(inner_value.try_into()?)
            }
            UnverifiedPipelineOperationConfig::SetDefault => Self::SetDefault,
        })
    }
}

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
            pipeline_operation: value.pipeline_operation.try_into()?,
        })
    }
}
