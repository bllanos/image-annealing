use crate::path::{
    InputFileError, InputFilePath, PathError, TryFromWithPathContext, UnverifiedInputFilePath,
};
use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct UnverifiedInputTextFilePath<'a>(pub UnverifiedInputFilePath<'a>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InputTextFilePath<'a>(pub InputFilePath<'a>);

impl TryFromWithPathContext<UnverifiedInputTextFilePath<'_>> for InputTextFilePath<'static> {
    type Error = PathError<InputFileError>;

    fn try_from_with_path_context<P: AsRef<Path>>(
        value: UnverifiedInputTextFilePath,
        base_path: P,
    ) -> Result<Self, Self::Error> {
        Ok(Self(<InputFilePath as TryFromWithPathContext<
            UnverifiedInputFilePath,
        >>::try_from_with_path_context(
            value.0, base_path
        )?))
    }
}

impl<'a> fmt::Display for InputTextFilePath<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl TryFromWithPathContext<UnverifiedInputTextFilePath<'_>> for String {
    type Error = Box<dyn Error>;

    fn try_from_with_path_context<P: AsRef<Path>>(
        value: UnverifiedInputTextFilePath,
        base_path: P,
    ) -> Result<Self, Self::Error> {
        let path = <InputTextFilePath as TryFromWithPathContext<
        UnverifiedInputTextFilePath,
        >>::try_from_with_path_context(
            value, base_path
        )?;
        Ok(fs::read_to_string(path.0 .0)?)
    }
}

#[cfg(test)]
mod tests;
