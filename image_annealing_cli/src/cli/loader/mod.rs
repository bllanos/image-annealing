use super::super::config::{DisplacementGoalPath, LosslessImagePath, PermutationPath};
use image_annealing::compute::format::{ImageFileReader, LosslessImage, Rgba8Image};
use image_annealing::{CandidatePermutation, DisplacementGoal};
use std::error::Error;

pub fn load_candidate_permutation(
    path: &PermutationPath,
) -> Result<CandidatePermutation, Box<dyn Error>> {
    Rgba8Image::load(path)
}

pub fn load_displacement_goal(
    path: &DisplacementGoalPath,
) -> Result<DisplacementGoal, Box<dyn Error>> {
    DisplacementGoal::load(path)
}

pub fn load_image(path: &LosslessImagePath) -> Result<LosslessImage, Box<dyn Error>> {
    LosslessImage::load(path.format(), path.to_vec().as_slice())
}
