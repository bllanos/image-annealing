use super::super::super::system::{DevicePollType, PermuteOperationInput, System};
use super::super::format::{ImageFormat, ImageFormatError, LosslessImage};
use super::super::OutputStatus;
use super::validate_permutation::{ValidatePermutation, ValidatePermutationInput};
use super::{CompletionStatus, CompletionStatusHolder, FinalOutputHolder};
use crate::image_utils::check_dimensions_match2;
use crate::{CandidatePermutation, ValidatedPermutation};
use async_trait::async_trait;
use std::default::Default;
use std::error::Error;

#[derive(Default)]
pub struct PermuteParameters {
    pub permuted_image_format: Option<ImageFormat>,
}

#[derive(Default)]
pub struct PermuteInput {
    pub candidate_permutation: Option<CandidatePermutation>,
    pub original_image: Option<LosslessImage>,
}

pub struct PermuteOutput {
    pub permutation: Option<ValidatedPermutation>,
    pub original_image: Option<LosslessImage>,
    pub permuted_image: LosslessImage,
}

pub struct Permute {
    completion_status: CompletionStatus,
    input: PermuteInput,
    validator: Option<ValidatePermutation>,
    permutation: Option<ValidatedPermutation>,
    permuted_image_format: Option<ImageFormat>,
    has_given_output: bool,
}

impl Permute {
    pub fn new(mut input: PermuteInput, parameters: &PermuteParameters) -> Self {
        let validator = input.candidate_permutation.take().map(|permutation| {
            ValidatePermutation::new(
                ValidatePermutationInput {
                    candidate_permutation: permutation,
                },
                &Default::default(),
            )
        });
        Self {
            completion_status: CompletionStatus::new(),
            input,
            validator,
            permutation: None,
            permuted_image_format: parameters.permuted_image_format,
            has_given_output: false,
        }
    }

    pub fn step(&mut self, system: &mut System) -> Result<OutputStatus, Box<dyn Error>> {
        self.checked_step(system)
    }

    pub async fn partial_output(&self, _poll_type: DevicePollType) -> Option<()> {
        None
    }

    pub async fn full_output(
        &mut self,
        system: &mut System,
        poll_type: DevicePollType,
    ) -> Option<PermuteOutput> {
        self.checked_output(system, poll_type).await
    }
}

impl CompletionStatusHolder for Permute {
    fn get_status(&self) -> &CompletionStatus {
        &self.completion_status
    }

    fn set_status(&mut self, status: CompletionStatus) {
        self.completion_status = status;
    }

    fn unchecked_step(&mut self, system: &mut System) -> Result<OutputStatus, Box<dyn Error>> {
        match self.validator.as_mut() {
            Some(v) => {
                debug_assert!(self.permutation.is_none());

                let status = v.step(system)?;
                if status.is_final() && status.is_full() {
                    self.permutation = v.full_output().map(|output| output.validated_permutation);
                    self.validator = None;
                }
                Ok(OutputStatus::NoNewOutput)
            }
            None => {
                let image_option = self.input.original_image.as_ref();
                match image_option {
                    Some(image) => {
                        check_dimensions_match2(system, image)?;
                        let input_format = image.format();
                        match self.permuted_image_format {
                            Some(format) => {
                                if format != input_format {
                                    return Err(Box::new(ImageFormatError::Mismatch {
                                        image_name: String::from("permutation"),
                                        input_format,
                                        output_format: format,
                                    }));
                                }
                            }
                            None => self.permuted_image_format = Some(input_format),
                        }
                    }
                    None => {
                        if self.permuted_image_format.is_none() {
                            return Err(Box::new(ImageFormatError::Missing {
                                image_name: String::from("permuted output"),
                            }));
                        }
                    }
                }

                system.operation_permute(&PermuteOperationInput {
                    permutation: self.permutation.as_ref(),
                    image: image_option,
                })?;
                self.completion_status = CompletionStatus::Finished;
                Ok(OutputStatus::FinalFullOutput)
            }
        }
    }
}

#[async_trait]
impl FinalOutputHolder<PermuteOutput> for Permute {
    fn has_given_output(&self) -> bool {
        self.has_given_output
    }
    fn set_has_given_output(&mut self) {
        self.has_given_output = true;
    }

    async fn unchecked_output(
        &mut self,
        system: &mut System,
        poll_type: DevicePollType,
    ) -> Option<PermuteOutput> {
        system
            .output_permuted_image(poll_type, self.permuted_image_format.unwrap())
            .await
            .ok()
            .map(|permuted_image| PermuteOutput {
                permutation: self.permutation.take(),
                original_image: self.input.original_image.take(),
                permuted_image,
            })
    }
}
