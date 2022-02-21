use super::super::super::system::{DimensionsMismatchError, SwapOperationInput, System};
use super::super::OutputStatus;
use super::validate_permutation::{
    ValidatePermutation, ValidatePermutationInput, ValidatePermutationParameters,
};
use super::{CompletionStatus, CompletionStatusHolder, FinalFullOutputHolder};
use crate::{CandidatePermutation, DisplacementGoal, ImageDimensions, ValidatedPermutation};
use std::default::Default;
use std::error::Error;

pub struct SwapParameters {}

#[derive(Default)]
pub struct SwapInput {
    pub candidate_permutation: Option<CandidatePermutation>,
    pub displacement_goal: Option<DisplacementGoal>,
}

pub struct SwapOutput {
    pub input_permutation: Option<ValidatedPermutation>,
    pub input_displacement_goal: Option<DisplacementGoal>,
    pub output_permutation: ValidatedPermutation,
}

pub struct Swap {
    completion_status: CompletionStatus,
    validator: Option<ValidatePermutation>,
    input_permutation: Option<ValidatedPermutation>,
    input_displacement_goal: Option<DisplacementGoal>,
    has_given_output: bool,
}

impl Swap {
    pub fn new(mut input: SwapInput, _parameters: SwapParameters) -> Self {
        let validator = input.candidate_permutation.take().map(|permutation| {
            ValidatePermutation::new(
                ValidatePermutationInput {
                    candidate_permutation: permutation,
                },
                ValidatePermutationParameters {},
            )
        });
        Self {
            completion_status: CompletionStatus::new(),
            validator,
            input_permutation: None,
            input_displacement_goal: input.displacement_goal.take(),
            has_given_output: false,
        }
    }

    pub fn step(&mut self, system: &mut System) -> Result<OutputStatus, Box<dyn Error>> {
        self.checked_step(system)
    }

    pub fn partial_output(&self) -> Option<()> {
        None
    }

    pub fn full_output(&mut self, system: &mut System) -> Option<SwapOutput> {
        self.checked_full_output(system)
    }
}

impl CompletionStatusHolder for Swap {
    fn get_status(&self) -> &CompletionStatus {
        &self.completion_status
    }

    fn set_status(&mut self, status: CompletionStatus) {
        self.completion_status = status;
    }

    fn unchecked_step(&mut self, system: &mut System) -> Result<OutputStatus, Box<dyn Error>> {
        match self.validator.take() {
            Some(mut v) => {
                debug_assert!(self.input_permutation.is_none());

                let status = v.step(system)?;
                match status {
                    OutputStatus::NoNewOutput
                    | OutputStatus::NewPartialOutput
                    | OutputStatus::NewFullOutput
                    | OutputStatus::NewPartialAndFullOutput
                    | OutputStatus::FinalPartialOutput => {
                        self.validator = Some(v);
                    }
                    OutputStatus::FinalFullOutput | OutputStatus::FinalPartialAndFullOutput => {
                        self.input_permutation =
                            v.full_output().map(|output| output.validated_permutation);
                    }
                }
                Ok(OutputStatus::NoNewOutput)
            }
            None => {
                if let Some(ref displacement_goal) = self.input_displacement_goal {
                    let dimensions = ImageDimensions::from_image(displacement_goal.as_ref())?;
                    if *system.image_dimensions() != dimensions {
                        return Err(Box::new(DimensionsMismatchError::new(
                            *system.image_dimensions(),
                            dimensions,
                        )));
                    }
                }

                system.operation_swap(&SwapOperationInput {
                    permutation: self.input_permutation.as_ref(),
                    displacement_goal: self.input_displacement_goal.as_ref(),
                })?;
                self.completion_status = CompletionStatus::Finished;
                Ok(OutputStatus::FinalFullOutput)
            }
        }
    }
}

impl FinalFullOutputHolder<SwapOutput> for Swap {
    fn has_given_output(&self) -> bool {
        self.has_given_output
    }
    fn set_has_given_output(&mut self) {
        self.has_given_output = true;
    }

    fn unchecked_full_output(&mut self, system: &mut System) -> Option<SwapOutput> {
        system
            .output_permutation()
            .ok()
            .map(|output_permutation| SwapOutput {
                input_permutation: self.input_permutation.take(),
                input_displacement_goal: self.input_displacement_goal.take(),
                output_permutation,
            })
    }
}
