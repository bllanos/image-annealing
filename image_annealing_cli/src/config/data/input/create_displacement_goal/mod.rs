use super::super::{
    InputDisplacementGoalPath, InputLosslessImagePath, InputPermutationPath,
    UnverifiedImageDimensionsConfig, UnverifiedInputDisplacementGoalPath,
    UnverifiedInputLosslessImagePath, UnverifiedInputPermutationPath,
};
use image_annealing::ImageDimensions;
use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::path::Path;

#[derive(Default, Deserialize)]
pub struct UnverifiedCreateDisplacementGoalInputDataConfig<'a> {
    displacement_goal: Option<UnverifiedInputDisplacementGoalPath<'a>>,
    candidate_permutation: Option<UnverifiedInputPermutationPath<'a>>,
    image: Option<UnverifiedInputLosslessImagePath<'a>>,
}

#[derive(Deserialize)]
pub enum UnverifiedCreateDisplacementGoalInputConfig<'a> {
    ImageDimensions(UnverifiedImageDimensionsConfig),
    Input(UnverifiedCreateDisplacementGoalInputDataConfig<'a>),
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
pub struct CreateDisplacementGoalInputConfig<'a> {
    pub displacement_goal: Option<InputDisplacementGoalPath<'a>>,
    pub candidate_permutation: Option<InputPermutationPath<'a>>,
    pub image: Option<InputLosslessImagePath<'a>>,
}

impl CreateDisplacementGoalInputConfig<'static> {
    pub fn try_from_unverified_with_path_context<P: AsRef<Path>>(
        config: UnverifiedCreateDisplacementGoalInputConfig,
        base_path: P,
    ) -> Result<(Self, ImageDimensions), Box<dyn Error>> {
        Ok(match config {
            UnverifiedCreateDisplacementGoalInputConfig::ImageDimensions(
                image_dimensions_config,
            ) => (Default::default(), image_dimensions_config.try_into()?),
            UnverifiedCreateDisplacementGoalInputConfig::Input(value) => {
                let mut displacement_goal: Option<InputDisplacementGoalPath> = None;
                let mut candidate_permutation: Option<InputPermutationPath> = None;
                let mut image: Option<InputLosslessImagePath> = None;
                let mut dimensions: Vec<ImageDimensions> = Vec::new();
                if let Some(path) = value.displacement_goal {
                    let (displacement_goal_checked, displacement_goal_dimensions) =
                        InputDisplacementGoalPath::try_from_unverified_with_path_context(
                            path, &base_path,
                        )?;
                    displacement_goal = Some(displacement_goal_checked);
                    dimensions.push(displacement_goal_dimensions);
                }
                if let Some(path) = value.candidate_permutation {
                    let (candidate_permutation_checked, permutation_dimensions) =
                        InputPermutationPath::try_from_unverified_with_path_context(
                            path, &base_path,
                        )?;
                    candidate_permutation = Some(candidate_permutation_checked);
                    dimensions.push(permutation_dimensions);
                }
                if let Some(path) = value.image {
                    let (image_checked, image_dimensions) =
                        InputLosslessImagePath::try_from_unverified_with_path_context(
                            path, &base_path,
                        )?;
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
