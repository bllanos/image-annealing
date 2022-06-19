use super::super::super::super::link::swap::SwapPass;
use crate::{DisplacementGoal, ValidatedPermutation};
use std::fmt;

pub trait SwapRatio: fmt::Display {
    fn is_none_accepted(&self) -> bool {
        self.accepted() == 0
    }

    fn accepted_fraction(&self) -> f64 {
        let total = self.total();
        if total == 0 {
            0.0
        } else {
            self.accepted() as f64 / total as f64
        }
    }

    fn total(&self) -> usize;

    fn accepted(&self) -> usize;
}

pub trait SwapPassSwapRatio: SwapRatio + fmt::Display {
    fn pass(&self) -> SwapPass;
}

pub trait SwapPassSequenceSwapRatio: SwapRatio + fmt::Display {
    fn passes<'a, 'b>(&'a self) -> Box<dyn Iterator<Item = &'a dyn SwapPassSwapRatio> + 'b>
    where
        'a: 'b;
}

pub struct SwapPartialOutput {
    pub counts: Box<dyn SwapPassSequenceSwapRatio>,
}

pub struct SwapReturnedInput {
    pub permutation: Option<ValidatedPermutation>,
    pub displacement_goal: Option<DisplacementGoal>,
}

pub struct SwapFullOutput {
    pub input: Option<SwapReturnedInput>,
    pub output_permutation: ValidatedPermutation,
    pub pass: SwapPass,
}

#[cfg(test)]
mod tests;
