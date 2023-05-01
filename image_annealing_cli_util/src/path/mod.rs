use relative_path::{RelativePath, RelativePathBuf};
use std::borrow::Cow;
use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};

mod input;

pub use input::{
    InputDirectoryPath, InputFileError, InputFilePath, UnverifiedInputDirectoryPath,
    UnverifiedInputFilePath,
};

#[derive(Debug, Clone)]
pub enum DirectoryError {
    NotADirectory(PathBuf),
    NotFound(PathBuf),
}

impl fmt::Display for DirectoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotADirectory(path) => {
                write!(f, "path '{}' is not a directory", path.display())
            }
            Self::NotFound(path) => {
                write!(f, "directory '{}' does not exist", path.display())
            }
        }
    }
}

impl Error for DirectoryError {}

fn check_directory_path<P: AsRef<Path>>(directory: P) -> Result<(), DirectoryError> {
    let directory = directory.as_ref();
    if directory.exists() {
        if directory.is_dir() {
            Ok(())
        } else {
            Err(DirectoryError::NotADirectory(directory.to_path_buf()))
        }
    } else {
        Err(DirectoryError::NotFound(directory.to_path_buf()))
    }
}

pub trait FromWithPathContext<T: ?Sized, P: AsRef<Path>>: Sized {
    fn from_with_path_context(value: T, base_path: P) -> Self;
}

pub trait IntoWithPathContext<T, P: AsRef<Path>>: Sized {
    fn into_with_path_context(self, base_path: P) -> T;
}

impl<T, U, P> IntoWithPathContext<U, P> for T
where
    P: AsRef<Path>,
    U: FromWithPathContext<T, P>,
{
    fn into_with_path_context(self, base_path: P) -> U {
        <U as FromWithPathContext<T, P>>::from_with_path_context(self, base_path)
    }
}

pub trait TryFromWithPathContext<T: ?Sized, P: AsRef<Path>>: Sized {
    type Error;

    fn try_from_with_path_context(value: T, base_path: P) -> Result<Self, Self::Error>;
}

pub trait TryIntoWithPathContext<T, P: AsRef<Path>>: Sized {
    type Error;

    fn try_into_with_path_context(self, base_path: P) -> Result<T, Self::Error>;
}

impl<T, U, P> TryIntoWithPathContext<U, P> for T
where
    P: AsRef<Path>,
    U: TryFromWithPathContext<T, P>,
{
    type Error = <U as TryFromWithPathContext<T, P>>::Error;

    fn try_into_with_path_context(self, base_path: P) -> Result<U, Self::Error> {
        <U as TryFromWithPathContext<T, P>>::try_from_with_path_context(self, base_path)
    }
}

impl<P: AsRef<Path>> FromWithPathContext<&RelativePath, P> for PathBuf {
    fn from_with_path_context(value: &RelativePath, base_path: P) -> Self {
        value.to_path(base_path)
    }
}

impl<T: AsRef<Path>, P: AsRef<Path>> TryFromWithPathContext<T, P> for RelativePathBuf {
    type Error = Box<dyn Error>;

    fn try_from_with_path_context(value: T, base_path: P) -> Result<Self, Self::Error> {
        Ok(Self::from_path(value.as_ref().strip_prefix(base_path)?)?)
    }
}

pub fn make_base_path<'a, P: AsRef<Path>>(base_path: P, candidate_path: &'a Path) -> Cow<'a, Path> {
    if candidate_path.is_absolute() {
        Cow::Borrowed(candidate_path)
    } else {
        Cow::Owned(base_path.as_ref().join(candidate_path))
    }
}

pub fn make_base_path_using_current_directory<'a>(
    candidate_path: &'a Path,
) -> std::io::Result<Cow<'a, Path>> {
    std::io::Result::Ok(make_base_path(std::env::current_dir()?, candidate_path))
}

#[cfg(test)]
mod tests;
