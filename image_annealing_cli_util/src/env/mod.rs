use std::error::Error;
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::path::PathBuf;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnvironmentVariableNotUnicodeError {
    pub key: OsString,
    pub value: OsString,
}

impl EnvironmentVariableNotUnicodeError {
    pub fn new<T: AsRef<OsStr>>(key: T, value: OsString) -> Self {
        Self {
            key: OsString::from(key.as_ref()),
            value,
        }
    }
}

impl fmt::Display for EnvironmentVariableNotUnicodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "environment variable \"{}\" has a value that is invalid UTF-8, \"{}\"",
            self.key.to_string_lossy(),
            self.value.to_string_lossy()
        )
    }
}

impl Error for EnvironmentVariableNotUnicodeError {}

pub struct EnvironmentVariableData<'a> {
    pub key: &'a OsStr,
    pub value: Option<OsString>,
}

pub fn parse_environment_variable(
    data: EnvironmentVariableData,
) -> Result<Option<String>, EnvironmentVariableNotUnicodeError> {
    match data.value {
        Some(os_string) => match os_string.into_string() {
            Ok(string) => Ok(Some(string)),
            Err(os_string) => Err(EnvironmentVariableNotUnicodeError::new(data.key, os_string)),
        },
        None => Ok(None),
    }
}

pub fn parse_environment_variables<'a, const N: usize>(
    data: [EnvironmentVariableData<'a>; N],
) -> Result<[Option<String>; N], EnvironmentVariableNotUnicodeError> {
    // TODO [`array::try_map` is still experimental](https://github.com/rust-lang/rust/issues/79711),
    //      otherwise we would use it, and would not make `EnvironmentVariableNotUnicodeError` implement `Clone`.
    let parse_results = data.map(parse_environment_variable);
    parse_results
        .iter()
        .find(|&result| result.is_err())
        .cloned()
        .transpose()?;
    Ok(parse_results.map(Result::unwrap))
}

#[derive(Debug, Eq, PartialEq)]
pub struct InvalidBooleanEnvironmentVariableValueError(pub String);

impl fmt::Display for InvalidBooleanEnvironmentVariableValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Boolean environment variable value, \"{}\", is not a case-insensitive match with \"true\", \"1\", \"false\" or \"0\"",
            self.0,
        )
    }
}

impl Error for InvalidBooleanEnvironmentVariableValueError {}

pub fn parse_bool_from_environment_variable(
    value: &str,
) -> Result<bool, InvalidBooleanEnvironmentVariableValueError> {
    let normalized_value = value.trim();
    if normalized_value.eq_ignore_ascii_case("true") || normalized_value == "1" {
        Ok(true)
    } else if normalized_value.eq_ignore_ascii_case("false") || normalized_value == "0" {
        Ok(false)
    } else {
        Err(InvalidBooleanEnvironmentVariableValueError(
            normalized_value.to_string(),
        ))
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct FilepathEnvironmentVariableValue(pub PathBuf);

impl From<OsString> for FilepathEnvironmentVariableValue {
    fn from(value: OsString) -> Self {
        Self(PathBuf::from(value))
    }
}

#[cfg(test)]
mod tests;
