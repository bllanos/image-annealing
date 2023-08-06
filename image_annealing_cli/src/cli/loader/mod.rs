use super::super::config::{
    InputDisplacementGoalPath, InputLosslessImagePath, InputPermutationPath,
};
use image_annealing::compute::format::{ImageFileReader, LosslessImage};
use image_annealing::{CandidatePermutation, DisplacementGoal};
use std::error::Error;

pub fn load_candidate_permutation(
    path: &InputPermutationPath,
) -> Result<CandidatePermutation, Box<dyn Error>> {
    CandidatePermutation::load(path.0 .0.as_ref())
}

pub fn load_displacement_goal(
    path: &InputDisplacementGoalPath,
) -> Result<DisplacementGoal, Box<dyn Error>> {
    DisplacementGoal::load(path.0 .0.as_ref())
}

pub fn load_image(path: &InputLosslessImagePath) -> Result<LosslessImage, Box<dyn Error>> {
    LosslessImage::load(path.format(), path.as_vec().as_slice())
}
