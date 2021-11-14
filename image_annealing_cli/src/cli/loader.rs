use image::io::Reader as ImageReader;
use image::DynamicImage;
use image_annealing::compute::format::VectorFieldImageBuffer;
use image_annealing::CandidatePermutation;
use std::error::Error;
use std::path::Path;

fn load_vector_field<P: AsRef<Path>>(path: P) -> Result<VectorFieldImageBuffer, Box<dyn Error>> {
    Ok(ImageReader::open(path)?.decode()?.to_rgba8())
}

pub fn load_candidate_permutation<P: AsRef<Path>>(
    path: P,
) -> Result<CandidatePermutation, Box<dyn Error>> {
    Ok(CandidatePermutation(load_vector_field(path)?))
}

pub fn load_image<P: AsRef<Path>>(path: P) -> Result<DynamicImage, Box<dyn Error>> {
    Ok(ImageReader::open(path)?.decode()?)
}
