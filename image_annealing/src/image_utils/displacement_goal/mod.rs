use super::manipulation;
use super::validation::{self, CandidatePermutation, ValidatedPermutation};
use crate::compute::format::{VectorFieldImageBuffer, VectorFieldImageBufferComponent};
use std::error::Error;

pub struct DisplacementGoal(VectorFieldImageBuffer);

impl DisplacementGoal {
    pub fn from_vector_field(vector_field: VectorFieldImageBuffer) -> Self {
        Self(vector_field)
    }
    pub fn from_candidate_permutation(
        candidate_permutation: CandidatePermutation,
    ) -> Result<Self, Box<dyn Error>> {
        let validated_permutation = validation::validate_permutation(candidate_permutation.0)?;
        Ok(Self::from(validated_permutation))
    }
    pub fn as_raw_slice(&self) -> &[VectorFieldImageBufferComponent] {
        self.0.as_raw().as_slice()
    }
}

impl AsRef<VectorFieldImageBuffer> for DisplacementGoal {
    fn as_ref(&self) -> &VectorFieldImageBuffer {
        &self.0
    }
}

impl From<ValidatedPermutation> for DisplacementGoal {
    fn from(permutation: ValidatedPermutation) -> Self {
        Self(manipulation::invert_permutation(&permutation))
    }
}

#[cfg(test)]
mod tests;
