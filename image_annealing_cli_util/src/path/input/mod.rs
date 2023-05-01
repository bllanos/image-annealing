use super::{DirectoryError, FromWithPathContext, TryFromWithPathContext};
use relative_path::RelativePath;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub enum InputFileError {
    NotAFile(PathBuf),
    NotFound(PathBuf),
}

impl fmt::Display for InputFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotAFile(path) => {
                write!(f, "path '{}' is not a file", path.display())
            }
            Self::NotFound(path) => {
                write!(f, "file '{}' does not exist", path.display())
            }
        }
    }
}

impl Error for InputFileError {}

#[derive(Serialize, Deserialize)]
pub struct UnverifiedInputFilePath<'a>(pub Cow<'a, RelativePath>);

pub struct InputFilePath<'a>(pub Cow<'a, Path>);

impl<'a, P: AsRef<Path>> TryFromWithPathContext<UnverifiedInputFilePath<'a>, P>
    for InputFilePath<'static>
{
    type Error = InputFileError;

    fn try_from_with_path_context(
        value: UnverifiedInputFilePath<'a>,
        base_path: P,
    ) -> Result<Self, Self::Error> {
        let full_path = PathBuf::from_with_path_context(&value.0, base_path);
        if full_path.exists() {
            if full_path.is_file() {
                Ok(Self(Cow::Owned(full_path)))
            } else {
                Err(InputFileError::NotAFile(full_path))
            }
        } else {
            Err(InputFileError::NotFound(full_path))
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UnverifiedInputDirectoryPath<'a>(Cow<'a, RelativePath>);

pub struct InputDirectoryPath<'a>(Cow<'a, Path>);

impl<'a, P: AsRef<Path>> TryFromWithPathContext<UnverifiedInputDirectoryPath<'a>, P>
    for InputDirectoryPath<'static>
{
    type Error = DirectoryError;

    fn try_from_with_path_context(
        value: UnverifiedInputDirectoryPath<'a>,
        base_path: P,
    ) -> Result<Self, Self::Error> {
        let full_path = PathBuf::from_with_path_context(&value.0, base_path);
        super::check_directory_path(&full_path)?;
        Ok(Self(Cow::Owned(full_path)))
    }
}
