use super::super::super::system::{DimensionsMismatchError, System};
use super::super::OutputStatus;
use super::CompletionStatus;
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

    pub fn step(&mut self, system: &System) -> Result<OutputStatus, Box<dyn Error>> {
        self.completion_status.ok_if_pending()?;
        debug_assert!(self.full_output.is_none());

        let ValidatePermutationInput {
            candidate_permutation,
        } = self.input.take().unwrap();
        match ImageDimensions::from_image(&candidate_permutation.0) {
            Ok(dimensions) => {
                if *system.image_dimensions() == dimensions {
                    match validation::validate_permutation(candidate_permutation.0) {
                        Ok(validated_permutation) => {
                            self.full_output = Some(validated_permutation);
                            self.completion_status = CompletionStatus::Finished;
                            Ok(OutputStatus::FinalFullOutput)
                        }
                        Err(e) => {
                            self.completion_status = CompletionStatus::Failed;
                            Err(e)
                        }
                    }
                } else {
                    self.completion_status = CompletionStatus::Failed;
                    Err(Box::new(DimensionsMismatchError::new(
                        *system.image_dimensions(),
                        dimensions,
                    )))
                }
            }
            Err(e) => {
                self.completion_status = CompletionStatus::Failed;
                Err(Box::new(e))
            }
        }
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
