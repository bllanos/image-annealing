use super::super::{
    DisplacementGoalPath, ImagePath, LosslessImagePath, PermutationPath,
    UnverifiedImageDimensionsConfig, UnverifiedLosslessImagePath,
};
use image_annealing::ImageDimensions;
use serde::Deserialize;
use std::error::Error;
use std::fmt;

#[derive(Default, Deserialize)]
pub struct UnverifiedCreateDisplacementGoalInputDataConfig {
    displacement_goal: Option<String>,
    candidate_permutation: Option<String>,
    image: Option<UnverifiedLosslessImagePath>,
}

#[derive(Deserialize)]
pub enum UnverifiedCreateDisplacementGoalInputConfig {
    ImageDimensions(UnverifiedImageDimensionsConfig),
    Input(UnverifiedCreateDisplacementGoalInputDataConfig),
}

#[derive(Debug, Clone)]
struct NoInputDataError;

impl fmt::Display for NoInputDataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "at least one input must be provided when specifying input data as opposed to image dimensions")
    }
}

impl Error for NoInputDataError {}

fn check_dimensions<T: IntoIterator<Item = ImageDimensions>>(
    dimensions: T,
) -> Result<ImageDimensions, Box<dyn Error>> {
    let mut iterator = dimensions.into_iter();
    match iterator.next() {
        Some(first_dimensions) => iterator.try_fold(first_dimensions, |dim1, dim2| {
            super::super::check_dimensions_match2(&dim1, &dim2).map(|&dim| dim)
        }),
        None => Err(Box::new(NoInputDataError)),
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct CreateDisplacementGoalInputConfig {
    pub displacement_goal: Option<DisplacementGoalPath>,
    pub candidate_permutation: Option<PermutationPath>,
    pub image: Option<LosslessImagePath>,
}

impl CreateDisplacementGoalInputConfig {
    pub fn from_config(
        config: UnverifiedCreateDisplacementGoalInputConfig,
    ) -> Result<(Self, ImageDimensions), Box<dyn Error>> {
        Ok(match config {
            UnverifiedCreateDisplacementGoalInputConfig::ImageDimensions(
                image_dimensions_config,
            ) => (Default::default(), image_dimensions_config.try_into()?),
            UnverifiedCreateDisplacementGoalInputConfig::Input(value) => {
                let mut displacement_goal: Option<DisplacementGoalPath> = None;
                let mut candidate_permutation: Option<PermutationPath> = None;
                let mut image: Option<LosslessImagePath> = None;
                let mut dimensions: Vec<ImageDimensions> = Vec::new();
                if let Some(path) = value.displacement_goal {
                    let (displacement_goal_checked, displacement_goal_dimensions) =
                        DisplacementGoalPath::from_input_path(path)?;
                    displacement_goal = Some(displacement_goal_checked);
                    dimensions.push(displacement_goal_dimensions);
                }
                if let Some(path) = value.candidate_permutation {
                    let (candidate_permutation_checked, permutation_dimensions) =
                        PermutationPath::from_input_path(path)?;
                    candidate_permutation = Some(candidate_permutation_checked);
                    dimensions.push(permutation_dimensions);
                }
                if let Some(path) = value.image {
                    let (image_checked, image_dimensions) =
                        LosslessImagePath::from_input_path(path)?;
                    image = Some(image_checked);
                    dimensions.push(image_dimensions);
                }
                (
                    Self {
                        displacement_goal,
                        candidate_permutation,
                        image,
                    },
                    check_dimensions(dimensions)?,
                )
            }
        })
    }
}

#[cfg(test)]
mod tests;
