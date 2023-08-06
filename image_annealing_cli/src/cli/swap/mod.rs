use super::loader;
use crate::config::{InputDisplacementGoalPath, InputPermutationPath, OutputPermutationPath, SwapParametersConfig, SwapStopConfig};
use futures::join;
use image_annealing::compute::{Dispatcher, SwapInput, SwapParameters};
use image_annealing::{CandidatePermutation, DisplacementGoal};
use std::error::Error;

mod iter;
mod output;

use iter::{SwapIter, TaggedPermutation};
use output::TaggedPermutationWriter;

pub fn run_and_save_swap(
    dispatcher: Box<dyn Dispatcher>,
    candidate_permutation: &InputPermutationPath,
    displacement_goal: &InputDisplacementGoalPath,
    permutation_output_path_prefix: &OutputPermutationPath,
    parameters: &SwapParametersConfig,
) -> Result<(), Box<dyn Error>> {
    let mut iter = run_swap(
        dispatcher,
        Some(loader::load_candidate_permutation(candidate_permutation)?),
        Some(loader::load_displacement_goal(displacement_goal)?),
        parameters,
    );
    let writer = TaggedPermutationWriter::new(permutation_output_path_prefix);
    let mut output_permutation: Option<TaggedPermutation> = None;

    while let Some(result) = iter.next() {
        output_permutation = Some(match output_permutation.take() {
            Some(permutation) => {
                let join_result = futures::executor::block_on(async {
                    // `join!` is used instead of `try_join!` because the previous permutation
                    // should be saved even if there is an error creating the next permutation.
                    join!(writer.save(permutation), result)
                });
                join_result.0?;
                join_result.1
            }
            None => futures::executor::block_on(result),
        }?)
    }
    if let Some(permutation) = output_permutation {
        let path = futures::executor::block_on(writer.save(permutation))?;
        println!("Wrote final swapped permutation to: {}", path.display());
    }
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
