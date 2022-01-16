use crate::{CandidatePermutationPath, DisplacementGoalPath, ImagePath};
use image::io::Reader as ImageReader;
use image::DynamicImage;
use image_annealing::compute::format::VectorFieldImageBuffer;
use image_annealing::{CandidatePermutation, DisplacementGoal};
use std::error::Error;
use std::path::Path;

fn load_vector_field<P: AsRef<Path>>(path: P) -> Result<VectorFieldImageBuffer, Box<dyn Error>> {
    Ok(ImageReader::open(path)?.decode()?.to_rgba8())
}

pub fn load_candidate_permutation(
    path: &CandidatePermutationPath,
) -> Result<CandidatePermutation, Box<dyn Error>> {
    Ok(CandidatePermutation(load_vector_field(&path.0)?))
}

pub fn load_displacement_goal(
    path: &DisplacementGoalPath,
) -> Result<DisplacementGoal, Box<dyn Error>> {
    Ok(DisplacementGoal::from_vector_field(load_vector_field(
        &path.0,
    )?))
}

pub fn load_image(path: &ImagePath) -> Result<DynamicImage, Box<dyn Error>> {
    Ok(ImageReader::open(&path.0)?.decode()?)
}
