use std::error::Error;
use std::fmt;
use std::path::{self, Path, PathBuf};

#[derive(Debug, Clone)]
pub enum FileError {
    NotAFile(PathBuf),
    NotFound(PathBuf),
}

impl fmt::Display for FileError {
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

impl Error for FileError {}

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

pub fn check_input_file_path<P: AsRef<Path>>(filepath: P) -> Result<(), FileError> {
    let filepath = filepath.as_ref();
    if filepath.exists() {
        if filepath.is_file() {
            Ok(())
        } else {
            Err(FileError::NotAFile(filepath.to_path_buf()))
        }
    } else {
        Err(FileError::NotFound(filepath.to_path_buf()))
    }
}

pub fn check_directory_path<P: AsRef<Path>>(directory: P) -> Result<(), DirectoryError> {
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

pub fn convert_path_separators<T: AsRef<str>>(filepath: T) -> String {
    let filepath = String::from(filepath.as_ref());
    if <String as AsRef<Path>>::as_ref(&filepath).is_absolute() {
        filepath
    } else {
        let new_separator = path::MAIN_SEPARATOR.to_string();
        filepath.replace(&['\\', '/'], &new_separator)
    }
}

pub fn convert_and_check_input_file_path<T: AsRef<str>>(filepath: T) -> Result<String, FileError> {
    let new_path = convert_path_separators(filepath);
    check_input_file_path(&new_path)?;
    Ok(new_path)
}

#[cfg(test)]
mod tests;
