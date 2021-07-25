use image::GenericImageView;
use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::path::Path;

#[derive(Debug, Clone)]
pub enum InvalidDimensionError<T>
where
    T: TryInto<usize> + std::fmt::Debug + std::fmt::Display,
{
    InvalidNumberType(T),
    ZeroWidth,
    ZeroHeight,
}

impl<T> fmt::Display for InvalidDimensionError<T>
where
    T: TryInto<usize> + std::fmt::Debug + std::fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InvalidDimensionError::InvalidNumberType(value) => {
                write!(
                    f,
                    "failed to convert {} to the required type for dimensions",
                    value
                )
            }
            InvalidDimensionError::ZeroWidth => write!(f, "width is zero"),
            InvalidDimensionError::ZeroHeight => write!(f, "height is zero"),
        }
    }
}

impl<T> Error for InvalidDimensionError<T>
where
    T: TryInto<usize> + std::fmt::Debug + std::fmt::Display,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct ValueOutOfBounds {
    value: usize,
    bounds: ImageDimensions,
}

impl fmt::Display for ValueOutOfBounds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} is out of bounds {}", self.value, self.bounds)
    }
}

#[derive(Debug, Clone)]
pub enum OutOfBoundsCoordinatesError<T>
where
    T: TryInto<usize> + std::fmt::Debug + std::fmt::Display,
{
    InvalidNumberType(T),
    OutOfBoundX(ValueOutOfBounds),
    OutOfBoundY(ValueOutOfBounds),
}

impl<T> fmt::Display for OutOfBoundsCoordinatesError<T>
where
    T: TryInto<usize> + std::fmt::Debug + std::fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OutOfBoundsCoordinatesError::InvalidNumberType(value) => {
                write!(
                    f,
                    "failed to convert {} to the required type for coordinates",
                    value
                )
            }
            OutOfBoundsCoordinatesError::OutOfBoundX(detail) => write!(f, "x = {}", detail),
            OutOfBoundsCoordinatesError::OutOfBoundY(detail) => {
                write!(f, "y = {}", detail)
            }
        }
    }
}

impl<T> Error for OutOfBoundsCoordinatesError<T>
where
    T: TryInto<usize> + std::fmt::Debug + std::fmt::Display,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct ImageDimensions {
    width: usize,
    height: usize,
}

impl ImageDimensions {
    pub fn new<T: TryInto<usize> + std::fmt::Debug + std::fmt::Display + Copy>(
        width: T,
        height: T,
    ) -> Result<Self, InvalidDimensionError<T>> {
        let width_usize = width
            .try_into()
            .map_err(|_| InvalidDimensionError::InvalidNumberType(width))?;
        let height_usize = height
            .try_into()
            .map_err(|_| InvalidDimensionError::InvalidNumberType(height))?;
        if width_usize == 0 {
            Err(InvalidDimensionError::ZeroWidth)
        } else if height_usize == 0 {
            Err(InvalidDimensionError::ZeroHeight)
        } else {
            Ok(ImageDimensions {
                width: width_usize,
                height: height_usize,
            })
        }
    }

    pub fn from_image<T>(image: &T) -> Result<Self, InvalidDimensionError<u32>>
    where
        T: GenericImageView,
    {
        let (width, height) = image.dimensions();
        Self::new(width, height)
    }

    pub fn from_image_path<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let (width, height) = image::image_dimensions(path)?;
        Ok(Self::new(width, height)?)
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

    pub fn make_linear_index<T>(&self, x: T, y: T) -> Result<usize, OutOfBoundsCoordinatesError<T>>
    where
        T: TryInto<usize> + std::fmt::Debug + std::fmt::Display + Copy,
    {
        let x_usize = x
            .try_into()
            .map_err(|_| OutOfBoundsCoordinatesError::InvalidNumberType(x))?;
        let y_usize = y
            .try_into()
            .map_err(|_| OutOfBoundsCoordinatesError::InvalidNumberType(y))?;
        if x_usize >= self.width {
            Err(OutOfBoundsCoordinatesError::OutOfBoundX(ValueOutOfBounds {
                value: x_usize,
                bounds: *self,
            }))
        } else if y_usize >= self.height {
            Err(OutOfBoundsCoordinatesError::OutOfBoundY(ValueOutOfBounds {
                value: y_usize,
                bounds: *self,
            }))
        } else {
            Ok(y_usize
                .checked_mul(self.width)
                .unwrap()
                .checked_add(x_usize)
                .unwrap())
        }
    }
}

impl fmt::Display for ImageDimensions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(width, height) = ({}, {})", self.width, self.height)
    }
}

#[cfg(test)]
mod tests;
