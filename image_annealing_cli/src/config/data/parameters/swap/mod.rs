use super::super::number::{InvalidNonnegativeProperFractionError, NonnegativeProperFraction};
use serde::Deserialize;
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fmt;
use std::num::NonZeroUsize;

#[derive(Clone, Copy, Deserialize)]
pub enum UnverifiedSwapStopThreshold {
    SwapsAccepted(usize),
    SwapAcceptanceFraction(f64),
}

#[derive(Clone, Copy, Deserialize)]
pub struct UnverifiedIterationCount(pub usize);

#[derive(Clone, Deserialize)]
pub enum UnverifiedSwapStopConfig {
    Bounded {
        iteration_count: UnverifiedIterationCount,
        threshold: Option<UnverifiedSwapStopThreshold>,
    },
    Unbounded(UnverifiedSwapStopThreshold),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SwapStopThreshold {
    SwapsAccepted(usize),
    SwapAcceptanceFraction(NonnegativeProperFraction),
}

impl TryFrom<UnverifiedSwapStopThreshold> for SwapStopThreshold {
    type Error = InvalidNonnegativeProperFractionError;

    fn try_from(value: UnverifiedSwapStopThreshold) -> Result<Self, Self::Error> {
        match value {
            UnverifiedSwapStopThreshold::SwapsAccepted(count) => Ok(Self::SwapsAccepted(count)),
            UnverifiedSwapStopThreshold::SwapAcceptanceFraction(fraction) => {
                Ok(Self::SwapAcceptanceFraction(fraction.try_into()?))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct InvalidIterationCountError;

impl fmt::Display for InvalidIterationCountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "iteration count cannot be zero")
    }
}

impl Error for InvalidIterationCountError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IterationCount(pub NonZeroUsize);

impl IterationCount {
    pub fn get(self) -> usize {
        self.0.get()
    }
}

impl TryFrom<UnverifiedIterationCount> for IterationCount {
    type Error = InvalidIterationCountError;

    fn try_from(value: UnverifiedIterationCount) -> Result<Self, Self::Error> {
        Ok(Self(
            NonZeroUsize::new(value.0).ok_or(InvalidIterationCountError)?,
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SwapStopConfig {
    Bounded {
        iteration_count: IterationCount,
        threshold: Option<SwapStopThreshold>,
    },
    Unbounded(SwapStopThreshold),
}

impl TryFrom<UnverifiedSwapStopConfig> for SwapStopConfig {
    type Error = Box<dyn Error>;

    fn try_from(value: UnverifiedSwapStopConfig) -> Result<Self, Self::Error> {
        match value {
            UnverifiedSwapStopConfig::Bounded {
                iteration_count,
                threshold,
            } => Ok(Self::Bounded {
                iteration_count: iteration_count.try_into()?,
                threshold: match threshold {
                    Some(inner) => Some(inner.try_into()?),
                    None => None,
                },
            }),
            UnverifiedSwapStopConfig::Unbounded(threshold) => {
                Ok(Self::Unbounded(threshold.try_into()?))
            }
        }
    }
}

#[derive(Clone, Deserialize)]
pub struct UnverifiedSwapParametersConfig {
    pub stop: UnverifiedSwapStopConfig,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SwapParametersConfig {
    pub stop: SwapStopConfig,
}

impl TryFrom<UnverifiedSwapParametersConfig> for SwapParametersConfig {
    type Error = Box<dyn Error>;

    fn try_from(value: UnverifiedSwapParametersConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            stop: value.stop.try_into()?,
        })
    }
}

#[cfg(test)]
mod tests;
