use super::loader;
use crate::config::{
    DisplacementGoalPath, IterationCount, PermutationPath, SwapParametersConfig, SwapStopConfig,
    SwapStopThreshold,
};
use image_annealing::compute::format::ImageFileWriter;
use image_annealing::compute::{
    Dispatcher, OutputStatus, SwapAlgorithm, SwapInput, SwapParameters, SwapPartialOutput, SwapPass,
};
use image_annealing::{CandidatePermutation, DisplacementGoal, ValidatedPermutation};
use std::error::Error;

pub fn run_and_save_swap(
    dispatcher: Box<dyn Dispatcher>,
    candidate_permutation: &PermutationPath,
    displacement_goal: &DisplacementGoalPath,
    permutation_output_path_prefix: &PermutationPath,
    parameters: &SwapParametersConfig,
) -> Result<(), Box<dyn Error>> {
    let mut output_path = None;
    for result in run_swap(
        dispatcher,
        Some(loader::load_candidate_permutation(candidate_permutation)?),
        Some(loader::load_displacement_goal(displacement_goal)?),
        parameters,
    ) {
        let tagged_permutation = result?;
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

struct SwapIter {
    algorithm_option: Option<Box<SwapAlgorithm>>,
    swap_parameters: SwapParameters,
    threshold: Option<SwapStopThreshold>,
    iteration_count: Option<IterationCount>,
    output_intermediate_permutations: bool,
    last_pass: SwapPass,
    round_index: usize,
    pass_index: usize,
    finished: bool,
}

impl SwapIter {
    pub fn new(
        algorithm: Box<SwapAlgorithm>,
        swap_parameters: SwapParameters,
        threshold: Option<SwapStopThreshold>,
        iteration_count: Option<IterationCount>,
        output_intermediate_permutations: bool,
    ) -> Self {
        let last_pass = *swap_parameters.sequence.iter().last().unwrap();
        Self {
            algorithm_option: Some(algorithm),
            swap_parameters,
            threshold,
            iteration_count,
            output_intermediate_permutations,
            last_pass,
            round_index: 0,
            pass_index: 0,
            finished: false,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct TaggedPermutation {
    pub permutation: ValidatedPermutation,
    pub round_index: usize,
    pub pass_index: usize,
    pub pass: SwapPass,
}

impl Iterator for SwapIter {
    type Item = Result<TaggedPermutation, Box<dyn Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            None
        } else {
            let result = (|| -> Self::Item {
                let mut output = None;
                loop {
                    let mut algorithm = self.algorithm_option.take().unwrap();
                    let mut status = OutputStatus::NoNewOutput;
                    if self.output_intermediate_permutations {
                        while !status.is_full() && !status.is_final() {
                            status = algorithm.step()?;
                        }
                        let full_output = algorithm.full_output().unwrap();
                        if !status.is_final() {
                            if full_output.pass == self.last_pass {
                                status = algorithm.step_until_finished()?
                            }
                        }
                        output = Some(TaggedPermutation {
                            permutation: full_output.output_permutation,
                            round_index: self.round_index,
                            pass_index: self.pass_index,
                            pass: full_output.pass,
                        });
                        self.pass_index += 1;
                    } else {
                        while !status.is_final() {
                            status = algorithm.step()?;
                            if status.is_full() {
                                self.pass_index += 1;
                            }
                        }
                        self.pass_index = self.pass_index.checked_sub(1).unwrap();
                    };

                    if status.is_final() {
                        let mut stop = false;

                        if let Some(count) = self.iteration_count {
                            if self.threshold.is_none() {
                                println!("Texel swap round {}", self.round_index);
                            }
                            if self.round_index == count.get().checked_sub(1).unwrap() {
                                stop = true;
                            }
                        }

                        if let Some(threshold_variant) = self.threshold {
                            let SwapPartialOutput {
                                counts: swap_counts,
                            } = algorithm.partial_output().unwrap();
                            println!("Texel swap round {}, {}", self.round_index, swap_counts);
                            if !stop {
                                match threshold_variant {
                                    SwapStopThreshold::SwapsAccepted(number_of_swaps) => {
                                        if swap_counts.accepted() <= number_of_swaps {
                                            stop = true;
                                        }
                                    }
                                    SwapStopThreshold::SwapAcceptanceFraction(
                                        fraction_of_swaps,
                                    ) => {
                                        if swap_counts.accepted_fraction()
                                            <= fraction_of_swaps.get()
                                        {
                                            stop = true;
                                        }
                                    }
                                }
                            }
                        }

                        if stop {
                            self.finished = true;
                            output.get_or_insert_with(|| {
                                let full_output = algorithm.full_output().unwrap();
                                TaggedPermutation {
                                    permutation: full_output.output_permutation,
                                    round_index: self.round_index,
                                    pass_index: self.pass_index,
                                    pass: full_output.pass,
                                }
                            });
                            break;
                        } else {
                            let dispatcher = algorithm.return_to_dispatcher();
                            self.algorithm_option =
                                Some(dispatcher.swap(Default::default(), &self.swap_parameters));
                            self.round_index += 1;
                        }
                        self.pass_index = 0;
                    } else {
                        self.algorithm_option = Some(algorithm);
                    }
                    if self.output_intermediate_permutations {
                        break;
                    }
                }
                Ok(output.unwrap())
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
