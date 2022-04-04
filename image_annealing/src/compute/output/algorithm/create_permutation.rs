use super::super::super::system::System;
use super::super::OutputStatus;
use super::{CompletionStatus, CompletionStatusHolder, FinalOutputHolder};
use crate::ValidatedPermutation;
use std::error::Error;

pub struct CreatePermutationParameters {}

pub struct CreatePermutationInput {}

pub struct CreatePermutationOutput {
    pub validated_permutation: ValidatedPermutation,
}

pub struct CreatePermutation {
    completion_status: CompletionStatus,
    has_given_output: bool,
}

impl CreatePermutation {
    pub fn new(_input: CreatePermutationInput, _parameters: &CreatePermutationParameters) -> Self {
        Self {
            completion_status: CompletionStatus::new(),
            has_given_output: false,
        }
    }

    pub fn step(&mut self, system: &mut System) -> Result<OutputStatus, Box<dyn Error>> {
        self.checked_step(system)
    }

    pub fn partial_output(&self) -> Option<()> {
        None
    }

    pub fn full_output(&mut self, system: &mut System) -> Option<CreatePermutationOutput> {
        self.checked_output(system)
    }
}

impl CompletionStatusHolder for CreatePermutation {
    fn get_status(&self) -> &CompletionStatus {
        &self.completion_status
    }

    fn set_status(&mut self, status: CompletionStatus) {
        self.completion_status = status;
    }

    fn unchecked_step(&mut self, system: &mut System) -> Result<OutputStatus, Box<dyn Error>> {
        system.operation_create_permutation()?;
        self.completion_status = CompletionStatus::Finished;
        Ok(OutputStatus::FinalFullOutput)
    }
}

impl FinalOutputHolder<CreatePermutationOutput> for CreatePermutation {
    fn has_given_output(&self) -> bool {
        self.has_given_output
    }
    fn set_has_given_output(&mut self) {
        self.has_given_output = true;
    }

    fn unchecked_output(&mut self, system: &mut System) -> Option<CreatePermutationOutput> {
        system
            .output_permutation()
            .ok()
            .map(|validated_permutation| CreatePermutationOutput {
                validated_permutation,
            })
    }
}
