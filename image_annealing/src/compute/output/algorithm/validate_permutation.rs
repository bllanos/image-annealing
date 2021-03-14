use super::super::super::dispatch::{DimensionsMismatchError, DispatcherImplementation};
use super::super::format::PermutationImageBuffer;
use super::super::{AlreadyFailedError, OutputStatus};
use crate::image_utils::validation::{self, ValidatedPermutation};
use crate::image_utils::ImageDimensions;
use std::error::Error;

pub struct ValidatePermutationParameters {}

pub struct ValidatePermutationInput {
    pub candidate_permutation: PermutationImageBuffer,
}

pub struct ValidatePermutation {
    input: Option<ValidatePermutationInput>,
    full_output: Option<ValidatedPermutation>,
}

impl ValidatePermutation {
    pub fn new(
        input: ValidatePermutationInput,
        _parameters: ValidatePermutationParameters,
    ) -> Self {
        Self {
            input: Some(input),
            full_output: None,
        }
    }

    pub fn step(
        &mut self,
        dispatcher: &DispatcherImplementation,
    ) -> Result<OutputStatus, Box<dyn Error>> {
        if let Some(ValidatePermutationInput {
            candidate_permutation,
        }) = self.input.take()
        {
            if *dispatcher.image_dimensions()
                == ImageDimensions::from_image(&candidate_permutation)?
            {
                self.full_output = Some(validation::validate_permutation(candidate_permutation)?);
            } else {
                return Err(Box::new(DimensionsMismatchError));
            }
        }
        match self.full_output.as_ref() {
            Some(_) => Ok(OutputStatus::FinalFullOutput),
            None => Err(Box::new(AlreadyFailedError)),
        }
    }

    pub fn partial_output(&self) -> Option<&()> {
        None
    }

    pub fn full_output(&self) -> Option<&ValidatedPermutation> {
        self.full_output.as_ref()
    }
}
