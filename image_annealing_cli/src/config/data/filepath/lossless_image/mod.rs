use image_annealing::compute::format::ImageFormat;
use image_annealing::{DimensionsMismatchError, ImageDimensions};
use image_annealing_cli_util::path::{
    InputFilePath, OutputFileError, OutputFilePath, PathError, TryFromWithPathContext,
    UnverifiedInputFilePath, UnverifiedOutputFilePath,
};
use serde::Deserialize;
use std::error::Error;
use std::path::Path;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum UnverifiedInputLosslessImagePath<'a> {
    Rgba8(UnverifiedInputFilePath<'a>),
    Rgba8x2(UnverifiedInputFilePath<'a>, UnverifiedInputFilePath<'a>),
    Rgba8x3(
        UnverifiedInputFilePath<'a>,
        UnverifiedInputFilePath<'a>,
        UnverifiedInputFilePath<'a>,
    ),
    Rgba8x4(
        UnverifiedInputFilePath<'a>,
        UnverifiedInputFilePath<'a>,
        UnverifiedInputFilePath<'a>,
        UnverifiedInputFilePath<'a>,
    ),
    Rgba16(UnverifiedInputFilePath<'a>),
    Rgba16x2(UnverifiedInputFilePath<'a>, UnverifiedInputFilePath<'a>),
    Rgba16Rgba8(UnverifiedInputFilePath<'a>, UnverifiedInputFilePath<'a>),
    Rgba16Rgba8x2(
        UnverifiedInputFilePath<'a>,
        UnverifiedInputFilePath<'a>,
        UnverifiedInputFilePath<'a>,
    ),
}

impl<'a> UnverifiedInputLosslessImagePath<'a> {
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

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum UnverifiedOutputLosslessImagePath<'a> {
    Rgba8(UnverifiedOutputFilePath<'a>),
    Rgba8x2(UnverifiedOutputFilePath<'a>, UnverifiedOutputFilePath<'a>),
    Rgba8x3(
        UnverifiedOutputFilePath<'a>,
        UnverifiedOutputFilePath<'a>,
        UnverifiedOutputFilePath<'a>,
    ),
    Rgba8x4(
        UnverifiedOutputFilePath<'a>,
        UnverifiedOutputFilePath<'a>,
        UnverifiedOutputFilePath<'a>,
        UnverifiedOutputFilePath<'a>,
    ),
    Rgba16(UnverifiedOutputFilePath<'a>),
    Rgba16x2(UnverifiedOutputFilePath<'a>, UnverifiedOutputFilePath<'a>),
    Rgba16Rgba8(UnverifiedOutputFilePath<'a>, UnverifiedOutputFilePath<'a>),
    Rgba16Rgba8x2(
        UnverifiedOutputFilePath<'a>,
        UnverifiedOutputFilePath<'a>,
        UnverifiedOutputFilePath<'a>,
    ),
}

impl<'a> UnverifiedOutputLosslessImagePath<'a> {
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

fn check_dimensions_match2<'a>(
    image_path1: &InputFilePath<'a>,
    image_path2: &InputFilePath<'a>,
) -> Result<ImageDimensions, Box<dyn Error>> {
    let dimensions1 = ImageDimensions::from_image_path(&image_path1.0)?;
    let dimensions2 = ImageDimensions::from_image_path(&image_path2.0)?;
    if dimensions1 == dimensions2 {
        Ok(dimensions1)
    } else {
        Err(Box::new(DimensionsMismatchError::new(
            dimensions1,
            dimensions2,
        )))
    }
}

fn check_dimensions_match3<'a>(
    image_path1: &InputFilePath<'a>,
    image_path2: &InputFilePath<'a>,
    image_path3: &InputFilePath<'a>,
) -> Result<ImageDimensions, Box<dyn Error>> {
    check_dimensions_match2(image_path1, image_path2)?;
    check_dimensions_match2(image_path1, image_path3)
}

fn check_dimensions_match4<'a>(
    image_path1: &InputFilePath<'a>,
    image_path2: &InputFilePath<'a>,
    image_path3: &InputFilePath<'a>,
    image_path4: &InputFilePath<'a>,
) -> Result<ImageDimensions, Box<dyn Error>> {
    check_dimensions_match3(image_path1, image_path2, image_path3)?;
    check_dimensions_match2(image_path1, image_path4)
}

#[derive(Debug, PartialEq, Eq)]
pub enum InputLosslessImagePath<'a> {
    Rgba8(InputFilePath<'a>),
    Rgba8x2(InputFilePath<'a>, InputFilePath<'a>),
    Rgba8x3(InputFilePath<'a>, InputFilePath<'a>, InputFilePath<'a>),
    Rgba8x4(
        InputFilePath<'a>,
        InputFilePath<'a>,
        InputFilePath<'a>,
        InputFilePath<'a>,
    ),
    Rgba16(InputFilePath<'a>),
    Rgba16x2(InputFilePath<'a>, InputFilePath<'a>),
    Rgba16Rgba8(InputFilePath<'a>, InputFilePath<'a>),
    Rgba16Rgba8x2(InputFilePath<'a>, InputFilePath<'a>, InputFilePath<'a>),
}

impl<'a> InputLosslessImagePath<'a> {
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

    pub fn as_vec(&self) -> Vec<&Path> {
        match self {
            Self::Rgba8(path) => vec![path.0.as_ref()],
            Self::Rgba8x2(path1, path2) => vec![path1.0.as_ref(), path2.0.as_ref()],
            Self::Rgba8x3(path1, path2, path3) => {
                vec![path1.0.as_ref(), path2.0.as_ref(), path3.0.as_ref()]
            }
            Self::Rgba8x4(path1, path2, path3, path4) => {
                vec![
                    path1.0.as_ref(),
                    path2.0.as_ref(),
                    path3.0.as_ref(),
                    path4.0.as_ref(),
                ]
            }
            Self::Rgba16(path) => vec![path.0.as_ref()],
            Self::Rgba16x2(path1, path2) => vec![path1.0.as_ref(), path2.0.as_ref()],
            Self::Rgba16Rgba8(path1, path2) => vec![path1.0.as_ref(), path2.0.as_ref()],
            Self::Rgba16Rgba8x2(path1, path2, path3) => {
                vec![path1.0.as_ref(), path2.0.as_ref(), path3.0.as_ref()]
            }
        }
    }
}

impl<'a> InputLosslessImagePath<'static> {
    pub fn try_from_unverified_with_path_context<P: AsRef<Path>>(
        path: UnverifiedInputLosslessImagePath<'a>,
        base_path: P,
    ) -> Result<(Self, ImageDimensions), Box<dyn Error>> {
        Ok(match path {
            UnverifiedInputLosslessImagePath::Rgba8(unverified_path) => {
                let path = InputFilePath::try_from_with_path_context(unverified_path, &base_path)?;
                let dimensions = ImageDimensions::from_image_path(&path.0)?;
                (Self::Rgba8(path), dimensions)
            }
            UnverifiedInputLosslessImagePath::Rgba8x2(unverified_path1, unverified_path2) => {
                let path1 =
                    InputFilePath::try_from_with_path_context(unverified_path1, &base_path)?;
                let path2 =
                    InputFilePath::try_from_with_path_context(unverified_path2, &base_path)?;
                let dimensions = check_dimensions_match2(&path1, &path2)?;
                (Self::Rgba8x2(path1, path2), dimensions)
            }
            UnverifiedInputLosslessImagePath::Rgba8x3(
                unverified_path1,
                unverified_path2,
                unverified_path3,
            ) => {
                let path1 =
                    InputFilePath::try_from_with_path_context(unverified_path1, &base_path)?;
                let path2 =
                    InputFilePath::try_from_with_path_context(unverified_path2, &base_path)?;
                let path3 =
                    InputFilePath::try_from_with_path_context(unverified_path3, &base_path)?;
                let dimensions = check_dimensions_match3(&path1, &path2, &path3)?;
                (Self::Rgba8x3(path1, path2, path3), dimensions)
            }
            UnverifiedInputLosslessImagePath::Rgba8x4(
                unverified_path1,
                unverified_path2,
                unverified_path3,
                unverified_path4,
            ) => {
                let path1 =
                    InputFilePath::try_from_with_path_context(unverified_path1, &base_path)?;
                let path2 =
                    InputFilePath::try_from_with_path_context(unverified_path2, &base_path)?;
                let path3 =
                    InputFilePath::try_from_with_path_context(unverified_path3, &base_path)?;
                let path4 =
                    InputFilePath::try_from_with_path_context(unverified_path4, &base_path)?;
                let dimensions = check_dimensions_match4(&path1, &path2, &path3, &path4)?;
                (Self::Rgba8x4(path1, path2, path3, path4), dimensions)
            }
            UnverifiedInputLosslessImagePath::Rgba16(unverified_path) => {
                let path = InputFilePath::try_from_with_path_context(unverified_path, &base_path)?;
                let dimensions = ImageDimensions::from_image_path(&path.0)?;
                (Self::Rgba16(path), dimensions)
            }
            UnverifiedInputLosslessImagePath::Rgba16x2(unverified_path1, unverified_path2) => {
                let path1 =
                    InputFilePath::try_from_with_path_context(unverified_path1, &base_path)?;
                let path2 =
                    InputFilePath::try_from_with_path_context(unverified_path2, &base_path)?;
                let dimensions = check_dimensions_match2(&path1, &path2)?;
                (Self::Rgba16x2(path1, path2), dimensions)
            }
            UnverifiedInputLosslessImagePath::Rgba16Rgba8(unverified_path1, unverified_path2) => {
                let path1 =
                    InputFilePath::try_from_with_path_context(unverified_path1, &base_path)?;
                let path2 =
                    InputFilePath::try_from_with_path_context(unverified_path2, &base_path)?;
                let dimensions = check_dimensions_match2(&path1, &path2)?;
                (Self::Rgba16Rgba8(path1, path2), dimensions)
            }
            UnverifiedInputLosslessImagePath::Rgba16Rgba8x2(
                unverified_path1,
                unverified_path2,
                unverified_path3,
            ) => {
                let path1 =
                    InputFilePath::try_from_with_path_context(unverified_path1, &base_path)?;
                let path2 =
                    InputFilePath::try_from_with_path_context(unverified_path2, &base_path)?;
                let path3 =
                    InputFilePath::try_from_with_path_context(unverified_path3, &base_path)?;
                let dimensions = check_dimensions_match3(&path1, &path2, &path3)?;
                (Self::Rgba16Rgba8x2(path1, path2, path3), dimensions)
            }
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum OutputLosslessImagePath<'a> {
    Rgba8(OutputFilePath<'a>),
    Rgba8x2(OutputFilePath<'a>, OutputFilePath<'a>),
    Rgba8x3(OutputFilePath<'a>, OutputFilePath<'a>, OutputFilePath<'a>),
    Rgba8x4(
        OutputFilePath<'a>,
        OutputFilePath<'a>,
        OutputFilePath<'a>,
        OutputFilePath<'a>,
    ),
    Rgba16(OutputFilePath<'a>),
    Rgba16x2(OutputFilePath<'a>, OutputFilePath<'a>),
    Rgba16Rgba8(OutputFilePath<'a>, OutputFilePath<'a>),
    Rgba16Rgba8x2(OutputFilePath<'a>, OutputFilePath<'a>, OutputFilePath<'a>),
}

impl<'a> OutputLosslessImagePath<'a> {
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

    pub fn as_vec(&self) -> Vec<&Path> {
        match self {
            Self::Rgba8(path) => vec![path.0.as_ref()],
            Self::Rgba8x2(path1, path2) => vec![path1.0.as_ref(), path2.0.as_ref()],
            Self::Rgba8x3(path1, path2, path3) => {
                vec![path1.0.as_ref(), path2.0.as_ref(), path3.0.as_ref()]
            }
            Self::Rgba8x4(path1, path2, path3, path4) => {
                vec![
                    path1.0.as_ref(),
                    path2.0.as_ref(),
                    path3.0.as_ref(),
                    path4.0.as_ref(),
                ]
            }
            Self::Rgba16(path) => vec![path.0.as_ref()],
            Self::Rgba16x2(path1, path2) => vec![path1.0.as_ref(), path2.0.as_ref()],
            Self::Rgba16Rgba8(path1, path2) => vec![path1.0.as_ref(), path2.0.as_ref()],
            Self::Rgba16Rgba8x2(path1, path2, path3) => {
                vec![path1.0.as_ref(), path2.0.as_ref(), path3.0.as_ref()]
            }
        }
    }
}

impl<'a, P: AsRef<Path>> TryFromWithPathContext<UnverifiedOutputLosslessImagePath<'a>, P>
    for OutputLosslessImagePath<'static>
{
    type Error = PathError<OutputFileError>;

    fn try_from_with_path_context(
        value: UnverifiedOutputLosslessImagePath<'a>,
        base_path: P,
    ) -> Result<Self, Self::Error> {
        Ok(match value {
            UnverifiedOutputLosslessImagePath::Rgba8(unverified_path) => {
                let path = OutputFilePath::try_from_with_path_context(unverified_path, &base_path)?;
                Self::Rgba8(path)
            }
            UnverifiedOutputLosslessImagePath::Rgba8x2(unverified_path1, unverified_path2) => {
                let path1 =
                    OutputFilePath::try_from_with_path_context(unverified_path1, &base_path)?;
                let path2 =
                    OutputFilePath::try_from_with_path_context(unverified_path2, &base_path)?;
                Self::Rgba8x2(path1, path2)
            }
            UnverifiedOutputLosslessImagePath::Rgba8x3(
                unverified_path1,
                unverified_path2,
                unverified_path3,
            ) => {
                let path1 =
                    OutputFilePath::try_from_with_path_context(unverified_path1, &base_path)?;
                let path2 =
                    OutputFilePath::try_from_with_path_context(unverified_path2, &base_path)?;
                let path3 =
                    OutputFilePath::try_from_with_path_context(unverified_path3, &base_path)?;
                Self::Rgba8x3(path1, path2, path3)
            }
            UnverifiedOutputLosslessImagePath::Rgba8x4(
                unverified_path1,
                unverified_path2,
                unverified_path3,
                unverified_path4,
            ) => {
                let path1 =
                    OutputFilePath::try_from_with_path_context(unverified_path1, &base_path)?;
                let path2 =
                    OutputFilePath::try_from_with_path_context(unverified_path2, &base_path)?;
                let path3 =
                    OutputFilePath::try_from_with_path_context(unverified_path3, &base_path)?;
                let path4 =
                    OutputFilePath::try_from_with_path_context(unverified_path4, &base_path)?;
                Self::Rgba8x4(path1, path2, path3, path4)
            }
            UnverifiedOutputLosslessImagePath::Rgba16(unverified_path) => {
                let path = OutputFilePath::try_from_with_path_context(unverified_path, &base_path)?;
                Self::Rgba16(path)
            }
            UnverifiedOutputLosslessImagePath::Rgba16x2(unverified_path1, unverified_path2) => {
                let path1 =
                    OutputFilePath::try_from_with_path_context(unverified_path1, &base_path)?;
                let path2 =
                    OutputFilePath::try_from_with_path_context(unverified_path2, &base_path)?;
                Self::Rgba16x2(path1, path2)
            }
            UnverifiedOutputLosslessImagePath::Rgba16Rgba8(unverified_path1, unverified_path2) => {
                let path1 =
                    OutputFilePath::try_from_with_path_context(unverified_path1, &base_path)?;
                let path2 =
                    OutputFilePath::try_from_with_path_context(unverified_path2, &base_path)?;
                Self::Rgba16Rgba8(path1, path2)
            }
            UnverifiedOutputLosslessImagePath::Rgba16Rgba8x2(
                unverified_path1,
                unverified_path2,
                unverified_path3,
            ) => {
                let path1 =
                    OutputFilePath::try_from_with_path_context(unverified_path1, &base_path)?;
                let path2 =
                    OutputFilePath::try_from_with_path_context(unverified_path2, &base_path)?;
                let path3 =
                    OutputFilePath::try_from_with_path_context(unverified_path3, &base_path)?;
                Self::Rgba16Rgba8x2(path1, path2, path3)
            }
        })
    }
}

#[cfg(test)]
mod tests;
