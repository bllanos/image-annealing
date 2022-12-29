use image_annealing_cli_util::io;
use image_annealing_shader::shader::CreateDisplacementGoalShaderContent;
use serde::Deserialize;
use std::borrow::Cow;
use std::error::Error;
use std::fs;

#[derive(Deserialize)]
pub enum UnverifiedConfig {
    CreateDisplacementGoal { body: String },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Config<'a> {
    CreateDisplacementGoal(CreateDisplacementGoalShaderContent<'a>),
}

impl<'a> TryFrom<UnverifiedConfig> for Config<'a> {
    type Error = Box<dyn Error>;

    fn try_from(value: UnverifiedConfig) -> Result<Self, Self::Error> {
        Ok(match value {
            UnverifiedConfig::CreateDisplacementGoal {
                body: unverified_body_path,
            } => {
                let path = io::convert_and_check_input_file_path(unverified_body_path)?;
                Self::CreateDisplacementGoal(CreateDisplacementGoalShaderContent {
                    body: Cow::Owned(fs::read_to_string(path)?),
                })
            }
        })
    }
}

#[cfg(test)]
mod tests;
