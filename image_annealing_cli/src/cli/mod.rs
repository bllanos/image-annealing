use crate::config::Config;
use image_annealing::compute::format::ImageFileWriter;
use image_annealing::compute::{
    self, CreatePermutationInput, CreatePermutationParameters, Dispatcher, OutputStatus,
    PermuteInput, PermuteParameters, SwapInput, SwapParameters, SwapPassSelection,
    ValidatePermutationInput, ValidatePermutationParameters,
};
use image_annealing::ImageDimensions;
use std::error::Error;

mod loader;

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
        Config::Permute { original_image, .. } => {
            ImageDimensions::from_image_path(&original_image.0)?
        }
        Config::Swap {
            candidate_permutation,
            ..
        } => ImageDimensions::from_image_path(&candidate_permutation.0)?,
        Config::ValidatePermutation {
            candidate_permutation,
        } => ImageDimensions::from_image_path(&candidate_permutation.0)?,
    };
    compute::create_dispatcher(&dimensions)
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
            let permutation = algorithm.full_output().unwrap().validated_permutation;
            let output_path = permutation.as_ref().save_add_extension(path)?;
            println!("Wrote permutation to: {}", output_path.display());
        }
        Config::Permute {
            candidate_permutation,
            original_image,
            permuted_image_output_path_no_extension: path,
        } => {
            let mut algorithm = dispatcher.permute(
                PermuteInput {
                    candidate_permutation: Some(loader::load_candidate_permutation(
                        candidate_permutation,
                    )?),
                    original_image: Some(loader::load_image(original_image)?),
                },
                PermuteParameters {},
            );
            algorithm.step_until(OutputStatus::FinalFullOutput)?;
            let img = algorithm.full_output().unwrap().permuted_image;
            let output_path = img.save_add_extension(path)?;
            println!("Wrote permuted image to: {}", output_path.display());
        }
        Config::Swap {
            candidate_permutation,
            displacement_goal,
            permutation_output_path_no_extension: path,
        } => {
            let mut algorithm = dispatcher.swap(
                SwapInput {
                    candidate_permutation: Some(loader::load_candidate_permutation(
                        candidate_permutation,
                    )?),
                    displacement_goal: Some(loader::load_displacement_goal(displacement_goal)?),
                },
                SwapParameters::from_selection(SwapPassSelection::HORIZONTAL)?,
            );
            algorithm.step_until(OutputStatus::FinalFullOutput)?;
            let permutation = algorithm.full_output().unwrap().output_permutation;
            let output_path = permutation.as_ref().save_add_extension(path)?;
            println!("Wrote swapped permutation to: {}", output_path.display());
        }
        Config::ValidatePermutation {
            candidate_permutation,
        } => {
            let mut algorithm = dispatcher.validate_permutation(
                ValidatePermutationInput {
                    candidate_permutation: loader::load_candidate_permutation(
                        candidate_permutation,
                    )?,
                },
                ValidatePermutationParameters {},
            );
            algorithm.step_until(OutputStatus::FinalFullOutput)?;
            println!(
                "Candidate permutation '{}' is valid",
                candidate_permutation.0
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
