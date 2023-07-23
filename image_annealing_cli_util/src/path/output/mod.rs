use super::{
    DirectoryError, FromWithPathContext, NotADirectoryError, NotAFileError, PathError,
    TryFromWithPathContext,
};
use relative_path::RelativePath;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ParentPathError(pub DirectoryError);

impl ParentPathError {
    pub fn not_a_directory(path: PathBuf) -> Self {
        Self(DirectoryError::not_a_directory(path))
    }

    pub fn not_found(path: PathBuf) -> Self {
        Self(DirectoryError::not_found(path))
    }
}

impl fmt::Display for ParentPathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "parent path of {}", self.0)
    }
}

impl Error for ParentPathError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}

#[derive(Debug, Clone)]
pub enum OutputFileError {
    NotAFile(NotAFileError),
    ParentPath(ParentPathError),
}

impl OutputFileError {
    pub fn not_a_file(path: PathBuf) -> Self {
        Self::NotAFile(NotAFileError(path))
    }

    pub fn parent_path(err: ParentPathError) -> Self {
        Self::ParentPath(err)
    }
}

impl fmt::Display for OutputFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotAFile(err) => write!(f, "output {}", err),
            Self::ParentPath(err) => write!(f, "output file path error, {}", err),
        }
    }
}

impl Error for OutputFileError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(match self {
            Self::NotAFile(err) => err,
            Self::ParentPath(err) => err,
        })
    }
}

#[derive(Debug, Clone)]
pub enum OutputDirectoryError {
    NotADirectory(NotADirectoryError),
    ParentPath(ParentPathError),
}

impl OutputDirectoryError {
    pub fn not_a_directory(path: PathBuf) -> Self {
        Self::NotADirectory(NotADirectoryError(path))
    }

    pub fn parent_path(err: ParentPathError) -> Self {
        Self::ParentPath(err)
    }
}

impl fmt::Display for OutputDirectoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotADirectory(err) => write!(f, "output {}", err),
            Self::ParentPath(err) => write!(f, "output directory path error, {}", err),
        }
    }
}

impl Error for OutputDirectoryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(match self {
            Self::NotADirectory(err) => err,
            Self::ParentPath(err) => err,
        })
    }
}

fn check_parent_path<P: AsRef<Path>>(path: P) -> Result<(), PathError<ParentPathError>> {
    let path = path.as_ref();
    let parent_path_option = path.parent();
    match parent_path_option {
        Some(parent_path) if parent_path != Path::new("") => {
            super::check_directory_path(parent_path).map_err(|err| match err {
                PathError::Error(inner_err) => {
                    let path_buf = path.to_path_buf();
                    PathError::Error(match inner_err {
                        DirectoryError::NotADirectory(_) => {
                            ParentPathError::not_a_directory(path_buf)
                        }
                        DirectoryError::NotFound(_) => ParentPathError::not_found(path_buf),
                    })
                }
                PathError::IOError(inner_err) => PathError::IOError(inner_err),
            })
        }
        _ => Ok(()),
    }
}

#[derive(Serialize, Deserialize)]
pub struct UnverifiedOutputFilePath<'a>(pub Cow<'a, RelativePath>);

pub struct OutputFilePath<'a>(pub Cow<'a, Path>);

impl<'a, P: AsRef<Path>> TryFromWithPathContext<UnverifiedOutputFilePath<'a>, P>
    for OutputFilePath<'static>
{
    type Error = PathError<OutputFileError>;

    fn try_from_with_path_context(
        value: UnverifiedOutputFilePath<'a>,
        base_path: P,
    ) -> Result<Self, Self::Error> {
        let full_path = PathBuf::from_with_path_context(&value.0, base_path);
        if full_path.try_exists()? {
            if full_path.is_file() {
                Ok(Self(Cow::Owned(full_path)))
            } else {
                Err(PathError::Error(OutputFileError::not_a_file(full_path)))
            }
        } else {
            check_parent_path(&full_path).map_err(|err| match err {
                PathError::Error(inner_err) => {
                    PathError::Error(OutputFileError::parent_path(inner_err))
                }
                PathError::IOError(inner_err) => PathError::IOError(inner_err),
            })?;
            Ok(Self(Cow::Owned(full_path)))
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UnverifiedOutputDirectoryPath<'a>(pub Cow<'a, RelativePath>);

pub struct OutputDirectoryPath<'a>(pub Cow<'a, Path>);

impl<'a, P: AsRef<Path>> TryFromWithPathContext<UnverifiedOutputDirectoryPath<'a>, P>
    for OutputDirectoryPath<'static>
{
    type Error = PathError<OutputDirectoryError>;

    fn try_from_with_path_context(
        value: UnverifiedOutputDirectoryPath<'a>,
        base_path: P,
    ) -> Result<Self, Self::Error> {
        let full_path = PathBuf::from_with_path_context(&value.0, base_path);
        match super::check_directory_path(&full_path) {
            Err(PathError::Error(DirectoryError::NotFound(_))) => {
                check_parent_path(&full_path).map_err(|err| match err {
                    PathError::Error(inner_err) => {
                        PathError::Error(OutputDirectoryError::parent_path(inner_err))
                    }
                    PathError::IOError(inner_err) => PathError::IOError(inner_err),
                })?;
                Ok(Self(Cow::Owned(full_path)))
            }
            Err(PathError::Error(DirectoryError::NotADirectory(inner_err))) => Err(
                PathError::Error(OutputDirectoryError::NotADirectory(inner_err)),
            ),
            Err(PathError::IOError(inner_err)) => Err(PathError::IOError(inner_err)),
            Ok(_) => Ok(Self(Cow::Owned(full_path))),
        }
    }
}

#[cfg(test)]
mod tests;