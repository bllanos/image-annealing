use super::loader;
use crate::config::{
    DisplacementGoalPath, IterationCount, PermutationPath, SwapParametersConfig, SwapStopConfig,
    SwapStopThreshold,
};
use image_annealing::compute::format::ImageFileWriter;
use image_annealing::compute::{
    Dispatcher, SwapAlgorithm, SwapInput, SwapParameters, SwapPartialOutput,
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
    )
    .collect::<Result<Vec<_>, _>>()?
    .pop()
    .unwrap();
    let output_path = permutation.save_add_extension(permutation_output_path_no_extension)?;
    println!("Wrote swapped permutation to: {}", output_path.display());
    Ok(())
}

struct SwapIter {
    algorithm_option: Option<Box<SwapAlgorithm>>,
    swap_parameters: SwapParameters,
    threshold: Option<SwapStopThreshold>,
    iteration_count: Option<IterationCount>,
    i: usize,
    finished: bool,
}

impl SwapIter {
    pub fn new(
        algorithm: Box<SwapAlgorithm>,
        swap_parameters: SwapParameters,
        threshold: Option<SwapStopThreshold>,
        iteration_count: Option<IterationCount>,
    ) -> Self {
        Self {
            algorithm_option: Some(algorithm),
            swap_parameters,
            threshold,
            iteration_count,
            i: 0,
            finished: false,
        }
    }
}

impl Iterator for SwapIter {
    type Item = Result<ValidatedPermutation, Box<dyn Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            None
        } else {
            let result = (|| -> Self::Item {
                loop {
                    let mut algorithm = self.algorithm_option.take().unwrap();
                    algorithm.step_until_finished()?;

                    let mut stop = false;

                    if let Some(count) = self.iteration_count {
                        if self.threshold.is_none() {
                            println!("Texel swap round {}", self.i);
                        }
                        if self.i == count.get().checked_sub(1).unwrap() {
                            stop = true;
                        }
                    }

                    if let Some(threshold_variant) = self.threshold {
                        let SwapPartialOutput {
                            counts: swap_counts,
                        } = algorithm.partial_output().unwrap();
                        println!("Texel swap round {}, {}", self.i, swap_counts);
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
                        self.algorithm_option = Some(algorithm);
                        break;
                    } else {
                        let dispatcher = algorithm.return_to_dispatcher();
                        self.algorithm_option =
                            Some(dispatcher.swap(Default::default(), &self.swap_parameters));
                        self.i += 1;
                    }
                }
                self.finished = true;
                Ok(self
                    .algorithm_option
                    .take()
                    .unwrap()
                    .full_output()
                    .unwrap()
                    .output_permutation)
            })();
            if result.is_err() {
                self.finished = true;
            }
            Some(result)
        }
    }
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
    SwapIter::new(algorithm, swap_parameters, threshold, iteration_count)
}

#[cfg(test)]
mod tests;
