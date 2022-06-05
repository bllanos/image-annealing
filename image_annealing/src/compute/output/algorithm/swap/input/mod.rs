use crate::{CandidatePermutation, DisplacementGoal};
use std::default::Default;
use std::error::Error;
use std::fmt;

pub use super::super::super::super::link::swap::{SwapPass, SwapPassSelection};

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

#[derive(Clone, Debug, PartialEq)]
pub struct SwapParameters {
    selection: SwapPassSelection,
    swap_acceptance_threshold: f32,
    count_swap: bool,
}

impl SwapParameters {
    pub fn new(
        selection: SwapPassSelection,
        swap_acceptance_threshold: f32,
        count_swap: bool,
    ) -> Result<Self, InvalidSwapParametersError> {
        if selection.is_empty() {
            Err(InvalidSwapParametersError::NoPassesSelected)
        } else {
            Ok(Self {
                selection,
                swap_acceptance_threshold,
                count_swap,
            })
        }
    }

    pub fn from_selection(
        selection: SwapPassSelection,
    ) -> Result<Self, InvalidSwapParametersError> {
        Self::from_selection_and_threshold(selection, Default::default())
    }

    pub fn from_selection_and_threshold(
        selection: SwapPassSelection,
        swap_acceptance_threshold: f32,
    ) -> Result<Self, InvalidSwapParametersError> {
        Self::new(selection, swap_acceptance_threshold, false)
    }

    pub fn selection(&self) -> SwapPassSelection {
        self.selection
    }

    pub fn swap_acceptance_threshold(&self) -> f32 {
        self.swap_acceptance_threshold
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

#[derive(Debug, Default, Eq, PartialEq)]
pub struct SwapInput {
    pub candidate_permutation: Option<CandidatePermutation>,
    pub displacement_goal: Option<DisplacementGoal>,
}

#[cfg(test)]
mod tests;
