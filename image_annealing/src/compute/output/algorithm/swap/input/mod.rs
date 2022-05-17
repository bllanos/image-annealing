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

#[derive(Clone, Debug, Eq, PartialEq)]
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

#[derive(Debug, Default, Eq, PartialEq)]
pub struct SwapInput {
    pub candidate_permutation: Option<CandidatePermutation>,
    pub displacement_goal: Option<DisplacementGoal>,
}

#[cfg(test)]
mod tests;
