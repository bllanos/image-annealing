use std::error::Error;
use std::ffi::{OsStr, OsString};
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnvironmentVariableNotFoundError {
    key: OsString,
}

impl EnvironmentVariableNotFoundError {
    pub fn new<T: AsRef<OsStr>>(key: T) -> Self {
        Self {
            key: OsString::from(key.as_ref()),
        }
    }

    pub fn key(&self) -> &OsStr {
        &self.key
    }
}

impl fmt::Display for EnvironmentVariableNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "environment variable \"{}\" not found",
            self.key.to_string_lossy()
        )
    }
}

impl Error for EnvironmentVariableNotFoundError {}

#[cfg(test)]
mod tests;
