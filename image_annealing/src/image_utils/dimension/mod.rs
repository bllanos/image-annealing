use image::GenericImageView;
use std::error::Error;
use std::fmt;
use std::num::NonZeroUsize;
use std::path::Path;

#[derive(Debug, Clone)]
pub enum InvalidDimensionError<T>
where
    T: TryInto<usize> + std::fmt::Debug + std::fmt::Display,
{
    InvalidNumberType(T),
    ZeroWidth,
    ZeroHeight,
    DepthNotOne,
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
            InvalidDimensionError::DepthNotOne => {
                write!(f, "depth is not {}", ImageDimensions::DEPTH)
            }
        }
    }
}

impl<T> Error for InvalidDimensionError<T> where
    T: TryInto<usize> + std::fmt::Debug + std::fmt::Display
{
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
    OutOfBoundLinear(ValueOutOfBounds),
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
            OutOfBoundsCoordinatesError::OutOfBoundLinear(detail) => {
                write!(f, "linear index {}", detail)
            }
        }
    }
}

impl<T> Error for OutOfBoundsCoordinatesError<T> where
    T: TryInto<usize> + std::fmt::Debug + std::fmt::Display
{
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct ImageDimensions {
    width: NonZeroUsize,
    height: NonZeroUsize,
}

impl ImageDimensions {
    const DEPTH: usize = 1;

    pub fn new(width: NonZeroUsize, height: NonZeroUsize) -> Self {
        Self { width, height }
    }

    pub fn try_new<T: TryInto<usize> + std::fmt::Debug + std::fmt::Display + Copy>(
        width: T,
        height: T,
    ) -> Result<Self, InvalidDimensionError<T>> {
        let width_usize = width
            .try_into()
            .map_err(|_| InvalidDimensionError::InvalidNumberType(width))?;
        let height_usize = height
            .try_into()
            .map_err(|_| InvalidDimensionError::InvalidNumberType(height))?;
        Ok(Self::new(
            NonZeroUsize::new(width_usize).ok_or(InvalidDimensionError::ZeroWidth)?,
            NonZeroUsize::new(height_usize).ok_or(InvalidDimensionError::ZeroHeight)?,
        ))
    }

    pub fn from_image<T>(image: &T) -> Result<Self, InvalidDimensionError<u32>>
    where
        T: GenericImageView,
    {
        let (width, height) = image.dimensions();
        Self::try_new(width, height)
    }

    pub fn from_image_path<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let (width, height) = image::image_dimensions(path)?;
        Ok(Self::try_new(width, height)?)
    }

    pub fn width(&self) -> usize {
        self.width.get()
    }

    pub fn height(&self) -> usize {
        self.height.get()
    }

    pub fn count(&self) -> usize {
        self.width().checked_mul(self.height.get()).unwrap()
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
        if x_usize >= self.width() {
            Err(OutOfBoundsCoordinatesError::OutOfBoundX(ValueOutOfBounds {
                value: x_usize,
                bounds: *self,
            }))
        } else if y_usize >= self.height() {
            Err(OutOfBoundsCoordinatesError::OutOfBoundY(ValueOutOfBounds {
                value: y_usize,
                bounds: *self,
            }))
        } else {
            Ok(y_usize
                .checked_mul(self.width())
                .unwrap()
                .checked_add(x_usize)
                .unwrap())
        }
    }

    pub fn make_coordinates<T>(
        &self,
        k: T,
    ) -> Result<(usize, usize), OutOfBoundsCoordinatesError<T>>
    where
        T: TryInto<usize> + std::fmt::Debug + std::fmt::Display + Copy,
    {
        let k_usize = k
            .try_into()
            .map_err(|_| OutOfBoundsCoordinatesError::InvalidNumberType(k))?;
        if k_usize >= self.count() {
            Err(OutOfBoundsCoordinatesError::OutOfBoundLinear(
                ValueOutOfBounds {
                    value: k_usize,
                    bounds: *self,
                },
            ))
        } else {
            Ok((k_usize % self.width, k_usize / self.width))
        }
    }

    pub fn to_extent(&self) -> wgpu::Extent3d {
        wgpu::Extent3d {
            width: self.width().try_into().unwrap(),
            height: self.height().try_into().unwrap(),
            depth_or_array_layers: Self::DEPTH.try_into().unwrap(),
        }
    }
}

impl TryFrom<wgpu::Extent3d> for ImageDimensions {
    type Error = InvalidDimensionError<u32>;

    fn try_from(value: wgpu::Extent3d) -> Result<Self, Self::Error> {
        if <u32 as TryInto<usize>>::try_into(value.depth_or_array_layers).unwrap() == Self::DEPTH {
            ImageDimensions::try_new(value.width, value.height)
        } else {
            Err(InvalidDimensionError::DepthNotOne)
        }
    }
}

impl PartialEq<wgpu::Extent3d> for ImageDimensions {
    fn eq(&self, other: &wgpu::Extent3d) -> bool {
        Self::try_from(*other).map_or(false, |ref dimensions| self == dimensions)
    }
}

impl fmt::Display for ImageDimensions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(width, height) = ({}, {})", self.width, self.height)
    }
}

pub trait ImageDimensionsHolder {
    fn dimensions(&self) -> &ImageDimensions;
}

#[derive(Debug, Clone)]
pub struct DimensionsMismatchError(ImageDimensions, ImageDimensions);

impl DimensionsMismatchError {
    pub fn new(first: ImageDimensions, second: ImageDimensions) -> Self {
        Self(first, second)
    }
}

impl fmt::Display for DimensionsMismatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "mismatch in image dimensions, {} and {}", self.0, self.1)
    }
}

impl Error for DimensionsMismatchError {}

pub(crate) fn check_dimensions_match2<'a, T: ImageDimensionsHolder, U: ImageDimensionsHolder>(
    holder1: &'a T,
    holder2: &'a U,
) -> Result<&'a ImageDimensions, DimensionsMismatchError> {
    let dimensions = holder1.dimensions();
    if dimensions == holder2.dimensions() {
        Ok(dimensions)
    } else {
        Err(DimensionsMismatchError::new(
            *dimensions,
            *holder2.dimensions(),
        ))
    }
}

pub(crate) fn check_dimensions_match3<
    'a,
    T: ImageDimensionsHolder,
    U: ImageDimensionsHolder,
    V: ImageDimensionsHolder,
>(
    holder1: &'a T,
    holder2: &'a U,
    holder3: &'a V,
) -> Result<&'a ImageDimensions, DimensionsMismatchError> {
    check_dimensions_match2(holder1, holder2)?;
    check_dimensions_match2(holder1, holder3)
}

pub(crate) fn check_dimensions_match4<
    'a,
    T: ImageDimensionsHolder,
    U: ImageDimensionsHolder,
    V: ImageDimensionsHolder,
    W: ImageDimensionsHolder,
>(
    holder1: &'a T,
    holder2: &'a U,
    holder3: &'a V,
    holder4: &'a W,
) -> Result<&'a ImageDimensions, DimensionsMismatchError> {
    check_dimensions_match3(holder1, holder2, holder3)?;
    check_dimensions_match2(holder1, holder4)
}

#[cfg(test)]
mod tests;
