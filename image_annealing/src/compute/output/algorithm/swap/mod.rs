use super::super::super::system::{SwapOperationInput, System};
use super::super::OutputStatus;
use super::validate_permutation::{
    ValidatePermutation, ValidatePermutationInput, ValidatePermutationParameters,
};
use super::{CompletionStatus, CompletionStatusHolder, FinalOutputHolder};
use crate::image_utils::check_dimensions_match2;
use crate::{CandidatePermutation, DisplacementGoal, ValidatedPermutation};
use std::default::Default;
use std::error::Error;
use std::fmt;

pub use super::super::super::link::swap::{SwapPass, SwapPassSelection};
pub use super::super::super::system::{CountSwapOperationOutput, CountSwapOperationOutputPass};

#[derive(Debug, Clone)]
pub enum InvalidSwapParametersError {
    NoPassesSelected,
}

impl fmt::Display for InvalidSwapParametersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InvalidSwapParametersError::NoPassesSelected => {
                write!(f, "selection of swap passes is empty")
            }
        }
    }
}

impl Error for InvalidSwapParametersError {}

#[derive(Debug, Clone)]
pub struct SwapParameters {
    selection: SwapPassSelection,
    count_swap: bool,
}

impl SwapParameters {
    pub fn new(
        selection: SwapPassSelection,
        count_swap: bool,
    ) -> Result<Self, InvalidSwapParametersError> {
        if selection.is_empty() {
            Err(InvalidSwapParametersError::NoPassesSelected)
        } else {
            Ok(Self {
                selection,
                count_swap,
            })
        }
    }

    pub fn from_selection(
        selection: SwapPassSelection,
    ) -> Result<Self, InvalidSwapParametersError> {
        Self::new(selection, false)
    }

    pub fn selection(&self) -> SwapPassSelection {
        self.selection
    }

    pub fn count_swap(&self) -> bool {
        self.count_swap
    }
}

impl Default for SwapParameters {
    fn default() -> Self {
        Self::from_selection(SwapPassSelection::all()).unwrap()
    }
}

#[derive(Default)]
pub struct SwapInput {
    pub candidate_permutation: Option<CandidatePermutation>,
    pub displacement_goal: Option<DisplacementGoal>,
}

pub struct SwapPartialOutput {
    pub counts: CountSwapOperationOutput,
}

pub struct SwapFullOutput {
    pub input_permutation: Option<ValidatedPermutation>,
    pub input_displacement_goal: Option<DisplacementGoal>,
    pub output_permutation: ValidatedPermutation,
}

pub struct Swap {
    completion_status: CompletionStatus,
    validator: Option<ValidatePermutation>,
    input_permutation: Option<ValidatedPermutation>,
    input_displacement_goal: Option<DisplacementGoal>,
    is_first_pass: bool,
    remaining_passes: Vec<SwapPass>,
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
            is_first_pass: true,
            // TODO use the selected swap passes, in reverse
            remaining_passes: vec![SwapPass::Horizontal],
            do_count_swap: parameters.count_swap(),
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
        FinalOutputHolder::<SwapFullOutput>::checked_output(self, system)
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
            None => match self.remaining_passes.pop() {
                Some(pass) => {
                    if self.is_first_pass {
                        if let Some(ref displacement_goal) = self.input_displacement_goal {
                            check_dimensions_match2(system, displacement_goal)?;
                        }

                        system.operation_swap(&SwapOperationInput {
                            pass,
                            permutation: self.input_permutation.as_ref(),
                            displacement_goal: self.input_displacement_goal.as_ref(),
                        })?;
                        self.is_first_pass = false;
                    } else {
                        system.operation_swap(&SwapOperationInput::from_pass(pass))?;
                    }
                    Ok(OutputStatus::NoNewOutput)
                }
                None => {
                    let output_status = if self.do_count_swap {
                        system.operation_count_swap()?;
                        OutputStatus::FinalPartialAndFullOutput
                    } else {
                        OutputStatus::FinalFullOutput
                    };
                    self.completion_status = CompletionStatus::Finished;
                    Ok(output_status)
                }
            },
        }
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
            .output_count_swap()
            .ok()
            .map(|counts| SwapPartialOutput { counts })
    }
}

impl FinalOutputHolder<SwapFullOutput> for Swap {
    fn has_given_output(&self) -> bool {
        self.has_given_full_output
    }
    fn set_has_given_output(&mut self) {
        self.has_given_full_output = true;
    }

    fn unchecked_output(&mut self, system: &mut System) -> Option<SwapFullOutput> {
        system
            .output_permutation()
            .ok()
            .map(|output_permutation| SwapFullOutput {
                input_permutation: self.input_permutation.take(),
                input_displacement_goal: self.input_displacement_goal.take(),
                output_permutation,
            })
    }
}

#[cfg(test)]
mod tests;
