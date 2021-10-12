use super::super::super::system::{DimensionsMismatchError, PermuteOperationInput, System};
use super::super::format::LosslessImageBuffer;
use super::super::OutputStatus;
use super::validate_permutation::{
    ValidatePermutation, ValidatePermutationInput, ValidatePermutationParameters,
};
use super::CompletionStatus;
use crate::image_utils::validation::{CandidatePermutation, ValidatedPermutation};
use crate::image_utils::ImageDimensions;
use image::DynamicImage;
use std::default::Default;
use std::error::Error;

pub struct PermuteParameters {}

#[derive(Default)]
pub struct PermuteInput {
    pub candidate_permutation: Option<CandidatePermutation>,
    pub original_image: Option<DynamicImage>,
}

pub struct PermuteOutput {
    pub permutation: Option<ValidatedPermutation>,
    pub original_image: Option<DynamicImage>,
    pub permuted_image: LosslessImageBuffer,
}

pub struct Permute {
    completion_status: CompletionStatus,
    input: PermuteInput,
    validator: Option<ValidatePermutation>,
    permutation: Option<ValidatedPermutation>,
    has_given_output: bool,
}

impl Permute {
    pub fn new(mut input: PermuteInput, _parameters: PermuteParameters) -> Self {
        let validator = input.candidate_permutation.take().map(|permutation| {
            ValidatePermutation::new(
                ValidatePermutationInput {
                    candidate_permutation: permutation,
                },
                ValidatePermutationParameters {},
            )
        });
        Self {
            completion_status: CompletionStatus::new(),
            input,
            validator,
            permutation: None,
            has_given_output: false,
        }
    }

    pub fn step(&mut self, system: &mut System) -> Result<OutputStatus, Box<dyn Error>> {
        self.completion_status.ok_if_pending()?;
        match self.validator.take() {
            Some(mut v) => {
                debug_assert!(self.permutation.is_none());

                match v.step(system) {
                    Ok(status) => {
                        match status {
                            OutputStatus::NoNewOutput
                            | OutputStatus::NewPartialOutput
                            | OutputStatus::NewFullOutput
                            | OutputStatus::NewPartialAndFullOutput
                            | OutputStatus::FinalPartialOutput => {
                                self.validator = Some(v);
                            }
                            OutputStatus::FinalFullOutput
                            | OutputStatus::FinalPartialAndFullOutput => {
                                self.permutation =
                                    v.full_output().map(|output| output.validated_permutation);
                            }
                        }
                        Ok(OutputStatus::NoNewOutput)
                    }
                    Err(e) => {
                        self.completion_status = CompletionStatus::Failed;
                        Err(e)
                    }
                }
            }
            None => {
                let image_option = self.input.original_image.take();
                if let Some(ref image) = image_option {
                    match ImageDimensions::from_image(image) {
                        Ok(dimensions) => {
                            if *system.image_dimensions() != dimensions {
                                self.completion_status = CompletionStatus::Failed;
                                return Err(Box::new(DimensionsMismatchError::new(
                                    *system.image_dimensions(),
                                    dimensions,
                                )));
                            }
                        }
                        Err(e) => {
                            self.completion_status = CompletionStatus::Failed;
                            return Err(Box::new(e));
                        }
                    }
                }

                match system.operation_permute(&PermuteOperationInput {
                    permutation: self.permutation.as_ref(),
                    image: image_option.as_ref(),
                }) {
                    Ok(_) => {
                        self.input.original_image = image_option;
                        self.completion_status = CompletionStatus::Finished;
                        Ok(OutputStatus::FinalFullOutput)
                    }
                    Err(e) => {
                        self.completion_status = CompletionStatus::Failed;
                        Err(e)
                    }
                }
            }
        }
    }

    pub fn partial_output(&self) -> Option<()> {
        None
    }

    pub fn full_output(&mut self, system: &mut System) -> Option<PermuteOutput> {
        if self.has_given_output {
            None
        } else {
            match self.completion_status {
                CompletionStatus::Finished => {
                    self.has_given_output = true;
                    system
                        .output_permuted_image()
                        .ok()
                        .map(|permuted_image| PermuteOutput {
                            permutation: self.permutation.take(),
                            original_image: self.input.original_image.take(),
                            permuted_image,
                        })
                }
                _ => None,
            }
        }
    }
}
