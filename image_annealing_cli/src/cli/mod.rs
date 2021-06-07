use crate::config::Config;
use image::io::Reader as ImageReader;
use image::DynamicImage;
use image_annealing::compute::format::{ImageFileWriter, PermutationImageBuffer};
use image_annealing::compute::{
    self, CreatePermutationInput, CreatePermutationParameters, Dispatcher, OutputStatus,
    PermuteInput, PermuteParameters, ValidatePermutationInput, ValidatePermutationParameters,
};
use image_annealing::image_utils::ImageDimensions;
use std::error::Error;
use std::path::Path;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let dispatcher = create_dispatcher(&config)?;
    run_and_save(dispatcher, &config)?;
    Ok(())
}

fn create_dispatcher(config: &Config) -> Result<Box<dyn Dispatcher>, Box<dyn Error>> {
    let dimensions = match config {
        Config::CreatePermutation {
            image_dimensions, ..
        } => *image_dimensions,
        Config::Permute {
            original_image_path,
            ..
        } => ImageDimensions::from_image_path(original_image_path)?,
        Config::ValidatePermutation {
            candidate_permutation_path,
        } => ImageDimensions::from_image_path(candidate_permutation_path)?,
    };
    compute::create_dispatcher(&dimensions)
}

fn load_permutation<P: AsRef<Path>>(path: P) -> Result<PermutationImageBuffer, Box<dyn Error>> {
    Ok(ImageReader::open(path)?.decode()?.to_rgba8())
}

fn load_image<P: AsRef<Path>>(path: P) -> Result<DynamicImage, Box<dyn Error>> {
    Ok(ImageReader::open(path)?.decode()?)
}

fn run_and_save(dispatcher: Box<dyn Dispatcher>, config: &Config) -> Result<(), Box<dyn Error>> {
    match config {
        Config::CreatePermutation {
            permutation_output_path_no_extension: path,
            ..
        } => {
            let mut algorithm = dispatcher
                .create_permutation(CreatePermutationInput {}, CreatePermutationParameters {});
            algorithm.step_until(OutputStatus::FinalFullOutput)?;
            let img = algorithm.full_output().unwrap();
            let output_path = img.save_add_extension(path)?;
            println!("Wrote permutation to: {}", output_path.display());
        }
        Config::Permute {
            candidate_permutation_path,
            original_image_path,
            permuted_image_output_path_no_extension: path,
        } => {
            let mut algorithm = dispatcher.permute(
                PermuteInput {
                    candidate_permutation: Some(load_permutation(candidate_permutation_path)?),
                    original_image: Some(load_image(original_image_path)?),
                },
                PermuteParameters {},
            );
            algorithm.step_until(OutputStatus::FinalFullOutput)?;
            let img = algorithm.full_output().unwrap().permuted_image;
            let output_path = img.save_add_extension(path)?;
            println!("Wrote permuted image to: {}", output_path.display());
        }
        Config::ValidatePermutation {
            candidate_permutation_path,
        } => {
            let mut algorithm = dispatcher.validate_permutation(
                ValidatePermutationInput {
                    candidate_permutation: load_permutation(candidate_permutation_path)?,
                },
                ValidatePermutationParameters {},
            );
            match algorithm.step_until(OutputStatus::FinalFullOutput) {
                Ok(()) => println!(
                    "Candidate permutation '{}' is valid",
                    candidate_permutation_path
                ),
                Err(e) => return Err(e),
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
