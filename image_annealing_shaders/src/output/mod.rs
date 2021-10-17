use crate::shader;
use std::borrow::Cow;
use std::default::Default;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub enum OutputDirectoryError {
    NotADirectory(PathBuf),
    NotFound(PathBuf),
}

impl fmt::Display for OutputDirectoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OutputDirectoryError::NotADirectory(path) => {
                write!(f, "path '{}' is not a directory", path.display())
            }
            OutputDirectoryError::NotFound(path) => {
                write!(f, "path '{}' does not exist", path.display())
            }
        }
    }
}

impl Error for OutputDirectoryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct OutputConfig<'a> {
    pub create_permutation: Option<Cow<'a, Path>>,
    pub permute: Option<Cow<'a, Path>>,
    pub swap: Option<Cow<'a, Path>>,
}

impl<'a> OutputConfig<'a> {
    pub fn with_base_directory<P: AsRef<Path>>(
        directory: Option<P>,
    ) -> Result<Self, Box<dyn Error>> {
        let default_path = Path::new(".");
        let path = directory
            .as_ref()
            .map_or(default_path, <P as AsRef<Path>>::as_ref);
        if path.exists() {
            if path.is_dir() {
                Ok(Self {
                    create_permutation: Some(Cow::from(path.join("create_permutation.wgsl"))),
                    permute: Some(Cow::from(path.join("permute.wgsl"))),
                    swap: Some(Cow::from(path.join("swap.wgsl"))),
                })
            } else {
                Err(Box::new(OutputDirectoryError::NotADirectory(
                    path.to_path_buf(),
                )))
            }
        } else {
            Err(Box::new(OutputDirectoryError::NotFound(path.to_path_buf())))
        }
    }
}

pub fn write_files(config: &OutputConfig) -> std::io::Result<()> {
    if let Some(path) = config.create_permutation.as_ref() {
        let mut f = File::create(path)?;
        shader::create_permutation(&mut f)?;
    }
    if let Some(path) = config.permute.as_ref() {
        let mut f = File::create(path)?;
        shader::permute(&mut f)?;
    }
    if let Some(path) = config.swap.as_ref() {
        let mut f = File::create(path)?;
        shader::swap(&mut f)?;
    }
    Ok(())
}

pub fn write_default_files<'a, P: AsRef<Path>>(
    directory: Option<P>,
) -> Result<OutputConfig<'a>, Box<dyn Error>> {
    let config = OutputConfig::with_base_directory(directory)?;
    write_files(&config)?;
    Ok(config)
}

#[cfg(test)]
mod tests;
