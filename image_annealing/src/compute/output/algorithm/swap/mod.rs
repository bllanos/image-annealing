use super::super::super::system::{SwapOperationInput, System};
use super::super::OutputStatus;
use super::validate_permutation::{
    ValidatePermutation, ValidatePermutationInput, ValidatePermutationParameters,
};
use super::{CompletionStatus, CompletionStatusHolder, FinalOutputHolder};
use crate::image_utils::check_dimensions_match2;
use crate::{DisplacementGoal, ValidatedPermutation};
use std::error::Error;

mod input;
mod output;

pub use input::{
    InvalidSwapPassSelectionError, SwapInput, SwapParameters, SwapPass, SwapPassSequence,
    SwapPassSet,
};
pub use output::{
    SwapFullOutput, SwapPartialOutput, SwapPassSequenceSwapRatio, SwapPassSwapRatio, SwapRatio,
    SwapReturnedInput,
};

pub struct Swap {
    completion_status: CompletionStatus,
    validator: Option<ValidatePermutation>,
    input_permutation: Option<ValidatedPermutation>,
    input_displacement_goal: Option<DisplacementGoal>,
    sequence: SwapPassSequence,
    remaining_passes: Option<std::iter::Peekable<<SwapPassSequence as IntoIterator>::IntoIter>>,
    previous_pass: Option<SwapPass>,
    swap_acceptance_threshold: f32,
    do_count_swap: bool,
    has_given_partial_output: bool,
    has_given_full_output: bool,
}

impl Swap {
    pub fn new(mut input: SwapInput, parameters: &SwapParameters) -> Self {
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
            validator,
            input_permutation: None,
            input_displacement_goal: input.displacement_goal.take(),
            sequence: parameters.sequence,
            remaining_passes: None,
            previous_pass: None,
            swap_acceptance_threshold: parameters.swap_acceptance_threshold,
            do_count_swap: parameters.count_swap,
            has_given_partial_output: false,
            has_given_full_output: false,
        }
    }

    pub fn step(&mut self, system: &mut System) -> Result<OutputStatus, Box<dyn Error>> {
        self.checked_step(system)
    }

    pub fn partial_output(&mut self, system: &mut System) -> Option<SwapPartialOutput> {
        FinalOutputHolder::<SwapPartialOutput>::checked_output(self, system)
    }

    pub fn full_output(&mut self, system: &mut System) -> Option<SwapFullOutput> {
        if self.has_given_full_output {
            None
        } else {
            match self.completion_status {
                CompletionStatus::Failed => None,
                _ => match self.previous_pass {
                    Some(pass) => {
                        self.has_given_full_output = true;
                        system
                            .output_permutation()
                            .ok()
                            .map(|output_permutation| SwapFullOutput {
                                input: {
                                    let permutation = self.input_permutation.take();
                                    let displacement_goal = self.input_displacement_goal.take();
                                    if permutation.is_some() || displacement_goal.is_some() {
                                        Some(SwapReturnedInput {
                                            permutation,
                                            displacement_goal,
                                        })
                                    } else {
                                        None
                                    }
                                },
                                output_permutation,
                                pass,
                            })
                    }
                    None => None,
                },
            }
        }
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
        let output_status = match self.validator.take() {
            Some(mut v) => {
                debug_assert!(self.input_permutation.is_none());

                let status = v.step(system)?;
                if status.is_final() && status.is_full() {
                    self.input_permutation =
                        v.full_output().map(|output| output.validated_permutation);
                } else {
                    self.validator = Some(v);
                }
                OutputStatus::NoNewOutput
            }
            None => {
                let is_first_pass = match self.remaining_passes {
                    Some(_) => false,
                    None => {
                        self.remaining_passes = Some(self.sequence.into_iter().peekable());
                        true
                    }
                };
                match self.remaining_passes.as_mut().unwrap().next() {
                    Some(pass) => {
                        if is_first_pass {
                            if let Some(ref displacement_goal) = self.input_displacement_goal {
                                check_dimensions_match2(system, displacement_goal)?;
                            }

                            system.operation_swap(&SwapOperationInput {
                                pass,
                                acceptance_threshold: self.swap_acceptance_threshold,
                                permutation: self.input_permutation.as_ref(),
                                displacement_goal: self.input_displacement_goal.as_ref(),
                            })?;
                        } else {
                            system.operation_swap(&SwapOperationInput::from_pass_and_threshold(
                                pass,
                                self.swap_acceptance_threshold,
                            ))?;
                        }
                        self.previous_pass = Some(pass);
                        self.has_given_full_output = false;
                        match self.remaining_passes.as_mut().unwrap().peek() {
                            Some(_) => OutputStatus::NewFullOutput,
                            None => {
                                if self.do_count_swap {
                                    OutputStatus::NewFullOutput
                                } else {
                                    self.completion_status = CompletionStatus::Finished;
                                    OutputStatus::FinalFullOutput
                                }
                            }
                        }
                    }
                    None => {
                        if self.do_count_swap {
                            system.operation_count_swap(self.sequence)?;
                            self.completion_status = CompletionStatus::Finished;
                            OutputStatus::FinalPartialOutput
                        } else {
                            unreachable!()
                        }
                    }
                }
            }
        };
        Ok(output_status)
    }
}

impl FinalOutputHolder<SwapPartialOutput> for Swap {
    fn has_given_output(&self) -> bool {
        self.has_given_partial_output
    }
    fn set_has_given_output(&mut self) {
        self.has_given_partial_output = true;
    }

    fn unchecked_output(&mut self, system: &mut System) -> Option<SwapPartialOutput> {
        system
            .output_count_swap(&self.sequence)
            .ok()
            .map(|counts| SwapPartialOutput {
                counts: Box::new(counts),
            })
    }
}
