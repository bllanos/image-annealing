use super::validation::{self, CandidatePermutation, ValidatedPermutation};
use super::{manipulation, VectorField};
use crate::compute::format::{
    self, ImageFileReader, ImageFileWriter, ImageFileWriterSaveResult, Rgba8Image,
    VectorFieldImageBuffer, VectorFieldImageBufferComponent,
};
use crate::{ImageDimensions, ImageDimensionsHolder};
use std::error::Error;
use std::path::Path;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DisplacementGoal(Rgba8Image);

impl DisplacementGoal {
    pub fn from_vector_field_image(image: Rgba8Image) -> Self {
        Self(image)
    }

    pub fn from_vector_field(vector_field: VectorFieldImageBuffer) -> Result<Self, Box<dyn Error>> {
        Ok(Self(Rgba8Image::new(vector_field)?))
    }

    pub fn from_raw_candidate_permutation(
        vector_field: VectorFieldImageBuffer,
    ) -> Result<Self, Box<dyn Error>> {
        let validated_permutation = validation::validate_permutation(vector_field)?;
        Ok(Self::from(validated_permutation))
    }

    pub fn from_candidate_permutation(
        candidate_permutation: CandidatePermutation,
    ) -> Result<Self, Box<dyn Error>> {
        Self::from_raw_candidate_permutation(candidate_permutation.into_inner())
    }
}

impl AsRef<VectorFieldImageBuffer> for DisplacementGoal {
    fn as_ref(&self) -> &VectorFieldImageBuffer {
        self.0.as_ref()
    }
}

impl PartialEq<VectorFieldImageBuffer> for DisplacementGoal {
    fn eq(&self, other: &VectorFieldImageBuffer) -> bool {
        self.as_ref() == other
    }
}

impl ImageDimensionsHolder for DisplacementGoal {
    fn dimensions(&self) -> &ImageDimensions {
        self.0.dimensions()
    }
}

impl VectorField for DisplacementGoal {
    fn identity(dimensions: &ImageDimensions) -> Self {
        Self(Rgba8Image::new(format::identity(dimensions)).unwrap())
    }

    fn into_inner(self) -> VectorFieldImageBuffer {
        self.0.into_inner()
    }

    fn as_raw_slice(&self) -> &[VectorFieldImageBufferComponent] {
        self.0.as_ref().as_raw().as_slice()
    }
}

impl ImageFileReader for DisplacementGoal {
    fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        Ok(Self::from_vector_field_image(
            <Rgba8Image as ImageFileReader>::load(path)?,
        ))
    }
}

impl ImageFileWriter for DisplacementGoal {
    const EXTENSION: &'static str = <Rgba8Image as ImageFileWriter>::EXTENSION;

    fn save_add_extension<P: AsRef<Path>>(
        &self,
        path_no_extension: P,
    ) -> ImageFileWriterSaveResult {
        self.0.save_add_extension(path_no_extension)
    }
}

impl From<ValidatedPermutation> for DisplacementGoal {
    fn from(value: ValidatedPermutation) -> Self {
        Self::from_vector_field(manipulation::invert_permutation(&value)).unwrap()
    }
}

#[cfg(test)]
mod tests;
