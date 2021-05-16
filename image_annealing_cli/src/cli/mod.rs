use crate::config::Config;
use image::io::Reader as ImageReader;
use image_annealing::compute::format::ImageFileWriter;
use image_annealing::compute::{
    self, CreatePermutationInput, CreatePermutationParameters, Dispatcher, OutputStatus,
    ValidatePermutationInput, ValidatePermutationParameters,
};
use image_annealing::image_utils::ImageDimensions;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let dispatcher = create_dispatcher(&config)?;
    run_and_save(dispatcher, &config)?;
    Ok(())
}

fn create_dispatcher(config: &Config) -> Result<Box<dyn Dispatcher>, Box<dyn Error>> {
    let dimensions = match config {
        Config::CreatePermutationConfig {
            image_dimensions, ..
        } => *image_dimensions,
        Config::ValidatePermutationConfig {
            candidate_permutation_path,
        } => {
            let (width, height) = image::image_dimensions(&candidate_permutation_path)?;
            ImageDimensions::new(width, height)?
        }
    };
    compute::create_dispatcher(&dimensions)
}

fn run_and_save(dispatcher: Box<dyn Dispatcher>, config: &Config) -> Result<(), Box<dyn Error>> {
    match config {
        Config::CreatePermutationConfig {
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
        Config::ValidatePermutationConfig {
            candidate_permutation_path,
        } => {
            let candidate_permutation = ImageReader::open(candidate_permutation_path)?
                .decode()?
                .to_rgba8();
            let mut algorithm = dispatcher.validate_permutation(
                ValidatePermutationInput {
                    candidate_permutation,
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
