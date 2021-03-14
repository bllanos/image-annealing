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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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

    pub fn from_dimensions(width_height: (u32, u32)) -> Self {
        Rectangle {
            x: 0,
            y: 0,
            width: width_height.0,
            height: width_height.1,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    pub fn encloses(&self, other: &Self) -> bool {
        self.x <= other.x
            && self.y <= other.y
            && self.x + self.width >= other.x + other.width
            && self.y + self.height >= other.y + other.height
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

#[cfg(test)]
mod tests;
