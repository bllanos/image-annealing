use image_annealing::compute::format::ImageFormat;
use image_annealing::{DimensionsMismatchError, ImageDimensions};
use image_annealing_cli_util::io;
use serde::Deserialize;
use std::error::Error;
use std::path::Path;

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

impl UnverifiedLosslessImagePath {
    pub fn from_raw<T>(format: ImageFormat, paths: T) -> Self
    where
        T: IntoIterator,
        <T as IntoIterator>::Item: Into<String>,
    {
        let mut paths_iter = paths.into_iter();
        match format {
            ImageFormat::Rgba8 => Self::Rgba8(paths_iter.next().unwrap().into()),
            ImageFormat::Rgba8x2 => Self::Rgba8x2(
                paths_iter.next().unwrap().into(),
                paths_iter.next().unwrap().into(),
            ),
            ImageFormat::Rgba8x3 => Self::Rgba8x3(
                paths_iter.next().unwrap().into(),
                paths_iter.next().unwrap().into(),
                paths_iter.next().unwrap().into(),
            ),
            ImageFormat::Rgba8x4 => Self::Rgba8x4(
                paths_iter.next().unwrap().into(),
                paths_iter.next().unwrap().into(),
                paths_iter.next().unwrap().into(),
                paths_iter.next().unwrap().into(),
            ),
            ImageFormat::Rgba16 => Self::Rgba16(paths_iter.next().unwrap().into()),
            ImageFormat::Rgba16x2 => Self::Rgba16x2(
                paths_iter.next().unwrap().into(),
                paths_iter.next().unwrap().into(),
            ),
            ImageFormat::Rgba16Rgba8 => Self::Rgba16Rgba8(
                paths_iter.next().unwrap().into(),
                paths_iter.next().unwrap().into(),
            ),
            ImageFormat::Rgba16Rgba8x2 => Self::Rgba16Rgba8x2(
                paths_iter.next().unwrap().into(),
                paths_iter.next().unwrap().into(),
                paths_iter.next().unwrap().into(),
            ),
        }
    }

    pub fn from_raw_clone<T: AsRef<str>>(format: ImageFormat, paths: &[T]) -> Self {
        Self::from_raw(format, paths.iter().map(|path| String::from(path.as_ref())))
    }

    pub fn format(&self) -> ImageFormat {
        match self {
            Self::Rgba8(..) => ImageFormat::Rgba8,
            Self::Rgba8x2(..) => ImageFormat::Rgba8x2,
            Self::Rgba8x3(..) => ImageFormat::Rgba8x3,
            Self::Rgba8x4(..) => ImageFormat::Rgba8x4,
            Self::Rgba16(..) => ImageFormat::Rgba16,
            Self::Rgba16x2(..) => ImageFormat::Rgba16x2,
            Self::Rgba16Rgba8(..) => ImageFormat::Rgba16Rgba8,
            Self::Rgba16Rgba8x2(..) => ImageFormat::Rgba16Rgba8x2,
        }
    }
}

fn check_dimensions_match2<T: AsRef<Path>, U: AsRef<Path>>(
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

fn check_dimensions_match3<T: AsRef<Path>, U: AsRef<Path>, V: AsRef<Path>>(
    image_path1: &T,
    image_path2: &U,
    image_path3: &V,
) -> Result<ImageDimensions, Box<dyn Error>> {
    check_dimensions_match2(image_path1, image_path2)?;
    check_dimensions_match2(image_path1, image_path3)
}

fn check_dimensions_match4<T: AsRef<Path>, U: AsRef<Path>, V: AsRef<Path>, W: AsRef<Path>>(
    image_path1: &T,
    image_path2: &U,
    image_path3: &V,
    image_path4: &W,
) -> Result<ImageDimensions, Box<dyn Error>> {
    check_dimensions_match3(image_path1, image_path2, image_path3)?;
    check_dimensions_match2(image_path1, image_path4)
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
            Self::Rgba8(..) => ImageFormat::Rgba8,
            Self::Rgba8x2(..) => ImageFormat::Rgba8x2,
            Self::Rgba8x3(..) => ImageFormat::Rgba8x3,
            Self::Rgba8x4(..) => ImageFormat::Rgba8x4,
            Self::Rgba16(..) => ImageFormat::Rgba16,
            Self::Rgba16x2(..) => ImageFormat::Rgba16x2,
            Self::Rgba16Rgba8(..) => ImageFormat::Rgba16Rgba8,
            Self::Rgba16Rgba8x2(..) => ImageFormat::Rgba16Rgba8x2,
        }
    }

    pub fn to_vec(&self) -> Vec<&String> {
        match self {
            Self::Rgba8(path) => vec![path],
            Self::Rgba8x2(path1, path2) => vec![path1, path2],
            Self::Rgba8x3(path1, path2, path3) => vec![path1, path2, path3],
            Self::Rgba8x4(path1, path2, path3, path4) => {
                vec![path1, path2, path3, path4]
            }
            Self::Rgba16(path) => vec![path],
            Self::Rgba16x2(path1, path2) => vec![path1, path2],
            Self::Rgba16Rgba8(path1, path2) => vec![path1, path2],
            Self::Rgba16Rgba8x2(path1, path2, path3) => vec![path1, path2, path3],
        }
    }

    pub fn from_input_path(
        path: UnverifiedLosslessImagePath,
    ) -> Result<(Self, ImageDimensions), Box<dyn Error>> {
        Ok(match path {
            UnverifiedLosslessImagePath::Rgba8(unverified_path) => {
                let path = io::convert_and_check_input_file_path(unverified_path)?;
                let dimensions = ImageDimensions::from_image_path(&path)?;
                (Self::Rgba8(path), dimensions)
            }
            UnverifiedLosslessImagePath::Rgba8x2(unverified_path1, unverified_path2) => {
                let path1 = io::convert_and_check_input_file_path(unverified_path1)?;
                let path2 = io::convert_and_check_input_file_path(unverified_path2)?;
                let dimensions = check_dimensions_match2(&path1, &path2)?;
                (Self::Rgba8x2(path1, path2), dimensions)
            }
            UnverifiedLosslessImagePath::Rgba8x3(
                unverified_path1,
                unverified_path2,
                unverified_path3,
            ) => {
                let path1 = io::convert_and_check_input_file_path(unverified_path1)?;
                let path2 = io::convert_and_check_input_file_path(unverified_path2)?;
                let path3 = io::convert_and_check_input_file_path(unverified_path3)?;
                let dimensions = check_dimensions_match3(&path1, &path2, &path3)?;
                (Self::Rgba8x3(path1, path2, path3), dimensions)
            }
            UnverifiedLosslessImagePath::Rgba8x4(
                unverified_path1,
                unverified_path2,
                unverified_path3,
                unverified_path4,
            ) => {
                let path1 = io::convert_and_check_input_file_path(unverified_path1)?;
                let path2 = io::convert_and_check_input_file_path(unverified_path2)?;
                let path3 = io::convert_and_check_input_file_path(unverified_path3)?;
                let path4 = io::convert_and_check_input_file_path(unverified_path4)?;
                let dimensions = check_dimensions_match4(&path1, &path2, &path3, &path4)?;
                (Self::Rgba8x4(path1, path2, path3, path4), dimensions)
            }
            UnverifiedLosslessImagePath::Rgba16(unverified_path) => {
                let path = io::convert_and_check_input_file_path(unverified_path)?;
                let dimensions = ImageDimensions::from_image_path(&path)?;
                (Self::Rgba16(path), dimensions)
            }
            UnverifiedLosslessImagePath::Rgba16x2(unverified_path1, unverified_path2) => {
                let path1 = io::convert_and_check_input_file_path(unverified_path1)?;
                let path2 = io::convert_and_check_input_file_path(unverified_path2)?;
                let dimensions = check_dimensions_match2(&path1, &path2)?;
                (Self::Rgba16x2(path1, path2), dimensions)
            }
            UnverifiedLosslessImagePath::Rgba16Rgba8(unverified_path1, unverified_path2) => {
                let path1 = io::convert_and_check_input_file_path(unverified_path1)?;
                let path2 = io::convert_and_check_input_file_path(unverified_path2)?;
                let dimensions = check_dimensions_match2(&path1, &path2)?;
                (Self::Rgba16Rgba8(path1, path2), dimensions)
            }
            UnverifiedLosslessImagePath::Rgba16Rgba8x2(
                unverified_path1,
                unverified_path2,
                unverified_path3,
            ) => {
                let path1 = io::convert_and_check_input_file_path(unverified_path1)?;
                let path2 = io::convert_and_check_input_file_path(unverified_path2)?;
                let path3 = io::convert_and_check_input_file_path(unverified_path3)?;
                let dimensions = check_dimensions_match3(&path1, &path2, &path3)?;
                (Self::Rgba16Rgba8x2(path1, path2, path3), dimensions)
            }
        })
    }

    pub fn from_output_path(path_no_extension: UnverifiedLosslessImagePath) -> Self {
        match path_no_extension {
            UnverifiedLosslessImagePath::Rgba8(unverified_path) => {
                Self::Rgba8(io::convert_path_separators(unverified_path))
            }
            UnverifiedLosslessImagePath::Rgba8x2(unverified_path1, unverified_path2) => {
                Self::Rgba8x2(
                    io::convert_path_separators(unverified_path1),
                    io::convert_path_separators(unverified_path2),
                )
            }
            UnverifiedLosslessImagePath::Rgba8x3(
                unverified_path1,
                unverified_path2,
                unverified_path3,
            ) => Self::Rgba8x3(
                io::convert_path_separators(unverified_path1),
                io::convert_path_separators(unverified_path2),
                io::convert_path_separators(unverified_path3),
            ),
            UnverifiedLosslessImagePath::Rgba8x4(
                unverified_path1,
                unverified_path2,
                unverified_path3,
                unverified_path4,
            ) => Self::Rgba8x4(
                io::convert_path_separators(unverified_path1),
                io::convert_path_separators(unverified_path2),
                io::convert_path_separators(unverified_path3),
                io::convert_path_separators(unverified_path4),
            ),
            UnverifiedLosslessImagePath::Rgba16(unverified_path) => {
                Self::Rgba16(io::convert_path_separators(unverified_path))
            }
            UnverifiedLosslessImagePath::Rgba16x2(unverified_path1, unverified_path2) => {
                Self::Rgba16x2(
                    io::convert_path_separators(unverified_path1),
                    io::convert_path_separators(unverified_path2),
                )
            }
            UnverifiedLosslessImagePath::Rgba16Rgba8(unverified_path1, unverified_path2) => {
                Self::Rgba16Rgba8(
                    io::convert_path_separators(unverified_path1),
                    io::convert_path_separators(unverified_path2),
                )
            }
            UnverifiedLosslessImagePath::Rgba16Rgba8x2(
                unverified_path1,
                unverified_path2,
                unverified_path3,
            ) => Self::Rgba16Rgba8x2(
                io::convert_path_separators(unverified_path1),
                io::convert_path_separators(unverified_path2),
                io::convert_path_separators(unverified_path3),
            ),
        }
    }
}

#[cfg(test)]
mod tests;
