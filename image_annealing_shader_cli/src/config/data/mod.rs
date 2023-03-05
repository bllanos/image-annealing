use image_annealing_cli_util::io;
use image_annealing_shader::CreateDisplacementGoalShaderContent;
use serde::Deserialize;
use std::borrow::Cow;
use std::error::Error;
use std::fs;

#[derive(Deserialize)]
pub struct UnverifiedCreateDisplacementGoalConfig {
    pub body: String,
}

impl<'a> TryFrom<UnverifiedCreateDisplacementGoalConfig>
    for CreateDisplacementGoalShaderContent<'a>
{
    type Error = Box<dyn Error>;

    fn try_from(value: UnverifiedCreateDisplacementGoalConfig) -> Result<Self, Self::Error> {
        let path = io::convert_and_check_input_file_path(value.body)?;
        Ok(CreateDisplacementGoalShaderContent {
            body: Cow::Owned(fs::read_to_string(path)?),
        })
    }
}

#[derive(Deserialize)]
pub enum UnverifiedConfig {
    CreateDisplacementGoal(UnverifiedCreateDisplacementGoalConfig),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Config<'a> {
    CreateDisplacementGoal(CreateDisplacementGoalShaderContent<'a>),
}

impl<'a> TryFrom<UnverifiedConfig> for Config<'a> {
    type Error = Box<dyn Error>;

    fn try_from(value: UnverifiedConfig) -> Result<Self, Self::Error> {
        Ok(match value {
            UnverifiedConfig::CreateDisplacementGoal(inner_value) => {
                Self::CreateDisplacementGoal(inner_value.try_into()?)
            }
        })
    }
}

#[cfg(test)]
mod tests;
