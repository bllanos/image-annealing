use super::io::{convert_and_check_input_path, convert_path_separators};
use image_annealing::compute::format::ImageFormat;
use image_annealing::{compute, DimensionsMismatchError, ImageDimensions};
use serde::Deserialize;
use std::convert::TryFrom;
use std::error::Error;
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
pub struct PermutationPath(pub String);

impl PermutationPath {
    pub fn from_input_path(
        unverified_path: String,
    ) -> Result<(Self, ImageDimensions), Box<dyn Error>> {
        let path = convert_and_check_input_path(unverified_path)?;
        let dimensions = ImageDimensions::from_image_path(&path)?;
        Ok((Self(path), dimensions))
    }

    pub fn from_output_path(path_no_extension: String) -> Self {
        Self(convert_path_separators(path_no_extension))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DisplacementGoalPath(pub String);

impl DisplacementGoalPath {
    pub fn from_input_path(
        unverified_path: String,
    ) -> Result<(Self, ImageDimensions), Box<dyn Error>> {
        let path = convert_and_check_input_path(unverified_path)?;
        let dimensions = ImageDimensions::from_image_path(&path)?;
        Ok((Self(path), dimensions))
    }

    pub fn from_output_path(path_no_extension: String) -> Self {
        Self(convert_path_separators(path_no_extension))
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum UnverifiedLosslessImagePath {
    Rgba8(String),
    Rgba8x2(String, String),
    Rgba8x3(String, String, String),
    Rgba8x4(String, String, String, String),
    Rgba16(String),
    Rgba16x2(String, String),
    Rgba16Rgba8(String, String),
    Rgba16Rgba8x2(String, String, String),
}

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

fn check_dimensions_match2_paths<T: AsRef<Path>, U: AsRef<Path>>(
    image_path1: &T,
    image_path2: &U,
) -> Result<ImageDimensions, Box<dyn Error>> {
    let dimensions1 = ImageDimensions::from_image_path(image_path1.as_ref())?;
    let dimensions2 = ImageDimensions::from_image_path(image_path2.as_ref())?;
    if dimensions1 == dimensions2 {
        Ok(dimensions1)
    } else {
        Err(Box::new(DimensionsMismatchError::new(
            dimensions1,
            dimensions2,
        )))
    }
}

fn check_dimensions_match3_paths<T: AsRef<Path>, U: AsRef<Path>, V: AsRef<Path>>(
    image_path1: &T,
    image_path2: &U,
    image_path3: &V,
) -> Result<ImageDimensions, Box<dyn Error>> {
    check_dimensions_match2_paths(image_path1, image_path2)?;
    check_dimensions_match2_paths(image_path1, image_path3)
}

fn check_dimensions_match4_paths<T: AsRef<Path>, U: AsRef<Path>, V: AsRef<Path>, W: AsRef<Path>>(
    image_path1: &T,
    image_path2: &U,
    image_path3: &V,
    image_path4: &W,
) -> Result<ImageDimensions, Box<dyn Error>> {
    check_dimensions_match3_paths(image_path1, image_path2, image_path3)?;
    check_dimensions_match2_paths(image_path1, image_path4)
}

#[derive(Debug, PartialEq, Eq)]
pub enum LosslessImagePath {
    Rgba8(String),
    Rgba8x2(String, String),
    Rgba8x3(String, String, String),
    Rgba8x4(String, String, String, String),
    Rgba16(String),
    Rgba16x2(String, String),
    Rgba16Rgba8(String, String),
    Rgba16Rgba8x2(String, String, String),
}

impl LosslessImagePath {
    pub fn format(&self) -> ImageFormat {
        match self {
            LosslessImagePath::Rgba8(..) => ImageFormat::Rgba8,
            LosslessImagePath::Rgba8x2(..) => ImageFormat::Rgba8x2,
            LosslessImagePath::Rgba8x3(..) => ImageFormat::Rgba8x3,
            LosslessImagePath::Rgba8x4(..) => ImageFormat::Rgba8x4,
            LosslessImagePath::Rgba16(..) => ImageFormat::Rgba16,
            LosslessImagePath::Rgba16x2(..) => ImageFormat::Rgba16x2,
            LosslessImagePath::Rgba16Rgba8(..) => ImageFormat::Rgba16Rgba8,
            LosslessImagePath::Rgba16Rgba8x2(..) => ImageFormat::Rgba16Rgba8x2,
        }
    }

    pub fn to_vec(&self) -> Vec<&String> {
        match self {
            LosslessImagePath::Rgba8(path) => vec![path],
            LosslessImagePath::Rgba8x2(path1, path2) => vec![path1, path2],
            LosslessImagePath::Rgba8x3(path1, path2, path3) => vec![path1, path2, path3],
            LosslessImagePath::Rgba8x4(path1, path2, path3, path4) => {
                vec![path1, path2, path3, path4]
            }
            LosslessImagePath::Rgba16(path) => vec![path],
            LosslessImagePath::Rgba16x2(path1, path2) => vec![path1, path2],
            LosslessImagePath::Rgba16Rgba8(path1, path2) => vec![path1, path2],
            LosslessImagePath::Rgba16Rgba8x2(path1, path2, path3) => vec![path1, path2, path3],
        }
    }

    pub fn from_input_path(
        path: UnverifiedLosslessImagePath,
    ) -> Result<(Self, ImageDimensions), Box<dyn Error>> {
        Ok(match path {
            UnverifiedLosslessImagePath::Rgba8(unverified_path) => {
                let path = convert_and_check_input_path(unverified_path)?;
                let dimensions = ImageDimensions::from_image_path(&path)?;
                (Self::Rgba8(path), dimensions)
            }
            UnverifiedLosslessImagePath::Rgba8x2(unverified_path1, unverified_path2) => {
                let path1 = convert_and_check_input_path(unverified_path1)?;
                let path2 = convert_and_check_input_path(unverified_path2)?;
                let dimensions = check_dimensions_match2_paths(&path1, &path2)?;
                (Self::Rgba8x2(path1, path2), dimensions)
            }
            UnverifiedLosslessImagePath::Rgba8x3(
                unverified_path1,
                unverified_path2,
                unverified_path3,
            ) => {
                let path1 = convert_and_check_input_path(unverified_path1)?;
                let path2 = convert_and_check_input_path(unverified_path2)?;
                let path3 = convert_and_check_input_path(unverified_path3)?;
                let dimensions = check_dimensions_match3_paths(&path1, &path2, &path3)?;
                (Self::Rgba8x3(path1, path2, path3), dimensions)
            }
            UnverifiedLosslessImagePath::Rgba8x4(
                unverified_path1,
                unverified_path2,
                unverified_path3,
                unverified_path4,
            ) => {
                let path1 = convert_and_check_input_path(unverified_path1)?;
                let path2 = convert_and_check_input_path(unverified_path2)?;
                let path3 = convert_and_check_input_path(unverified_path3)?;
                let path4 = convert_and_check_input_path(unverified_path4)?;
                let dimensions = check_dimensions_match4_paths(&path1, &path2, &path3, &path4)?;
                (Self::Rgba8x4(path1, path2, path3, path4), dimensions)
            }
            UnverifiedLosslessImagePath::Rgba16(unverified_path) => {
                let path = convert_and_check_input_path(unverified_path)?;
                let dimensions = ImageDimensions::from_image_path(&path)?;
                (Self::Rgba16(path), dimensions)
            }
            UnverifiedLosslessImagePath::Rgba16x2(unverified_path1, unverified_path2) => {
                let path1 = convert_and_check_input_path(unverified_path1)?;
                let path2 = convert_and_check_input_path(unverified_path2)?;
                let dimensions = check_dimensions_match2_paths(&path1, &path2)?;
                (Self::Rgba16x2(path1, path2), dimensions)
            }
            UnverifiedLosslessImagePath::Rgba16Rgba8(unverified_path1, unverified_path2) => {
                let path1 = convert_and_check_input_path(unverified_path1)?;
                let path2 = convert_and_check_input_path(unverified_path2)?;
                let dimensions = check_dimensions_match2_paths(&path1, &path2)?;
                (Self::Rgba16Rgba8(path1, path2), dimensions)
            }
            UnverifiedLosslessImagePath::Rgba16Rgba8x2(
                unverified_path1,
                unverified_path2,
                unverified_path3,
            ) => {
                let path1 = convert_and_check_input_path(unverified_path1)?;
                let path2 = convert_and_check_input_path(unverified_path2)?;
                let path3 = convert_and_check_input_path(unverified_path3)?;
                let dimensions = check_dimensions_match3_paths(&path1, &path2, &path3)?;
                (Self::Rgba16Rgba8x2(path1, path2, path3), dimensions)
            }
        })
    }

    pub fn from_output_path(path_no_extension: UnverifiedLosslessImagePath) -> Self {
        match path_no_extension {
            UnverifiedLosslessImagePath::Rgba8(unverified_path) => {
                Self::Rgba8(convert_path_separators(unverified_path))
            }
            UnverifiedLosslessImagePath::Rgba8x2(unverified_path1, unverified_path2) => {
                Self::Rgba8x2(
                    convert_path_separators(unverified_path1),
                    convert_path_separators(unverified_path2),
                )
            }
            UnverifiedLosslessImagePath::Rgba8x3(
                unverified_path1,
                unverified_path2,
                unverified_path3,
            ) => Self::Rgba8x3(
                convert_path_separators(unverified_path1),
                convert_path_separators(unverified_path2),
                convert_path_separators(unverified_path3),
            ),
            UnverifiedLosslessImagePath::Rgba8x4(
                unverified_path1,
                unverified_path2,
                unverified_path3,
                unverified_path4,
            ) => Self::Rgba8x4(
                convert_path_separators(unverified_path1),
                convert_path_separators(unverified_path2),
                convert_path_separators(unverified_path3),
                convert_path_separators(unverified_path4),
            ),
            UnverifiedLosslessImagePath::Rgba16(unverified_path) => {
                Self::Rgba16(convert_path_separators(unverified_path))
            }
            UnverifiedLosslessImagePath::Rgba16x2(unverified_path1, unverified_path2) => {
                Self::Rgba16x2(
                    convert_path_separators(unverified_path1),
                    convert_path_separators(unverified_path2),
                )
            }
            UnverifiedLosslessImagePath::Rgba16Rgba8(unverified_path1, unverified_path2) => {
                Self::Rgba16Rgba8(
                    convert_path_separators(unverified_path1),
                    convert_path_separators(unverified_path2),
                )
            }
            UnverifiedLosslessImagePath::Rgba16Rgba8x2(
                unverified_path1,
                unverified_path2,
                unverified_path3,
            ) => Self::Rgba16Rgba8x2(
                convert_path_separators(unverified_path1),
                convert_path_separators(unverified_path2),
                convert_path_separators(unverified_path3),
            ),
        }
    }
}

#[derive(Deserialize)]
pub enum UnverifiedConfig {
    CreatePermutation {
        image_width: usize,
        image_height: usize,
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
        permutation_output_path_no_extension: String,
    },
    ValidatePermutation {
        candidate_permutation: String,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum AlgorithmConfig {
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
        permutation_output_path_no_extension: PermutationPath,
    },
    ValidatePermutation {
        candidate_permutation: PermutationPath,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    pub algorithm: AlgorithmConfig,
    pub dispatcher: compute::Config,
}

impl TryFrom<UnverifiedConfig> for Config {
    type Error = Box<dyn Error>;

    fn try_from(value: UnverifiedConfig) -> Result<Self, Self::Error> {
        let (algorithm_config, image_dimensions) = match value {
            UnverifiedConfig::CreatePermutation {
                image_width,
                image_height,
                permutation_output_path_no_extension,
            } => (
                AlgorithmConfig::CreatePermutation {
                    permutation_output_path_no_extension: PermutationPath::from_output_path(
                        permutation_output_path_no_extension,
                    ),
                },
                ImageDimensions::new(image_width, image_height)?,
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
                permutation_output_path_no_extension,
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
                        permutation_output_path_no_extension: PermutationPath::from_output_path(
                            permutation_output_path_no_extension,
                        ),
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
