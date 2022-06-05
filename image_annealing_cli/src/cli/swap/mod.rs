use super::loader;
use crate::config::{
    DisplacementGoalPath, PermutationPath, SwapParametersConfig, SwapStopConfig, SwapStopThreshold,
};
use image_annealing::compute::format::ImageFileWriter;
use image_annealing::compute::{
    Dispatcher, SwapInput, SwapParameters, SwapPartialOutput, SwapPassSelection,
};
use image_annealing::{CandidatePermutation, DisplacementGoal, ValidatedPermutation};
use std::error::Error;

pub fn run_and_save_swap(
    dispatcher: Box<dyn Dispatcher>,
    candidate_permutation: &PermutationPath,
    displacement_goal: &DisplacementGoalPath,
    permutation_output_path_no_extension: &PermutationPath,
    parameters: &SwapParametersConfig,
) -> Result<(), Box<dyn Error>> {
    let permutation = run_swap(
        dispatcher,
        Some(loader::load_candidate_permutation(candidate_permutation)?),
        Some(loader::load_displacement_goal(displacement_goal)?),
        parameters,
    )?;
    let output_path = permutation.save_add_extension(permutation_output_path_no_extension)?;
    println!("Wrote swapped permutation to: {}", output_path.display());
    Ok(())
}

fn run_swap(
    dispatcher: Box<dyn Dispatcher>,
    candidate_permutation: Option<CandidatePermutation>,
    displacement_goal: Option<DisplacementGoal>,
    parameters: &SwapParametersConfig,
) -> Result<ValidatedPermutation, Box<dyn Error>> {
    // TODO Input the SwapPassSelection from the SwapParametersConfig
    let swap_pass_selection = SwapPassSelection::HORIZONTAL;
    let (swap_parameters, threshold) = match parameters.stop {
        SwapStopConfig::Bounded {
            threshold: Some(threshold_variant),
            ..
        }
        | SwapStopConfig::Unbounded(threshold_variant) => (
            SwapParameters::new(
                swap_pass_selection,
                parameters.swap_acceptance_threshold,
                true,
            )?,
            Some(threshold_variant),
        ),
        _ => (
            SwapParameters::from_selection_and_threshold(
                swap_pass_selection,
                parameters.swap_acceptance_threshold,
            )?,
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

    let mut i = 0;
    let mut algorithm_option = Some(dispatcher.swap(
        SwapInput {
            candidate_permutation,
            displacement_goal,
        },
        &swap_parameters,
    ));
    loop {
        let mut algorithm = algorithm_option.take().unwrap();
        algorithm.step_until_finished()?;

        let mut stop = false;

        if let Some(count) = iteration_count {
            if threshold.is_none() {
                println!("Texel swap round {}", i);
            }
            if i == count.get().checked_sub(1).unwrap() {
                stop = true;
            }
        }

        if let Some(threshold_variant) = threshold {
            let SwapPartialOutput {
                counts: swap_counts,
            } = algorithm.partial_output().unwrap();
            println!("Texel swap round {}, {}", i, swap_counts);
            if !stop {
                match threshold_variant {
                    SwapStopThreshold::SwapsAccepted(number_of_swaps) => {
                        if swap_counts.accepted() <= number_of_swaps {
                            stop = true;
                        }
                    }
                    SwapStopThreshold::SwapAcceptanceFraction(fraction_of_swaps) => {
                        if swap_counts.accepted_fraction() <= fraction_of_swaps.get() {
                            stop = true;
                        }
                    }
                }
            }
        }

        if stop {
            algorithm_option = Some(algorithm);
            break;
        } else {
            let dispatcher = algorithm.return_to_dispatcher();
            algorithm_option = Some(dispatcher.swap(Default::default(), &swap_parameters));
            i += 1;
        }
    }
    Ok(algorithm_option
        .unwrap()
        .full_output()
        .unwrap()
        .output_permutation)
}

#[cfg(test)]
mod tests;
