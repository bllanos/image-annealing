use crate::config::{IterationCount, SwapStopThreshold};
use image_annealing::compute::{
    OutputStatus, SwapAlgorithm, SwapParameters, SwapPartialOutput, SwapPass,
};
use image_annealing::ValidatedPermutation;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Eq, PartialEq)]
pub struct TaggedPermutation {
    pub permutation: ValidatedPermutation,
    pub round_index: usize,
    pub pass_index: usize,
    pub pass: SwapPass,
}

type TaggedPermutationResult = Result<TaggedPermutation, Box<dyn Error>>;

// Ideally we would use this type in `Iterator::Item`,
// but generic associated types are not yet stable.
// See https://github.com/rust-lang/rust/issues/44265
type Item<'a> = Pin<Box<dyn Future<Output = TaggedPermutationResult> + Send + 'a>>;

pub struct SwapIter {
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

impl<'a> SwapIter {
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

    async fn generate_item(&mut self) -> TaggedPermutationResult {
        let mut output = None;
        loop {
            let mut algorithm = self.algorithm_option.take().unwrap();
            let mut status = OutputStatus::NoNewOutput;
            if self.output_intermediate_permutations {
                while !status.is_full() && !status.is_final() {
                    status = algorithm.step()?;
                }
                let full_output = algorithm.full_output().await.unwrap();
                if !status.is_final() && full_output.pass == self.last_pass {
                    status = algorithm.step_until_finished()?
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
                    } = algorithm.partial_output().await.unwrap();
                    println!("Texel swap round {}, {}", self.round_index, swap_counts);
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
                    self.finished = true;
                    if output.is_none() {
                        let full_output = algorithm.full_output().await.unwrap();
                        output = Some(TaggedPermutation {
                            permutation: full_output.output_permutation,
                            round_index: self.round_index,
                            pass_index: self.pass_index,
                            pass: full_output.pass,
                        })
                    }
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
    }

    pub fn next(&'a mut self) -> Option<Item<'a>> {
        if self.finished {
            None
        } else {
            Some(Box::pin(async {
                let result = self.generate_item().await;
                if result.is_err() {
                    self.finished = true;
                }
                result
            }))
        }
    }
}
