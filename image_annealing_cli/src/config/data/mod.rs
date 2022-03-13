use super::io::{convert_and_check_input_path, convert_path_separators};
use crate::{CandidatePermutationPath, DisplacementGoalPath, ImagePath};
use image_annealing::compute::DimensionsMismatchError;
use image_annealing::ImageDimensions;
use serde::Deserialize;
use std::convert::TryFrom;
use std::error::Error;

#[derive(Deserialize)]
pub enum UnverifiedConfig {
    CreatePermutation {
        image_width: usize,
        image_height: usize,
        permutation_output_path_no_extension: String,
    },
    Permute {
        candidate_permutation: String,
        original_image: String,
        permuted_image_output_path_no_extension: String,
    },
    Swap {
        candidate_permutation: String,
        displacement_goal: String,
        permutation_output_path_no_extension: String,
    },
    ValidatePermutation {
        candidate_permutation: String,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Config {
    CreatePermutation {
        image_dimensions: ImageDimensions,
        permutation_output_path_no_extension: String,
    },
    Permute {
        candidate_permutation: CandidatePermutationPath,
        original_image: ImagePath,
        permuted_image_output_path_no_extension: String,
    },
    Swap {
        candidate_permutation: CandidatePermutationPath,
        displacement_goal: DisplacementGoalPath,
        permutation_output_path_no_extension: String,
    },
    ValidatePermutation {
        candidate_permutation: CandidatePermutationPath,
    },
}

impl TryFrom<UnverifiedConfig> for Config {
    type Error = Box<dyn Error>;

    fn try_from(value: UnverifiedConfig) -> Result<Self, Self::Error> {
        Ok(match value {
            UnverifiedConfig::CreatePermutation {
                image_width,
                image_height,
                permutation_output_path_no_extension,
            } => Config::CreatePermutation {
                image_dimensions: ImageDimensions::new(image_width, image_height)?,
                permutation_output_path_no_extension: convert_path_separators(
                    permutation_output_path_no_extension,
                ),
            },
            UnverifiedConfig::Permute {
                candidate_permutation,
                original_image,
                permuted_image_output_path_no_extension,
            } => {
                let candidate_permutation_checked =
                    convert_and_check_input_path(candidate_permutation)?;
                let original_image_checked = convert_and_check_input_path(original_image)?;
                let image_dimensions = ImageDimensions::from_image_path(&original_image_checked)?;
                let permutation_dimensions =
                    ImageDimensions::from_image_path(&candidate_permutation_checked)?;
                if image_dimensions == permutation_dimensions {
                    Config::Permute {
                        candidate_permutation: CandidatePermutationPath(
                            candidate_permutation_checked,
                        ),
                        original_image: ImagePath(original_image_checked),
                        permuted_image_output_path_no_extension: convert_path_separators(
                            permuted_image_output_path_no_extension,
                        ),
                    }
                } else {
                    return Err(Box::new(DimensionsMismatchError::new(
                        image_dimensions,
                        permutation_dimensions,
                    )));
                }
            }
            UnverifiedConfig::Swap {
                candidate_permutation,
                displacement_goal,
                permutation_output_path_no_extension,
            } => {
                let candidate_permutation_checked =
                    convert_and_check_input_path(candidate_permutation)?;
                let displacement_goal_checked = convert_and_check_input_path(displacement_goal)?;
                let displacement_goal_dimensions =
                    ImageDimensions::from_image_path(&displacement_goal_checked)?;
                let permutation_dimensions =
                    ImageDimensions::from_image_path(&candidate_permutation_checked)?;
                if displacement_goal_dimensions == permutation_dimensions {
                    Config::Swap {
                        candidate_permutation: CandidatePermutationPath(
                            candidate_permutation_checked,
                        ),
                        displacement_goal: DisplacementGoalPath(displacement_goal_checked),
                        permutation_output_path_no_extension: convert_path_separators(
                            permutation_output_path_no_extension,
                        ),
                    }
                } else {
                    return Err(Box::new(DimensionsMismatchError::new(
                        displacement_goal_dimensions,
                        permutation_dimensions,
                    )));
                }
            }
            UnverifiedConfig::ValidatePermutation {
                candidate_permutation,
            } => Config::ValidatePermutation {
                candidate_permutation: CandidatePermutationPath(convert_and_check_input_path(
                    candidate_permutation,
                )?),
            },
        })
    }
}
