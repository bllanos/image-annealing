use image_annealing::{compute, DimensionsMismatchError, ImageDimensions};
use serde::Deserialize;
use std::error::Error;

mod dimension;
mod filepath;
mod input;
mod number;
mod parameters;

pub use dimension::UnverifiedImageDimensionsConfig;
pub use filepath::{
    DisplacementGoalPath, ImagePath, LosslessImagePath, PermutationPath,
    UnverifiedLosslessImagePath,
};
pub use input::{
    CreateDisplacementGoalInputConfig, UnverifiedCreateDisplacementGoalInputConfig,
    UnverifiedCreateDisplacementGoalInputDataConfig,
};
pub use number::{
    InvalidNonnegativeProperFractionError, InvalidNonnegativeRationalNumberError,
    NonnegativeProperFraction, NonnegativeRationalNumber,
};
pub use parameters::{
    InvalidIterationCountError, IterationCount, SwapParametersConfig, SwapPass, SwapStopConfig,
    SwapStopThreshold, UnverifiedIterationCount, UnverifiedSwapParametersConfig,
    UnverifiedSwapStopConfig, UnverifiedSwapStopThreshold,
};

fn check_dimensions_match2<'a>(
    dimensions1: &'a ImageDimensions,
    dimensions2: &'a ImageDimensions,
) -> Result<&'a ImageDimensions, Box<dyn Error>> {
    if dimensions1 == dimensions2 {
        Ok(dimensions1)
    } else {
        Err(Box::new(DimensionsMismatchError::new(
            *dimensions1,
            *dimensions2,
        )))
    }
}

#[derive(Deserialize)]
pub enum UnverifiedConfig {
    CreateDisplacementGoal {
        input: UnverifiedCreateDisplacementGoalInputConfig,
        displacement_goal_output_path_no_extension: String,
    },
    CreatePermutation {
        image_dimensions: UnverifiedImageDimensionsConfig,
        permutation_output_path_no_extension: String,
    },
    Permute {
        candidate_permutation: String,
        original_image: UnverifiedLosslessImagePath,
        permuted_image_output_path_no_extension: UnverifiedLosslessImagePath,
    },
    Swap {
        candidate_permutation: String,
        displacement_goal: String,
        permutation_output_path_prefix: String,
        parameters: UnverifiedSwapParametersConfig,
    },
    ValidatePermutation {
        candidate_permutation: String,
    },
}

#[derive(Debug, PartialEq)]
pub enum AlgorithmConfig {
    CreateDisplacementGoal {
        input: CreateDisplacementGoalInputConfig,
        displacement_goal_output_path_no_extension: DisplacementGoalPath,
    },
    CreatePermutation {
        permutation_output_path_no_extension: PermutationPath,
    },
    Permute {
        candidate_permutation: PermutationPath,
        original_image: LosslessImagePath,
        permuted_image_output_path_no_extension: LosslessImagePath,
    },
    Swap {
        candidate_permutation: PermutationPath,
        displacement_goal: DisplacementGoalPath,
        permutation_output_path_prefix: PermutationPath,
        parameters: SwapParametersConfig,
    },
    ValidatePermutation {
        candidate_permutation: PermutationPath,
    },
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub algorithm: AlgorithmConfig,
    pub dispatcher: compute::Config,
}

impl TryFrom<UnverifiedConfig> for Config {
    type Error = Box<dyn Error>;

    fn try_from(value: UnverifiedConfig) -> Result<Self, Self::Error> {
        let (algorithm_config, image_dimensions) = match value {
            UnverifiedConfig::CreateDisplacementGoal {
                input,
                displacement_goal_output_path_no_extension,
            } => {
                let (input_checked, image_dimensions) =
                    CreateDisplacementGoalInputConfig::from_config(input)?;
                (
                    AlgorithmConfig::CreateDisplacementGoal {
                        input: input_checked,
                        displacement_goal_output_path_no_extension:
                            DisplacementGoalPath::from_output_path(
                                displacement_goal_output_path_no_extension,
                            ),
                    },
                    image_dimensions,
                )
            }
            UnverifiedConfig::CreatePermutation {
                image_dimensions,
                permutation_output_path_no_extension,
            } => (
                AlgorithmConfig::CreatePermutation {
                    permutation_output_path_no_extension: PermutationPath::from_output_path(
                        permutation_output_path_no_extension,
                    ),
                },
                image_dimensions.try_into()?,
            ),
            UnverifiedConfig::Permute {
                candidate_permutation,
                original_image,
                permuted_image_output_path_no_extension,
            } => {
                let (candidate_permutation_checked, permutation_dimensions) =
                    PermutationPath::from_input_path(candidate_permutation)?;
                let (original_image_checked, image_dimensions) =
                    LosslessImagePath::from_input_path(original_image)?;
                check_dimensions_match2(&image_dimensions, &permutation_dimensions)?;
                (
                    AlgorithmConfig::Permute {
                        candidate_permutation: candidate_permutation_checked,
                        original_image: original_image_checked,
                        permuted_image_output_path_no_extension:
                            LosslessImagePath::from_output_path(
                                permuted_image_output_path_no_extension,
                            ),
                    },
                    image_dimensions,
                )
            }
            UnverifiedConfig::Swap {
                candidate_permutation,
                displacement_goal,
                permutation_output_path_prefix,
                parameters,
            } => {
                let (candidate_permutation_checked, permutation_dimensions) =
                    PermutationPath::from_input_path(candidate_permutation)?;
                let (displacement_goal_checked, displacement_goal_dimensions) =
                    DisplacementGoalPath::from_input_path(displacement_goal)?;
                check_dimensions_match2(&permutation_dimensions, &displacement_goal_dimensions)?;
                (
                    AlgorithmConfig::Swap {
                        candidate_permutation: candidate_permutation_checked,
                        displacement_goal: displacement_goal_checked,
                        permutation_output_path_prefix: PermutationPath::from_output_path(
                            permutation_output_path_prefix,
                        ),
                        parameters: parameters.try_into()?,
                    },
                    permutation_dimensions,
                )
            }
            UnverifiedConfig::ValidatePermutation {
                candidate_permutation,
            } => {
                let (candidate_permutation_path, image_dimensions) =
                    PermutationPath::from_input_path(candidate_permutation)?;
                (
                    AlgorithmConfig::ValidatePermutation {
                        candidate_permutation: candidate_permutation_path,
                    },
                    image_dimensions,
                )
            }
        };
        Ok(Config {
            algorithm: algorithm_config,
            dispatcher: compute::Config { image_dimensions },
        })
    }
}

#[cfg(test)]
mod tests;
