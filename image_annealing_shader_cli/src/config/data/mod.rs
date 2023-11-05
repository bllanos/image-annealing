use image_annealing_cli_util::path::{
    InputFilePath, TryFromWithPathContext, TryIntoWithPathContext, UnverifiedInputFilePath,
};
use image_annealing_shader::CreateDisplacementGoalShaderContent;
use serde::Deserialize;
use std::borrow::Cow;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Clone, Deserialize)]
pub struct UnverifiedCreateDisplacementGoalConfig<'a> {
    pub body: UnverifiedInputFilePath<'a>,
}

impl TryFromWithPathContext<UnverifiedCreateDisplacementGoalConfig<'_>>
    for CreateDisplacementGoalShaderContent<'static>
{
    type Error = Box<dyn Error>;

    fn try_from_with_path_context<P: AsRef<Path>>(
        value: UnverifiedCreateDisplacementGoalConfig,
        base_path: P,
    ) -> Result<Self, Self::Error> {
        let path = InputFilePath::try_from_with_path_context(value.body, base_path)?;
        Ok(CreateDisplacementGoalShaderContent {
            body: Cow::Owned(fs::read_to_string(path.0)?),
        })
    }
}

#[derive(Deserialize)]
pub enum UnverifiedConfig<'a> {
    CreateDisplacementGoal(UnverifiedCreateDisplacementGoalConfig<'a>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Config<'a> {
    CreateDisplacementGoal(CreateDisplacementGoalShaderContent<'a>),
}

impl TryFromWithPathContext<UnverifiedConfig<'_>> for Config<'static> {
    type Error = Box<dyn Error>;

    fn try_from_with_path_context<P: AsRef<Path>>(
        value: UnverifiedConfig,
        base_path: P,
    ) -> Result<Self, Self::Error> {
        Ok(match value {
            UnverifiedConfig::CreateDisplacementGoal(inner_value) => {
                Self::CreateDisplacementGoal(inner_value.try_into_with_path_context(base_path)?)
            }
        })
    }
}

#[cfg(test)]
mod tests;
