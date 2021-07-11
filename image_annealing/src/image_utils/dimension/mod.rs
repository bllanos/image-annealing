use image::GenericImageView;
use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct InvalidDimensionError;

impl fmt::Display for InvalidDimensionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid image dimensions supplied")
    }
}

impl Error for InvalidDimensionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct OutOfBoundsCoordinatesError;

impl fmt::Display for OutOfBoundsCoordinatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "coordinates are out of bounds")
    }
}

impl Error for OutOfBoundsCoordinatesError {
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
    pub fn new<T: TryInto<usize>>(width: T, height: T) -> Result<Self, InvalidDimensionError> {
        let width_usize = width.try_into().or(Err(InvalidDimensionError))?;
        let height_usize = height.try_into().or(Err(InvalidDimensionError))?;
        if width_usize == 0 || height_usize == 0 {
            Err(InvalidDimensionError)
        } else {
            Ok(ImageDimensions {
                width: width_usize,
                height: height_usize,
            })
        }
    }

    pub fn from_image<T>(image: &T) -> Result<Self, InvalidDimensionError>
    where
        T: GenericImageView,
    {
        let (width, height) = image.dimensions();
        Self::new(width as usize, height as usize)
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

    pub fn make_linear_index<T: TryInto<usize>>(
        &self,
        x: T,
        y: T,
    ) -> Result<usize, OutOfBoundsCoordinatesError> {
        let x_usize = x.try_into().or(Err(OutOfBoundsCoordinatesError))?;
        let y_usize = y.try_into().or(Err(OutOfBoundsCoordinatesError))?;
        if x_usize >= self.width || y_usize >= self.height {
            Err(OutOfBoundsCoordinatesError)
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
