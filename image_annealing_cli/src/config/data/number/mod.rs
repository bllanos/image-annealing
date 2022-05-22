use std::convert::TryFrom;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum InvalidNonnegativeRationalNumberError {
    Negative(f64),
    NonFinite(f64),
}

impl fmt::Display for InvalidNonnegativeRationalNumberError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Negative(value) => {
                write!(f, "{} is negative", value)
            }
            Self::NonFinite(value) => {
                write!(f, "{} is not finite", value)
            }
        }
    }
}

impl Error for InvalidNonnegativeRationalNumberError {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NonnegativeRationalNumber(f64);

impl Eq for NonnegativeRationalNumber {}

impl NonnegativeRationalNumber {
    pub fn new(value: f64) -> Result<Self, InvalidNonnegativeRationalNumberError> {
        if value.is_finite() {
            if value >= 0.0 {
                Ok(Self(value))
            } else {
                Err(InvalidNonnegativeRationalNumberError::Negative(value))
            }
        } else {
            Err(InvalidNonnegativeRationalNumberError::NonFinite(value))
        }
    }

    pub fn get(self) -> f64 {
        self.0
    }
}

impl TryFrom<f64> for NonnegativeRationalNumber {
    type Error = InvalidNonnegativeRationalNumberError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[derive(Debug, Clone)]
pub enum InvalidNonnegativeProperFractionError {
    NotLessThanOne(f64),
    Irrational(InvalidNonnegativeRationalNumberError),
}

impl fmt::Display for InvalidNonnegativeProperFractionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotLessThanOne(value) => {
                write!(f, "{} is not less than one", value)
            }
            Self::Irrational(err) => err.fmt(f),
        }
    }
}

impl Error for InvalidNonnegativeProperFractionError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NonnegativeProperFraction(NonnegativeRationalNumber);

impl NonnegativeProperFraction {
    pub fn new(value: f64) -> Result<Self, InvalidNonnegativeProperFractionError> {
        if value < 1.0 {
            Ok(Self(NonnegativeRationalNumber::new(value).map_err(
                InvalidNonnegativeProperFractionError::Irrational,
            )?))
        } else {
            Err(InvalidNonnegativeProperFractionError::NotLessThanOne(value))
        }
    }

    pub fn get(self) -> f64 {
        self.0.get()
    }
}

impl TryFrom<f64> for NonnegativeProperFraction {
    type Error = InvalidNonnegativeProperFractionError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
