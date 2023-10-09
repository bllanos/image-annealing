use relative_path::{RelativePath, RelativePathBuf};
use std::borrow::Cow;
use std::error::Error;
use std::ffi::OsStr;
use std::fmt;
use std::path::{Path, PathBuf};

mod input;
mod output;

pub use input::{
    check_input_file_path, InputDirectoryPath, InputFileError, InputFilePath,
    UnverifiedInputDirectoryPath, UnverifiedInputFilePath,
};

pub use output::{
    check_output_directory_path, check_output_file_path, OutputDirectoryError, OutputDirectoryPath,
    OutputFileError, OutputFilePath, ParentPathError, UnverifiedOutputDirectoryPath,
    UnverifiedOutputFilePath,
};

#[derive(Debug)]
pub enum PathError<T: Error + 'static> {
    Error(T),
    IOError(std::io::Error),
}

impl<T: Error> fmt::Display for PathError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Error(err) => <T as fmt::Display>::fmt(err, f),
            Self::IOError(err) => {
                write!(f, "io error {}", err)
            }
        }
    }
}

impl<T: Error + 'static> Error for PathError<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(match self {
            Self::Error(err) => err,
            Self::IOError(err) => err,
        })
    }
}

impl<T: Error + 'static> From<std::io::Error> for PathError<T> {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}

#[derive(Debug, Clone)]
pub struct PathNotFoundError(pub PathBuf);

impl fmt::Display for PathNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "path '{}' does not exist", self.0.display())
    }
}

impl Error for PathNotFoundError {}

#[derive(Debug, Clone)]
pub struct NotAFileError(PathBuf);

impl fmt::Display for NotAFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "path '{}' is not a file", self.0.display())
    }
}

impl Error for NotAFileError {}

#[derive(Debug, Clone)]
pub struct NotADirectoryError(PathBuf);

impl fmt::Display for NotADirectoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "path '{}' is not a directory", self.0.display())
    }
}

impl Error for NotADirectoryError {}

#[derive(Debug, Clone)]
pub enum DirectoryError {
    NotADirectory(NotADirectoryError),
    NotFound(PathNotFoundError),
}

impl DirectoryError {
    pub fn not_a_directory(path: PathBuf) -> Self {
        Self::NotADirectory(NotADirectoryError(path))
    }

    pub fn not_found(path: PathBuf) -> Self {
        Self::NotFound(PathNotFoundError(path))
    }
}

impl fmt::Display for DirectoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotADirectory(err) => err.fmt(f),
            Self::NotFound(err) => {
                write!(f, "directory {}", err)
            }
        }
    }
}

impl Error for DirectoryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(match self {
            Self::NotADirectory(err) => err,
            Self::NotFound(err) => err,
        })
    }
}

pub fn check_input_directory_path<P: AsRef<Path>>(
    directory: P,
) -> Result<(), PathError<DirectoryError>> {
    let directory = directory.as_ref();
    if directory.try_exists()? {
        if directory.is_dir() {
            Ok(())
        } else {
            Err(PathError::Error(DirectoryError::not_a_directory(
                directory.to_path_buf(),
            )))
        }
    } else {
        Err(PathError::Error(DirectoryError::not_found(
            directory.to_path_buf(),
        )))
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

pub fn make_base_path<P: AsRef<Path>, E, O>(
    candidate_path: &Path,
    make_context_path: O,
) -> Result<Cow<Path>, E>
where
    O: FnOnce() -> Result<P, E>,
{
    Ok(if candidate_path.is_absolute() {
        Cow::Borrowed(candidate_path)
    } else {
        Cow::Owned(make_context_path()?.as_ref().join(candidate_path))
    })
}

pub fn make_base_path_using_current_directory(
    candidate_path: &Path,
) -> Result<Cow<Path>, std::io::Error> {
    make_base_path(candidate_path, std::env::current_dir)
}

pub fn make_base_path_using_environment_variable<T: AsRef<OsStr>>(
    candidate_path: &Path,
    environment_variable_key: T,
) -> Result<Cow<Path>, std::env::VarError> {
    make_base_path(candidate_path, || std::env::var(environment_variable_key))
}

#[cfg(test)]
mod tests;
