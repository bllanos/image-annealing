use crate::{CandidatePermutation, DisplacementGoal};
use std::default::Default;

pub use super::super::super::super::link::swap::{
    InvalidSwapPassSelectionError, SwapPass, SwapPassSequence, SwapPassSet,
};

#[derive(Clone, Debug, PartialEq)]
pub struct SwapParameters {
    pub sequence: SwapPassSequence,
    pub swap_acceptance_threshold: f32,
    pub count_swap: bool,
}

impl SwapParameters {
    pub fn from_sequence(sequence: SwapPassSequence) -> Self {
        Self::from_sequence_and_threshold(sequence, Default::default())
    }

    pub fn from_sequence_and_threshold(
        sequence: SwapPassSequence,
        swap_acceptance_threshold: f32,
    ) -> Self {
        Self {
            sequence,
            swap_acceptance_threshold,
            count_swap: false,
        }
    }
}

impl Default for SwapParameters {
    fn default() -> Self {
        Self::from_sequence(SwapPassSequence::all())
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct SwapInput {
    pub candidate_permutation: Option<CandidatePermutation>,
    pub displacement_goal: Option<DisplacementGoal>,
}

#[cfg(test)]
mod tests;
