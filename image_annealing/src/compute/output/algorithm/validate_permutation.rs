use super::super::super::system::{DimensionsMismatchError, System};
use super::super::OutputStatus;
use super::{CompletionStatus, CompletionStatusHolder};
use crate::image_utils::validation::{self, CandidatePermutation, ValidatedPermutation};
use crate::ImageDimensions;
use std::error::Error;

pub struct ValidatePermutationParameters {}

pub struct ValidatePermutationInput {
    pub candidate_permutation: CandidatePermutation,
}

pub struct ValidatePermutationOutput {
    pub validated_permutation: ValidatedPermutation,
}

pub struct ValidatePermutation {
    completion_status: CompletionStatus,
    input: Option<ValidatePermutationInput>,
    full_output: Option<ValidatedPermutation>,
}

impl ValidatePermutation {
    pub fn new(
        input: ValidatePermutationInput,
        _parameters: ValidatePermutationParameters,
    ) -> Self {
        Self {
            completion_status: CompletionStatus::new(),
            input: Some(input),
            full_output: None,
        }
    }

    pub fn step(&mut self, system: &mut System) -> Result<OutputStatus, Box<dyn Error>> {
        self.checked_step(system)
    }

    pub fn partial_output(&self) -> Option<()> {
        None
    }

    pub fn full_output(&mut self) -> Option<ValidatePermutationOutput> {
        self.full_output
            .take()
            .map(|validated_permutation| ValidatePermutationOutput {
                validated_permutation,
            })
    }
}

impl CompletionStatusHolder for ValidatePermutation {
    fn get_status(&self) -> &CompletionStatus {
        &self.completion_status
    }

    fn set_status(&mut self, status: CompletionStatus) {
        self.completion_status = status;
    }

    fn unchecked_step(&mut self, system: &mut System) -> Result<OutputStatus, Box<dyn Error>> {
        debug_assert!(self.full_output.is_none());

        let ValidatePermutationInput {
            candidate_permutation,
        } = self.input.take().unwrap();
        let dimensions = ImageDimensions::from_image(&candidate_permutation.0)?;
        if *system.image_dimensions() == dimensions {
            self.full_output = Some(validation::validate_permutation(candidate_permutation.0)?);
            self.completion_status = CompletionStatus::Finished;
            Ok(OutputStatus::FinalFullOutput)
        } else {
            Err(Box::new(DimensionsMismatchError::new(
                *system.image_dimensions(),
                dimensions,
            )))
        }
    }
}
