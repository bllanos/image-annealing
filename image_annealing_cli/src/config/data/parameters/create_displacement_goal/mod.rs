use super::pipeline::{UnverifiedPipelineConfig, UnverifiedPipelineOperationConfig};
use image_annealing::compute::{
    CreateDisplacementGoalParameters, CreateDisplacementGoalPipelineOperation,
    CreateDisplacementGoalShaderConfig, PipelineConfig,
};
use image_annealing_cli_util::path::{TryFromWithPathContext, TryIntoWithPathContext};
use image_annealing_shader_cli::config::UnverifiedCreateDisplacementGoalConfig;
use serde::Deserialize;
use std::borrow::Cow;
use std::error::Error;
use std::path::Path;

#[derive(Clone, Deserialize)]
pub struct UnverifiedCreateDisplacementGoalShaderConfig<'a> {
    pub content: UnverifiedCreateDisplacementGoalConfig<'a>,
    pub entry_point: Cow<'a, str>,
}

impl<'a> TryFromWithPathContext<UnverifiedCreateDisplacementGoalShaderConfig<'a>>
    for CreateDisplacementGoalShaderConfig<'a>
{
    type Error = Box<dyn Error>;

    fn try_from_with_path_context<P: AsRef<Path>>(
        value: UnverifiedCreateDisplacementGoalShaderConfig<'a>,
        base_path: P,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            content: value.content.try_into_with_path_context(base_path)?,
            entry_point: value.entry_point,
        })
    }
}

impl<'a>
    TryFromWithPathContext<
        UnverifiedPipelineConfig<UnverifiedCreateDisplacementGoalShaderConfig<'a>>,
    > for PipelineConfig<CreateDisplacementGoalShaderConfig<'a>>
{
    type Error = Box<dyn Error>;

    fn try_from_with_path_context<P: AsRef<Path>>(
        value: UnverifiedPipelineConfig<UnverifiedCreateDisplacementGoalShaderConfig<'a>>,
        base_path: P,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            shader_config: value.shader_config.try_into_with_path_context(base_path)?,
            workgroup_grid: value.workgroup_grid.try_into()?,
        })
    }
}

pub type UnverifiedCreateDisplacementGoalPipelineOperationConfig<'a> =
    UnverifiedPipelineOperationConfig<UnverifiedCreateDisplacementGoalShaderConfig<'a>>;

impl<'a> TryFromWithPathContext<UnverifiedCreateDisplacementGoalPipelineOperationConfig<'a>>
    for CreateDisplacementGoalPipelineOperation<'a>
{
    type Error = Box<dyn Error>;

    fn try_from_with_path_context<P: AsRef<Path>>(
        value: UnverifiedCreateDisplacementGoalPipelineOperationConfig<'a>,
        base_path: P,
    ) -> Result<Self, Self::Error> {
        Ok(match value {
            UnverifiedPipelineOperationConfig::Set(inner_value) => {
                Self::Set(inner_value.try_into_with_path_context(base_path)?)
            }
            UnverifiedPipelineOperationConfig::SetDefault => Self::SetDefault,
        })
    }
}

#[derive(Clone, Default, Deserialize)]
pub struct UnverifiedCreateDisplacementGoalParametersConfig<'a> {
    pub pipeline_operation: UnverifiedCreateDisplacementGoalPipelineOperationConfig<'a>,
}

impl<'a> TryFromWithPathContext<UnverifiedCreateDisplacementGoalParametersConfig<'a>>
    for CreateDisplacementGoalParameters<'a>
{
    type Error = Box<dyn Error>;

    fn try_from_with_path_context<P: AsRef<Path>>(
        value: UnverifiedCreateDisplacementGoalParametersConfig<'a>,
        base_path: P,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            pipeline_operation: value
                .pipeline_operation
                .try_into_with_path_context(base_path)?,
        })
    }
}
