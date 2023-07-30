use super::{
    DirectoryError, FromWithPathContext, NotAFileError, PathError, PathNotFoundError,
    TryFromWithPathContext,
};
use relative_path::RelativePath;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub enum InputFileError {
    NotAFile(NotAFileError),
    NotFound(PathNotFoundError),
}

impl InputFileError {
    pub fn not_a_file(path: PathBuf) -> Self {
        Self::NotAFile(NotAFileError(path))
    }

    pub fn not_found(path: PathBuf) -> Self {
        Self::NotFound(PathNotFoundError(path))
    }
}

impl fmt::Display for InputFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotAFile(err) => write!(f, "input {}", err),
            Self::NotFound(err) => write!(f, "input file {}", err),
        }
    }
}

impl Error for InputFileError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(match self {
            Self::NotAFile(err) => err,
            Self::NotFound(err) => err,
        })
    }
}

pub fn check_input_file_path<P: AsRef<Path>>(path: P) -> Result<(), PathError<InputFileError>> {
    let path = path.as_ref();
    if path.try_exists()? {
        if path.is_file() {
            Ok(())
        } else {
            Err(PathError::Error(InputFileError::not_a_file(
                path.to_path_buf(),
            )))
        }
    } else {
        Err(PathError::Error(InputFileError::not_found(
            path.to_path_buf(),
        )))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct UnverifiedInputFilePath<'a>(pub Cow<'a, RelativePath>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputFilePath<'a>(pub Cow<'a, Path>);

impl<'a, P: AsRef<Path>> TryFromWithPathContext<UnverifiedInputFilePath<'a>, P>
    for InputFilePath<'static>
{
    type Error = PathError<InputFileError>;

    fn try_from_with_path_context(
        value: UnverifiedInputFilePath<'a>,
        base_path: P,
    ) -> Result<Self, Self::Error> {
        let full_path = PathBuf::from_with_path_context(&value.0, base_path);
        check_input_file_path(&full_path)?;
        Ok(Self(Cow::Owned(full_path)))
    }
}

impl<'a> fmt::Display for InputFilePath<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0.display(), f)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct UnverifiedInputDirectoryPath<'a>(pub Cow<'a, RelativePath>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputDirectoryPath<'a>(pub Cow<'a, Path>);

impl<'a, P: AsRef<Path>> TryFromWithPathContext<UnverifiedInputDirectoryPath<'a>, P>
    for InputDirectoryPath<'static>
{
    type Error = PathError<DirectoryError>;

    fn try_from_with_path_context(
        value: UnverifiedInputDirectoryPath<'a>,
        base_path: P,
    ) -> Result<Self, Self::Error> {
        let full_path = PathBuf::from_with_path_context(&value.0, base_path);
        super::check_input_directory_path(&full_path)?;
        Ok(Self(Cow::Owned(full_path)))
    }
}

impl<'a> fmt::Display for InputDirectoryPath<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0.display(), f)
    }
}

#[cfg(test)]
mod tests;
