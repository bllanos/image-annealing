use std::error::Error;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestError(pub String);

impl<T: Into<String>> From<T> for TestError {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Error for TestError {}

pub type TestResult = Result<(), TestError>;

pub fn test_result_equals_string<T: AsRef<str>>(result: TestResult, string: T) -> bool {
    match result {
        Ok(_) => false,
        Err(error) => error.0 == string.as_ref(),
    }
}

#[cfg(test)]
mod tests;
