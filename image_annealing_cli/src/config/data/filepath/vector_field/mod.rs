use image_annealing::ImageDimensions;
use image_annealing_cli_util::path::{
    InputFilePath, OutputFileError, OutputFilePath, PathError, TryFromWithPathContext,
    TryIntoWithPathContext, UnverifiedInputFilePath, UnverifiedOutputFilePath,
};
use serde::Deserialize;
use std::borrow::Cow;
use std::error::Error;
use std::fmt;
use std::path::Path;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct UnverifiedInputPermutationPath<'a>(pub UnverifiedInputFilePath<'a>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InputPermutationPath<'a>(pub InputFilePath<'a>);

impl InputPermutationPath<'static> {
    pub fn try_from_unverified_with_path_context<P: AsRef<Path>>(
        unverified_path: UnverifiedInputPermutationPath,
        base_path: P,
    ) -> Result<(Self, ImageDimensions), Box<dyn Error>> {
        let path = InputFilePath::try_from_with_path_context(unverified_path.0, base_path)?;
        let dimensions = ImageDimensions::from_image_path(&path.0)?;
        Ok((Self(path), dimensions))
    }
}

impl<'a> fmt::Display for InputPermutationPath<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct UnverifiedOutputPermutationPath<'a>(pub UnverifiedOutputFilePath<'a>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutputPermutationPath<'a>(pub OutputFilePath<'a>);

impl TryFromWithPathContext<UnverifiedOutputPermutationPath<'_>>
    for OutputPermutationPath<'static>
{
    type Error = PathError<OutputFileError>;

    fn try_from_with_path_context<P: AsRef<Path>>(
        value: UnverifiedOutputPermutationPath<'_>,
        base_path: P,
    ) -> Result<Self, Self::Error> {
        Ok(Self(value.0.try_into_with_path_context(base_path)?))
    }
}

impl<'a> fmt::Display for OutputPermutationPath<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl OutputPermutationPath<'_> {
    // Ideally, this would be a method, but it may not be possible to return a `Self` type parameterized with a lifetime.
    // (See https://stackoverflow.com/questions/57701914/trait-method-which-returns-self-type-with-a-different-type-and-or-lifetime-par)
    pub fn to_owned(instance: &OutputPermutationPath) -> OutputPermutationPath<'static> {
        OutputPermutationPath(OutputFilePath(Cow::Owned(
            instance.0 .0.clone().into_owned(),
        )))
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct UnverifiedInputDisplacementGoalPath<'a>(pub UnverifiedInputFilePath<'a>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InputDisplacementGoalPath<'a>(pub InputFilePath<'a>);

impl InputDisplacementGoalPath<'static> {
    pub fn try_from_unverified_with_path_context<P: AsRef<Path>>(
        unverified_path: UnverifiedInputDisplacementGoalPath,
        base_path: P,
    ) -> Result<(Self, ImageDimensions), Box<dyn Error>> {
        let path = InputFilePath::try_from_with_path_context(unverified_path.0, base_path)?;
        let dimensions = ImageDimensions::from_image_path(&path.0)?;
        Ok((Self(path), dimensions))
    }
}

impl<'a> fmt::Display for InputDisplacementGoalPath<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct UnverifiedOutputDisplacementGoalPath<'a>(pub UnverifiedOutputFilePath<'a>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutputDisplacementGoalPath<'a>(pub OutputFilePath<'a>);

impl TryFromWithPathContext<UnverifiedOutputDisplacementGoalPath<'_>>
    for OutputDisplacementGoalPath<'static>
{
    type Error = PathError<OutputFileError>;

    fn try_from_with_path_context<P: AsRef<Path>>(
        value: UnverifiedOutputDisplacementGoalPath<'_>,
        base_path: P,
    ) -> Result<Self, Self::Error> {
        Ok(Self(value.0.try_into_with_path_context(base_path)?))
    }
}

impl<'a> fmt::Display for OutputDisplacementGoalPath<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[cfg(test)]
mod tests;
