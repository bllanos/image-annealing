use crate::config::{AlgorithmConfig, Config};
use image_annealing::compute::format::ImageFileWriter;
use image_annealing::compute::{
    self, CreatePermutationInput, CreatePermutationParameters, Dispatcher, OutputStatus,
    PermuteInput, SwapInput, SwapParameters, SwapPassSelection, ValidatePermutationInput,
    ValidatePermutationParameters,
};
use std::error::Error;

mod loader;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let dispatcher = compute::create_dispatcher(&config.dispatcher)?;
    run_and_save(dispatcher, &config.algorithm)?;
    Ok(())
}

fn run_and_save(
    dispatcher: Box<dyn Dispatcher>,
    config: &AlgorithmConfig,
) -> Result<(), Box<dyn Error>> {
    match config {
        AlgorithmConfig::CreatePermutation {
            permutation_output_path_no_extension: path,
        } => {
            let mut algorithm = dispatcher
                .create_permutation(CreatePermutationInput {}, CreatePermutationParameters {});
            algorithm.step_until(OutputStatus::FinalFullOutput)?;
            let permutation = algorithm.full_output().unwrap().validated_permutation;
            let output_path = permutation.save_add_extension(&path.0)?;
            println!("Wrote permutation to: {}", output_path.display());
        }
        AlgorithmConfig::Permute {
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
                Default::default(),
            );
            algorithm.step_until(OutputStatus::FinalFullOutput)?;
            let img = algorithm.full_output().unwrap().permuted_image;
            let output_path = img.save_add_extension(path.to_vec().as_slice())?;
            println!("Wrote permuted image to: {:?}", output_path);
        }
        AlgorithmConfig::Swap {
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
            let output_path = permutation.save_add_extension(&path.0)?;
            println!("Wrote swapped permutation to: {}", output_path.display());
        }
        AlgorithmConfig::ValidatePermutation {
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
