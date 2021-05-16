use super::super::super::dispatch::{DimensionsMismatchError, DispatcherImplementation};
use super::super::format::PermutationImageBuffer;
use super::super::OutputStatus;
use super::CompletionStatus;
use crate::image_utils::validation::{self, ValidatedPermutation};
use crate::image_utils::ImageDimensions;
use std::error::Error;

pub struct ValidatePermutationParameters {}

pub struct ValidatePermutationInput {
    pub candidate_permutation: PermutationImageBuffer,
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

    pub fn step(
        &mut self,
        dispatcher: &DispatcherImplementation,
    ) -> Result<OutputStatus, Box<dyn Error>> {
        self.completion_status.ok_if_pending()?;
        debug_assert!(self.full_output.is_none());

        let ValidatePermutationInput {
            candidate_permutation,
        } = self.input.take().unwrap();
        match ImageDimensions::from_image(&candidate_permutation) {
            Ok(dimensions) => {
                if *dispatcher.image_dimensions() == dimensions {
                    match validation::validate_permutation(candidate_permutation) {
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
                    Err(Box::new(DimensionsMismatchError))
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

    pub fn full_output(&mut self) -> Option<ValidatedPermutation> {
        self.full_output.take()
    }
}
