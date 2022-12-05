use image_annealing::ImageDimensions;
use image_annealing_cli_util::io;
use std::error::Error;
use std::fmt;
use std::path::Path;

pub trait ImagePath: AsRef<Path> + AsRef<str> + Clone + Eq + fmt::Display + PartialEq
where
    Self: Sized,
{
    fn from_raw<T: Into<String>>(path: T) -> Self;

    fn from_raw_clone<T: AsRef<str>>(path: T) -> Self {
        Self::from_raw(String::from(path.as_ref()))
    }

    fn from_input_path<T: AsRef<str>>(
        unverified_path: T,
    ) -> Result<(Self, ImageDimensions), Box<dyn Error>> {
        let path = io::convert_and_check_input_file_path(unverified_path)?;
        let dimensions = ImageDimensions::from_image_path(&path)?;
        Ok((Self::from_raw(path), dimensions))
    }

    fn from_output_path<T: AsRef<str>>(path_no_extension: T) -> Self {
        Self::from_raw(io::convert_path_separators(path_no_extension))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PermutationPath(String);

impl fmt::Display for PermutationPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl AsRef<Path> for PermutationPath {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl AsRef<str> for PermutationPath {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl ImagePath for PermutationPath {
    fn from_raw<T: Into<String>>(path: T) -> Self {
        Self(path.into())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DisplacementGoalPath(String);

impl fmt::Display for DisplacementGoalPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl AsRef<Path> for DisplacementGoalPath {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl AsRef<str> for DisplacementGoalPath {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl ImagePath for DisplacementGoalPath {
    fn from_raw<T: Into<String>>(path: T) -> Self {
        Self(path.into())
    }
}

#[cfg(test)]
mod tests;
