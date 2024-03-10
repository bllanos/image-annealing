use std::error::Error;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum PowerPreference {
    None,
    LowPower,
    HighPerformance,
}

impl Default for PowerPreference {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct InvalidPowerPreferenceStringError(pub String);

impl fmt::Display for InvalidPowerPreferenceStringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "adapter power preference, \"{}\", is not a case-insensitive match with \"none\", \"low\" or \"high\"",
            self.0,
        )
    }
}

impl Error for InvalidPowerPreferenceStringError {}

impl TryFrom<&str> for PowerPreference {
    type Error = InvalidPowerPreferenceStringError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let normalized_value = value.trim();
        if normalized_value.eq_ignore_ascii_case("none") {
            Ok(Self::None)
        } else if normalized_value.eq_ignore_ascii_case("low") {
            Ok(Self::LowPower)
        } else if normalized_value.eq_ignore_ascii_case("high") {
            Ok(Self::HighPerformance)
        } else {
            Err(InvalidPowerPreferenceStringError(
                normalized_value.to_string(),
            ))
        }
    }
}

impl From<PowerPreference> for wgpu::PowerPreference {
    fn from(value: PowerPreference) -> Self {
        match value {
            PowerPreference::None => Self::None,
            PowerPreference::LowPower => Self::LowPower,
            PowerPreference::HighPerformance => Self::HighPerformance,
        }
    }
}

#[cfg(test)]
mod tests;
