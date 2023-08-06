use crate::config::{AlgorithmConfig, Config, CreateDisplacementGoalInputConfig};
use image_annealing::compute::format::ImageFileWriter;
use image_annealing::compute::{
    self, CreateDisplacementGoalInput, Dispatcher, PermuteInput, ValidatePermutationInput,
};
use std::error::Error;

mod loader;
mod swap;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let dispatcher = compute::create_dispatcher_block(&config.dispatcher)?;
    run_and_save(dispatcher, &config.algorithm)?;
    Ok(())
}

fn run_and_save(
    dispatcher: Box<dyn Dispatcher>,
    config: &AlgorithmConfig,
) -> Result<(), Box<dyn Error>> {
    match config {
        AlgorithmConfig::CreateDisplacementGoal {
            input:
                CreateDisplacementGoalInputConfig {
                    displacement_goal,
                    candidate_permutation,
                    image,
                },
            displacement_goal_output_path_no_extension: path,
        } => {
            let mut algorithm = dispatcher.create_displacement_goal(
                CreateDisplacementGoalInput {
                    displacement_goal: displacement_goal
                        .as_ref()
                        .map(loader::load_displacement_goal)
                        .transpose()?,
                    candidate_permutation: candidate_permutation
                        .as_ref()
                        .map(loader::load_candidate_permutation)
                        .transpose()?,
                    image: image.as_ref().map(loader::load_image).transpose()?,
                },
                &Default::default(),
            );
            algorithm.step_until_finished()?;
            let displacement_goal = algorithm
                .full_output_block()
                .unwrap()
                .output_displacement_goal;
            let output_path = displacement_goal.save_add_extension(&path.0 .0)?;
            println!("Wrote displacement goal to: {}", output_path.display());
        }
        AlgorithmConfig::CreatePermutation {
            permutation_output_path_no_extension: path,
        } => {
            let mut algorithm =
                dispatcher.create_permutation(Default::default(), &Default::default());
            algorithm.step_until_finished()?;
            let permutation = algorithm.full_output_block().unwrap().validated_permutation;
            let output_path = permutation.save_add_extension(&path.0 .0)?;
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
                &Default::default(),
            );
            algorithm.step_until_finished()?;
            let img = algorithm.full_output_block().unwrap().permuted_image;
            let output_path = img.save_add_extension(path.as_vec().as_slice())?;
            println!("Wrote permuted image to: {:?}", output_path);
        }
        AlgorithmConfig::Swap {
            candidate_permutation,
            displacement_goal,
            permutation_output_path_prefix,
            parameters,
        } => swap::run_and_save_swap(
            dispatcher,
            candidate_permutation,
            displacement_goal,
            permutation_output_path_prefix,
            parameters,
        )?,
        AlgorithmConfig::ValidatePermutation {
            candidate_permutation,
        } => {
            let mut algorithm = dispatcher.validate_permutation(
                ValidatePermutationInput {
                    candidate_permutation: loader::load_candidate_permutation(
                        candidate_permutation,
                    )?,
                },
                &Default::default(),
            );
            algorithm.step_until_finished()?;
            println!("Candidate permutation '{}' is valid", candidate_permutation);
        }
    }
    Ok(())
}
