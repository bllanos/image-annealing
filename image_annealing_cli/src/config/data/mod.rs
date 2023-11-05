use image_annealing::{compute, DimensionsMismatchError, ImageDimensions};
use image_annealing_cli_util::path::{TryFromWithPathContext, TryIntoWithPathContext};
use serde::Deserialize;
use std::error::Error;
use std::path::Path;

mod dimension;
mod filepath;
mod input;
mod number;
mod parameters;

pub use dimension::UnverifiedImageDimensionsConfig;
pub use filepath::{
    InputDisplacementGoalPath, InputLosslessImagePath, InputPermutationPath,
    OutputDisplacementGoalPath, OutputLosslessImagePath, OutputPermutationPath,
    UnverifiedInputDisplacementGoalPath, UnverifiedInputLosslessImagePath,
    UnverifiedInputPermutationPath, UnverifiedOutputDisplacementGoalPath,
    UnverifiedOutputLosslessImagePath, UnverifiedOutputPermutationPath,
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
pub enum UnverifiedConfig<'a> {
    CreateDisplacementGoal {
        input: UnverifiedCreateDisplacementGoalInputConfig<'a>,
        displacement_goal_output_path_no_extension: UnverifiedOutputDisplacementGoalPath<'a>,
    },
    CreatePermutation {
        image_dimensions: UnverifiedImageDimensionsConfig,
        permutation_output_path_no_extension: UnverifiedOutputPermutationPath<'a>,
    },
    Permute {
        candidate_permutation: UnverifiedInputPermutationPath<'a>,
        original_image: UnverifiedInputLosslessImagePath<'a>,
        permuted_image_output_path_no_extension: UnverifiedOutputLosslessImagePath<'a>,
    },
    Swap {
        candidate_permutation: UnverifiedInputPermutationPath<'a>,
        displacement_goal: UnverifiedInputDisplacementGoalPath<'a>,
        permutation_output_path_prefix: UnverifiedOutputPermutationPath<'a>,
        parameters: UnverifiedSwapParametersConfig,
    },
    ValidatePermutation {
        candidate_permutation: UnverifiedInputPermutationPath<'a>,
    },
}

#[derive(Debug, PartialEq)]
pub enum AlgorithmConfig<'a> {
    CreateDisplacementGoal {
        input: CreateDisplacementGoalInputConfig<'a>,
        displacement_goal_output_path_no_extension: OutputDisplacementGoalPath<'a>,
    },
    CreatePermutation {
        permutation_output_path_no_extension: OutputPermutationPath<'a>,
    },
    Permute {
        candidate_permutation: InputPermutationPath<'a>,
        original_image: InputLosslessImagePath<'a>,
        permuted_image_output_path_no_extension: OutputLosslessImagePath<'a>,
    },
    Swap {
        candidate_permutation: InputPermutationPath<'a>,
        displacement_goal: InputDisplacementGoalPath<'a>,
        permutation_output_path_prefix: OutputPermutationPath<'a>,
        parameters: SwapParametersConfig,
    },
    ValidatePermutation {
        candidate_permutation: InputPermutationPath<'a>,
    },
}

#[derive(Debug, PartialEq)]
pub struct Config<'a> {
    pub algorithm: AlgorithmConfig<'a>,
    pub dispatcher: compute::Config,
}

impl TryFromWithPathContext<UnverifiedConfig<'_>> for Config<'static> {
    type Error = Box<dyn Error>;

    fn try_from_with_path_context<P: AsRef<Path>>(
        value: UnverifiedConfig,
        base_path: P,
    ) -> Result<Self, Self::Error> {
        let (algorithm_config, image_dimensions) = match value {
            UnverifiedConfig::CreateDisplacementGoal {
                input,
                displacement_goal_output_path_no_extension,
            } => {
                let (input_checked, image_dimensions) =
                    CreateDisplacementGoalInputConfig::try_from_unverified_with_path_context(
                        input, &base_path,
                    )?;
                (
                    AlgorithmConfig::CreateDisplacementGoal {
                        input: input_checked,
                        displacement_goal_output_path_no_extension:
                            displacement_goal_output_path_no_extension
                                .try_into_with_path_context(&base_path)?,
                    },
                    image_dimensions,
                )
            }
            UnverifiedConfig::CreatePermutation {
                image_dimensions,
                permutation_output_path_no_extension,
            } => (
                AlgorithmConfig::CreatePermutation {
                    permutation_output_path_no_extension: permutation_output_path_no_extension
                        .try_into_with_path_context(&base_path)?,
                },
                image_dimensions.try_into()?,
            ),
            UnverifiedConfig::Permute {
                candidate_permutation,
                original_image,
                permuted_image_output_path_no_extension,
            } => {
                let (candidate_permutation_checked, permutation_dimensions) =
                    InputPermutationPath::try_from_unverified_with_path_context(
                        candidate_permutation,
                        &base_path,
                    )?;
                let (original_image_checked, image_dimensions) =
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        original_image,
                        &base_path,
                    )?;
                check_dimensions_match2(&image_dimensions, &permutation_dimensions)?;
                (
                    AlgorithmConfig::Permute {
                        candidate_permutation: candidate_permutation_checked,
                        original_image: original_image_checked,
                        permuted_image_output_path_no_extension:
                            permuted_image_output_path_no_extension
                                .try_into_with_path_context(&base_path)?,
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
                    InputPermutationPath::try_from_unverified_with_path_context(
                        candidate_permutation,
                        &base_path,
                    )?;
                let (displacement_goal_checked, displacement_goal_dimensions) =
                    InputDisplacementGoalPath::try_from_unverified_with_path_context(
                        displacement_goal,
                        &base_path,
                    )?;
                check_dimensions_match2(&permutation_dimensions, &displacement_goal_dimensions)?;
                (
                    AlgorithmConfig::Swap {
                        candidate_permutation: candidate_permutation_checked,
                        displacement_goal: displacement_goal_checked,
                        permutation_output_path_prefix: permutation_output_path_prefix
                            .try_into_with_path_context(&base_path)?,
                        parameters: parameters.try_into()?,
                    },
                    permutation_dimensions,
                )
            }
            UnverifiedConfig::ValidatePermutation {
                candidate_permutation,
            } => {
                let (candidate_permutation_path, image_dimensions) =
                    InputPermutationPath::try_from_unverified_with_path_context(
                        candidate_permutation,
                        &base_path,
                    )?;
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
