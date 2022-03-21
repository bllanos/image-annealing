use super::super::super::link::swap::SwapPass;
use super::super::super::output::format::LosslessImage;
use crate::{DisplacementGoal, ValidatedPermutation};
use std::default::Default;

#[derive(Default)]
pub struct PermuteOperationInput<'a> {
    pub permutation: Option<&'a ValidatedPermutation>,
    pub image: Option<&'a LosslessImage>,
}

pub struct SwapOperationInput<'a> {
    pub pass: SwapPass,
    pub permutation: Option<&'a ValidatedPermutation>,
    pub displacement_goal: Option<&'a DisplacementGoal>,
}

impl<'a> SwapOperationInput<'a> {
    pub fn from_pass(pass: SwapPass) -> Self {
        Self {
            pass,
            permutation: None,
            displacement_goal: None,
        }
    }
}
