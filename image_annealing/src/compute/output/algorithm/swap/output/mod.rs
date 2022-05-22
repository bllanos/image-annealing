use super::super::super::super::link::swap::SwapPass;
use crate::{DisplacementGoal, ValidatedPermutation};
use std::fmt;

pub trait SwapRatio: fmt::Display {
    fn is_none_accepted(&self) -> bool {
        self.accepted() == 0
    }

    fn accepted_fraction(&self) -> f64 {
        if self.total() == 0 {
            0.0
        } else {
            self.accepted() as f64 / self.total() as f64
        }
    }

    fn total(&self) -> usize;

    fn accepted(&self) -> usize;
}

pub trait SwapPassSwapRatio: SwapRatio + fmt::Display {
    fn pass(&self) -> SwapPass;
}

pub trait SwapPassSelectionSwapRatio: SwapRatio + fmt::Display {
    fn passes<'a, 'b>(&'a self) -> Box<dyn Iterator<Item = &'a dyn SwapPassSwapRatio> + 'b>
    where
        'a: 'b;
}

pub struct SwapPartialOutput {
    pub counts: Box<dyn SwapPassSelectionSwapRatio>,
}

pub struct SwapFullOutput {
    pub input_permutation: Option<ValidatedPermutation>,
    pub input_displacement_goal: Option<DisplacementGoal>,
    pub output_permutation: ValidatedPermutation,
}

#[cfg(test)]
mod tests;
