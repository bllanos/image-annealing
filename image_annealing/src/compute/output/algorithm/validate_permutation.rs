use super::super::super::system::System;
use super::super::OutputStatus;
use super::{CompletionStatus, CompletionStatusHolder};
use crate::image_utils::check_dimensions_match2;
use crate::image_utils::validation::{self, CandidatePermutation, ValidatedPermutation};
use crate::VectorField;
use std::error::Error;

#[derive(Default)]
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
        _parameters: &ValidatePermutationParameters,
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
        check_dimensions_match2(system, &candidate_permutation)?;
        self.full_output = Some(validation::validate_permutation(
            candidate_permutation.into_inner(),
        )?);
        self.completion_status = CompletionStatus::Finished;
        Ok(OutputStatus::FinalFullOutput)
    }
}
