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

#[derive(Default)]
pub struct OutputConfig<'a> {
    create_permutation: Option<Cow<'a, Path>>,
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
    Ok(())
}

pub fn write_default_files<P: AsRef<Path>>(directory: Option<P>) -> Result<(), Box<dyn Error>> {
    Ok(write_files(&OutputConfig::with_base_directory(directory)?)?)
}
