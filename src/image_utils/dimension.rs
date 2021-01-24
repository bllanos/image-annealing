use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct InvalidDimensionError;

impl fmt::Display for InvalidDimensionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid dimensions supplied")
    }
}

impl Error for InvalidDimensionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub struct ImageDimensions {
    width: usize,
    height: usize,
}

impl ImageDimensions {
    pub fn new(width: usize, height: usize) -> Result<Self, InvalidDimensionError> {
        if width == 0 || height == 0 {
            Err(InvalidDimensionError)
        } else {
            Ok(ImageDimensions { width, height })
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn count(&self) -> usize {
        self.width.checked_mul(self.height).unwrap()
    }
}
