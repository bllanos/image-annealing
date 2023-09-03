use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct ConfigFileParsingError {
    filepath: PathBuf,
    error: serde_json::Error,
}

impl ConfigFileParsingError {
    pub fn new<P: AsRef<Path>>(filepath: P, error: serde_json::Error) -> Self {
        Self {
            filepath: filepath.as_ref().to_path_buf(),
            error,
        }
    }

    pub fn filepath(&self) -> &Path {
        &self.filepath
    }
}

impl fmt::Display for ConfigFileParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "error parsing the contents of '{}' as a configuration file: {}",
            self.filepath.display(),
            self.error
        )
    }
}

impl Error for ConfigFileParsingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}
