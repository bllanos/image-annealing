use super::super::super::system::{DevicePollType, System};
use super::super::OutputStatus;
use super::{CompletionStatus, CompletionStatusHolder, FinalOutputHolder};
use crate::ValidatedPermutation;
use async_trait::async_trait;
use std::error::Error;

#[derive(Default)]
pub struct CreatePermutationParameters {}

#[derive(Default)]
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

    pub async fn partial_output(&self, _poll_type: DevicePollType) -> Option<()> {
        None
    }

    pub async fn full_output(
        &mut self,
        system: &mut System,
        poll_type: DevicePollType,
    ) -> Option<CreatePermutationOutput> {
        self.checked_output(system, poll_type).await
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

#[async_trait]
impl FinalOutputHolder<CreatePermutationOutput> for CreatePermutation {
    fn has_given_output(&self) -> bool {
        self.has_given_output
    }

    fn set_has_given_output(&mut self) {
        self.has_given_output = true;
    }

    async fn unchecked_output(
        &mut self,
        system: &mut System,
        poll_type: DevicePollType,
    ) -> Option<CreatePermutationOutput> {
        system
            .output_permutation(poll_type)
            .await
            .ok()
            .map(|validated_permutation| CreatePermutationOutput {
                validated_permutation,
            })
    }
}
