use std::cmp::Eq;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct InvalidCornersError;

impl fmt::Display for InvalidCornersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "invalid specification of top left and bottom right image rectangle corners"
        )
    }
}

impl Error for InvalidCornersError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug, Eq)]
pub struct Rectangle {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

pub const N_CORNERS: usize = 4;

impl Rectangle {
    pub fn from_corners(x1: u32, y1: u32, x2: u32, y2: u32) -> Result<Self, InvalidCornersError> {
        if x1 <= x2 && y1 <= y2 {
            Ok(Rectangle {
                x: x1,
                y: y1,
                width: x2 - x1,
                height: y2 - y1,
            })
        } else {
            Err(InvalidCornersError)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }
}

impl PartialEq for Rectangle {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
            && self.y == other.y
            && self.width == other.width
            && self.height == other.height
    }
}

#[cfg(test)]
mod tests;
