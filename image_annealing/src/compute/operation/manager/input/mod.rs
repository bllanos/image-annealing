use super::super::super::link::swap::SwapPass;
use super::super::super::output::format::LosslessImage;
use crate::{DisplacementGoal, ValidatedPermutation};
use std::default::Default;

#[derive(Default)]
pub struct CreateDisplacementGoalOperationInput<'a> {
    pub displacement_goal: Option<&'a DisplacementGoal>,
    pub permutation: Option<&'a ValidatedPermutation>,
    pub image: Option<&'a LosslessImage>,
}

#[derive(Default)]
pub struct PermuteOperationInput<'a> {
    pub permutation: Option<&'a ValidatedPermutation>,
    pub image: Option<&'a LosslessImage>,
}

#[derive(Debug, PartialEq)]
pub struct SwapOperationInput<'a> {
    pub pass: SwapPass,
    pub acceptance_threshold: f32,
    pub permutation: Option<&'a ValidatedPermutation>,
    pub displacement_goal: Option<&'a DisplacementGoal>,
}

impl<'a> SwapOperationInput<'a> {
    pub fn from_pass_and_threshold(pass: SwapPass, acceptance_threshold: f32) -> Self {
        Self {
            pass,
            acceptance_threshold,
            permutation: None,
            displacement_goal: None,
        }
    }
}

#[cfg(test)]
mod tests;
