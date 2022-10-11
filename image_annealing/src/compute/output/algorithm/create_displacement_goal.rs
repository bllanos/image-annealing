use super::super::super::system::{CreateDisplacementGoalOperationInput, DevicePollType, System};
use super::super::format::LosslessImage;
use super::super::OutputStatus;
use super::validate_permutation::{
    ValidatePermutation, ValidatePermutationInput, ValidatePermutationParameters,
};
use super::{CompletionStatus, CompletionStatusHolder, FinalOutputHolder};
use crate::image_utils::check_dimensions_match2;
use crate::{CandidatePermutation, DisplacementGoal, ValidatedPermutation};
use async_trait::async_trait;
use std::default::Default;
use std::error::Error;

pub struct CreateDisplacementGoalParameters {}

#[derive(Default)]
pub struct CreateDisplacementGoalInput {
    pub displacement_goal: Option<DisplacementGoal>,
    pub candidate_permutation: Option<CandidatePermutation>,
    pub image: Option<LosslessImage>,
}

pub struct CreateDisplacementGoalOutput {
    pub input_displacement_goal: Option<DisplacementGoal>,
    pub permutation: Option<ValidatedPermutation>,
    pub image: Option<LosslessImage>,
    pub output_displacement_goal: DisplacementGoal,
}

pub struct CreateDisplacementGoal {
    completion_status: CompletionStatus,
    input: CreateDisplacementGoalInput,
    validator: Option<ValidatePermutation>,
    permutation: Option<ValidatedPermutation>,
    has_given_output: bool,
}

impl CreateDisplacementGoal {
    pub fn new(
        mut input: CreateDisplacementGoalInput,
        _parameters: &CreateDisplacementGoalParameters,
    ) -> Self {
        let validator = input.candidate_permutation.take().map(|permutation| {
            ValidatePermutation::new(
                ValidatePermutationInput {
                    candidate_permutation: permutation,
                },
                &ValidatePermutationParameters {},
            )
        });
        Self {
            completion_status: CompletionStatus::new(),
            input,
            validator,
            permutation: None,
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
    ) -> Option<CreateDisplacementGoalOutput> {
        self.checked_output(system, poll_type).await
    }
}

impl CompletionStatusHolder for CreateDisplacementGoal {
    fn get_status(&self) -> &CompletionStatus {
        &self.completion_status
    }

    fn set_status(&mut self, status: CompletionStatus) {
        self.completion_status = status;
    }

    fn unchecked_step(&mut self, system: &mut System) -> Result<OutputStatus, Box<dyn Error>> {
        match self.validator.as_mut() {
            Some(v) => {
                debug_assert!(self.permutation.is_none());

                let status = v.step(system)?;
                if status.is_final() && status.is_full() {
                    self.permutation = v.full_output().map(|output| output.validated_permutation);
                    self.validator = None;
                }
                Ok(OutputStatus::NoNewOutput)
            }
            None => {
                if let Some(ref displacement_goal) = self.input.displacement_goal {
                    check_dimensions_match2(system, displacement_goal)?;
                }

                if let Some(ref image) = self.input.image {
                    check_dimensions_match2(system, image)?;
                }

                system.operation_create_displacement_goal(
                    &CreateDisplacementGoalOperationInput {
                        displacement_goal: self.input.displacement_goal.as_ref(),
                        permutation: self.permutation.as_ref(),
                        image: self.input.image.as_ref(),
                    },
                )?;
                self.completion_status = CompletionStatus::Finished;
                Ok(OutputStatus::FinalFullOutput)
            }
        }
    }
}

#[async_trait]
impl FinalOutputHolder<CreateDisplacementGoalOutput> for CreateDisplacementGoal {
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
    ) -> Option<CreateDisplacementGoalOutput> {
        system
            .output_displacement_goal(poll_type)
            .await
            .ok()
            .map(|displacement_goal| CreateDisplacementGoalOutput {
                input_displacement_goal: self.input.displacement_goal.take(),
                permutation: self.permutation.take(),
                image: self.input.image.take(),
                output_displacement_goal: displacement_goal,
            })
    }
}
