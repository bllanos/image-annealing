use std::error::Error;
use std::ffi::{OsStr, OsString};
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnvironmentVariableAccessError {
    key: OsString,
}

impl EnvironmentVariableAccessError {
    pub fn new<T: AsRef<OsStr>>(key: T) -> Self {
        Self {
            key: OsString::from(key.as_ref()),
        }
    }

    pub fn key(&self) -> &OsStr {
        &self.key
    }
}

impl fmt::Display for EnvironmentVariableAccessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "environment variable \"{}\" was not found or could not be accessed",
            self.key.to_string_lossy()
        )
    }
}

impl Error for EnvironmentVariableAccessError {}

pub fn var_os<T: AsRef<OsStr>>(
    environment_variable_key: T,
) -> Result<OsString, EnvironmentVariableAccessError> {
    std::env::var_os(&environment_variable_key)
        .ok_or_else(|| EnvironmentVariableAccessError::new(&environment_variable_key))
}

#[cfg(test)]
mod tests;
