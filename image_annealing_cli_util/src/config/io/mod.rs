use crate::path::TryFromWithPathContext;
use serde::de::DeserializeOwned;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
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

pub fn parse_config_file<T: ?Sized + 'static, U, P>(filename: P) -> Result<T, Box<dyn Error>>
where
    T: TryFromWithPathContext<U, Error = Box<dyn Error>>,
    U: DeserializeOwned,
    P: AsRef<Path>,
{
    crate::path::check_input_file_path(&filename)?;
    let file = File::open(&filename)?;
    let reader = BufReader::new(file);
    let unverified_config: U =
        serde_json::from_reader(reader).map_err(|e| ConfigFileParsingError::new(&filename, e))?;

    let config =
        T::try_from_with_path_context(unverified_config, filename.as_ref().parent().unwrap())?;
    Ok(config)
}

#[cfg(test)]
mod tests;
