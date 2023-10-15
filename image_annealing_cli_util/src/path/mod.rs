use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};

mod conversion;
mod input;
mod output;

pub use conversion::{
    make_base_path, make_base_path_using_current_directory,
    make_base_path_using_environment_variable, FromWithPathContext, IntoWithPathContext,
    TryFromWithPathContext, TryIntoWithPathContext,
};

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

#[cfg(test)]
mod tests;
