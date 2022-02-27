use super::super::super::link::swap::SwapPass;
use crate::{DisplacementGoal, ValidatedPermutation};
use std::default::Default;

#[derive(Default)]
pub struct PermuteOperationInput<'a> {
    pub permutation: Option<&'a ValidatedPermutation>,
    pub image: Option<&'a image::DynamicImage>,
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
