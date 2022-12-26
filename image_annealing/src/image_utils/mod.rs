use crate::compute::format::{self, VectorFieldImageBuffer, VectorFieldImageBufferComponent};

mod dimension;
pub mod displacement_goal;
mod manipulation;
pub mod validation;

pub(crate) use dimension::{
    check_dimensions_match2, check_dimensions_match3, check_dimensions_match4,
};
pub use dimension::{
    DimensionsMismatchError, ImageDimensions, ImageDimensionsHolder, InvalidDimensionError,
};

pub trait VectorField:
    AsRef<VectorFieldImageBuffer> + PartialEq<VectorFieldImageBuffer> + ImageDimensionsHolder
{
    fn identity(dimensions: &ImageDimensions) -> Self;

    fn into_inner(self) -> VectorFieldImageBuffer;

    fn as_raw_slice(&self) -> &[VectorFieldImageBufferComponent];

    fn is_identity(&self) -> bool {
        format::is_identity(self.as_ref())
    }
}

#[cfg(test)]
mod tests;
