use super::ImageDimensions;
use crate::compute::conversion::{PermutationEntry, PermutationEntryComponent};
use crate::compute::format::{PermutationImageBuffer, PermutationImageBufferComponent};
use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::ops::IndexMut;

#[derive(Debug, Clone, Copy)]
pub struct PermutationPixelData {
    x: u32,
    y: u32,
    value: PermutationEntry,
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
        x: u32,
        y: u32,
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

impl Error for PermutationFlaw {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub struct ValidatedPermutation {
    data: PermutationImageBuffer,
    dimensions: ImageDimensions,
}

impl ValidatedPermutation {
    pub fn as_raw_slice(&self) -> &[PermutationImageBufferComponent] {
        &self.data.as_raw().as_slice()
    }
    pub fn dimensions(&self) -> ImageDimensions {
        self.dimensions
    }
}

impl AsRef<PermutationImageBuffer> for ValidatedPermutation {
    fn as_ref(&self) -> &PermutationImageBuffer {
        &self.data
    }
}

pub fn validate_permutation(
    image: PermutationImageBuffer,
) -> Result<ValidatedPermutation, Box<dyn Error>> {
    let dimensions = ImageDimensions::from_image(&image)?;
    let mut sources: Vec<Option<PermutationPixelData>> = vec![None; dimensions.count()];
    for (x_in, y_in, px) in image.enumerate_pixels() {
        let x = x_in as i64;
        let y = y_in as i64;
        let delta = PermutationEntry(
            PermutationEntryComponent::from_be_bytes([px[0], px[1]]),
            PermutationEntryComponent::from_be_bytes([px[2], px[3]]),
        );
        let target = (x + (delta.0 as i64), y + (delta.1 as i64));
        match dimensions.make_linear_index(target.0, target.1) {
            Err(_) => {
                return Err(Box::new(PermutationFlaw::OutOfBounds {
                    dimensions,
                    data: PermutationPixelData {
                        x: x_in,
                        y: y_in,
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
                                x: x_in,
                                y: y_in,
                                value: delta,
                            },
                        }));
                    }
                    None => {
                        *element = Some(PermutationPixelData {
                            x: x_in,
                            y: y_in,
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

#[cfg(test)]
mod tests;
