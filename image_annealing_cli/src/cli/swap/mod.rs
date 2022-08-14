use super::loader;
use crate::config::{DisplacementGoalPath, PermutationPath, SwapParametersConfig, SwapStopConfig};
use image_annealing::compute::format::ImageFileWriter;
use image_annealing::compute::{Dispatcher, SwapInput, SwapParameters};
use image_annealing::{CandidatePermutation, DisplacementGoal};
use std::error::Error;

mod iter;
use iter::SwapIter;

pub fn run_and_save_swap(
    dispatcher: Box<dyn Dispatcher>,
    candidate_permutation: &PermutationPath,
    displacement_goal: &DisplacementGoalPath,
    permutation_output_path_prefix: &PermutationPath,
    parameters: &SwapParametersConfig,
) -> Result<(), Box<dyn Error>> {
    let mut output_path = None;
    let mut iter = run_swap(
        dispatcher,
        Some(loader::load_candidate_permutation(candidate_permutation)?),
        Some(loader::load_displacement_goal(displacement_goal)?),
        parameters,
    );
    while let Some(result) = iter.next() {
        let tagged_permutation = futures::executor::block_on(result)?;
        let path_no_extension = format!(
            "{}_round_{}_pass_{}_{}",
            permutation_output_path_prefix,
            tagged_permutation.round_index,
            tagged_permutation.pass_index,
            tagged_permutation.pass.snake_case_name()
        );
        output_path = Some(
            tagged_permutation
                .permutation
                .save_add_extension(path_no_extension)?,
        );
    }
    output_path
        .iter()
        .for_each(|path| println!("Wrote final swapped permutation to: {}", path.display()));
    Ok(())
}

fn run_swap(
    dispatcher: Box<dyn Dispatcher>,
    candidate_permutation: Option<CandidatePermutation>,
    displacement_goal: Option<DisplacementGoal>,
    parameters: &SwapParametersConfig,
) -> SwapIter {
    let (swap_parameters, threshold) = match parameters.stop {
        SwapStopConfig::Bounded {
            threshold: Some(threshold_variant),
            ..
        }
        | SwapStopConfig::Unbounded(threshold_variant) => (
            SwapParameters {
                sequence: parameters.swap_pass_sequence,
                swap_acceptance_threshold: parameters.swap_acceptance_threshold,
                count_swap: true,
            },
            Some(threshold_variant),
        ),
        _ => (
            SwapParameters::from_sequence_and_threshold(
                parameters.swap_pass_sequence,
                parameters.swap_acceptance_threshold,
            ),
            None,
        ),
    };
    let iteration_count = match parameters.stop {
        SwapStopConfig::Bounded {
            iteration_count: count,
            ..
        } => Some(count),
        SwapStopConfig::Unbounded(_) => None,
    };

    let algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation,
            displacement_goal,
        },
        &swap_parameters,
    );
    SwapIter::new(
        algorithm,
        swap_parameters,
        threshold,
        iteration_count,
        parameters.output_intermediate_permutations,
    )
}

#[cfg(test)]
mod tests;
