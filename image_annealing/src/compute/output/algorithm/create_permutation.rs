use super::super::super::system::System;
use super::super::OutputStatus;
use super::CompletionStatus;
use crate::image_utils::validation::ValidatedPermutation;
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
    pub fn new(_input: CreatePermutationInput, _parameters: CreatePermutationParameters) -> Self {
        Self {
            completion_status: CompletionStatus::new(),
            has_given_output: false,
        }
    }

    pub fn step(&mut self, system: &mut System) -> Result<OutputStatus, Box<dyn Error>> {
        self.completion_status.ok_if_pending()?;
        match system.operation_create_permutation() {
            Ok(_) => {
                self.completion_status = CompletionStatus::Finished;
                Ok(OutputStatus::FinalFullOutput)
            }
            Err(e) => {
                self.completion_status = CompletionStatus::Failed;
                Err(e)
            }
        }
    }

    pub fn partial_output(&self) -> Option<()> {
        None
    }

    pub fn full_output(&mut self, system: &mut System) -> Option<CreatePermutationOutput> {
        if self.has_given_output {
            None
        } else {
            match self.completion_status {
                CompletionStatus::Finished => {
                    self.has_given_output = true;
                    system
                        .output_permutation()
                        .ok()
                        .map(|validated_permutation| CreatePermutationOutput {
                            validated_permutation,
                        })
                }
                _ => None,
            }
        }
    }
}
