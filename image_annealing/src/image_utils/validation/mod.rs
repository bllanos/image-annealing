use super::manipulation;
use super::{ImageDimensions, ImageDimensionsHolder};
use crate::compute::conversion::VectorFieldEntry;
use crate::compute::format::{
    ImageFileWriter, ImageFileWriterSaveResult, Rgba8Image, VectorFieldImageBuffer,
    VectorFieldImageBufferComponent,
};
use std::error::Error;
use std::fmt;
use std::ops::IndexMut;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub struct PermutationPixelData {
    x: usize,
    y: usize,
    value: VectorFieldEntry,
}

impl fmt::Display for PermutationPixelData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(x, y, delta_x, delta_y) = ({}, {}, {}, {})",
            self.x, self.y, self.value.0, self.value.1
        )
    }
}

#[derive(Debug, Clone)]
pub enum PermutationFlaw {
    OutOfBounds {
        dimensions: ImageDimensions,
        data: PermutationPixelData,
    },
    Duplicate {
        x: usize,
        y: usize,
        first_source: PermutationPixelData,
        second_source: PermutationPixelData,
    },
}

impl fmt::Display for PermutationFlaw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PermutationFlaw::OutOfBounds { dimensions, data } => write!(
                f,
                "out of bounds mapping {} for an image of dimensions {}",
                data, dimensions
            ),
            PermutationFlaw::Duplicate {
                x,
                y,
                first_source,
                second_source,
            } => write!(
                f,
                "entries {} and {} both map to location (x, y) = ({}, {})",
                first_source, second_source, x, y
            ),
        }
    }
}

impl Error for PermutationFlaw {}

pub type CandidatePermutation = Rgba8Image;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedPermutation {
    data: VectorFieldImageBuffer,
    dimensions: ImageDimensions,
}

impl ValidatedPermutation {
    pub fn as_raw_slice(&self) -> &[VectorFieldImageBufferComponent] {
        self.data.as_raw().as_slice()
    }

    pub fn into_inner(self) -> VectorFieldImageBuffer {
        self.data
    }

    pub fn inverse(&self) -> Self {
        Self {
            data: manipulation::invert_permutation(self),
            dimensions: self.dimensions,
        }
    }
}

impl AsRef<VectorFieldImageBuffer> for ValidatedPermutation {
    fn as_ref(&self) -> &VectorFieldImageBuffer {
        &self.data
    }
}

impl ImageDimensionsHolder for ValidatedPermutation {
    fn dimensions(&self) -> &ImageDimensions {
        &self.dimensions
    }
}

impl ImageFileWriter for ValidatedPermutation {
    const EXTENSION: &'static str = <VectorFieldImageBuffer as ImageFileWriter>::EXTENSION;

    fn save_add_extension<P: AsRef<Path>>(
        &self,
        path_no_extension: P,
    ) -> ImageFileWriterSaveResult {
        self.data.save_add_extension(path_no_extension)
    }
}

pub(crate) fn validate_permutation(
    image: VectorFieldImageBuffer,
) -> Result<ValidatedPermutation, Box<dyn Error>> {
    let dimensions = ImageDimensions::from_image(&image)?;
    let mut sources: Vec<Option<PermutationPixelData>> = vec![None; dimensions.count()];
    for (x_in, y_in, px) in image.enumerate_pixels() {
        let x = i64::from(x_in);
        let y = i64::from(y_in);
        let delta = VectorFieldEntry::from_pixel(px);
        let target = (x + i64::from(delta.0), y + i64::from(delta.1));
        match dimensions.make_linear_index(target.0, target.1) {
            Err(_) => {
                return Err(Box::new(PermutationFlaw::OutOfBounds {
                    dimensions,
                    data: PermutationPixelData {
                        x: x_in.try_into()?,
                        y: y_in.try_into()?,
                        value: delta,
                    },
                }))
            }
            Ok(linear_index) => {
                let element = sources.index_mut(linear_index);
                match element {
                    Some(entry) => {
                        return Err(Box::new(PermutationFlaw::Duplicate {
                            x: target.0.try_into()?,
                            y: target.1.try_into()?,
                            first_source: *entry,
                            second_source: PermutationPixelData {
                                x: x_in.try_into()?,
                                y: y_in.try_into()?,
                                value: delta,
                            },
                        }));
                    }
                    None => {
                        *element = Some(PermutationPixelData {
                            x: x_in.try_into()?,
                            y: y_in.try_into()?,
                            value: delta,
                        })
                    }
                }
            }
        }
    }
    Ok(ValidatedPermutation {
        data: image,
        dimensions,
    })
}

/// Cast a vector field to a permutation without checking if the vector field satisfies
/// permutation constraints
///
/// # Safety
///
/// If the vector field does not satisfy permutation constraints, operations that assume
/// it does satisfy the constraints will produce invalid results.
///
/// Furthermore, permutation constraints allow code to assume that vectors map to independent data.
/// Therefore, violation of the constraints may also lead to nondeterministic behavior, such as
/// caused by race conditions.
pub unsafe fn vector_field_into_validated_permutation_unchecked(
    vector_field: VectorFieldImageBuffer,
) -> ValidatedPermutation {
    let dimensions = ImageDimensions::from_image(&vector_field).unwrap();
    ValidatedPermutation {
        data: vector_field,
        dimensions,
    }
}

#[cfg(test)]
mod tests;
