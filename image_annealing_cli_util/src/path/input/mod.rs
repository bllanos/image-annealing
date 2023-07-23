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

#[derive(Serialize, Deserialize)]
pub struct UnverifiedInputFilePath<'a>(pub Cow<'a, RelativePath>);

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
        if full_path.try_exists()? {
            if full_path.is_file() {
                Ok(Self(Cow::Owned(full_path)))
            } else {
                Err(PathError::Error(InputFileError::not_a_file(full_path)))
            }
        } else {
            Err(PathError::Error(InputFileError::not_found(full_path)))
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UnverifiedInputDirectoryPath<'a>(pub Cow<'a, RelativePath>);

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
        super::check_directory_path(&full_path)?;
        Ok(Self(Cow::Owned(full_path)))
    }
}

#[cfg(test)]
mod tests;
