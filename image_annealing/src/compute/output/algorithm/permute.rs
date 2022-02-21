use super::super::super::system::{DimensionsMismatchError, PermuteOperationInput, System};
use super::super::format::LosslessImageBuffer;
use super::super::OutputStatus;
use super::validate_permutation::{
    ValidatePermutation, ValidatePermutationInput, ValidatePermutationParameters,
};
use super::{CompletionStatus, CompletionStatusHolder, FinalFullOutputHolder};
use crate::{CandidatePermutation, ImageDimensions, ValidatedPermutation};
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
        self.checked_step(system)
    }

    pub fn partial_output(&self) -> Option<()> {
        None
    }

    pub fn full_output(&mut self, system: &mut System) -> Option<PermuteOutput> {
        self.checked_full_output(system)
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
        match self.validator.take() {
            Some(mut v) => {
                debug_assert!(self.permutation.is_none());

                let status = v.step(system)?;
                match status {
                    OutputStatus::NoNewOutput
                    | OutputStatus::NewPartialOutput
                    | OutputStatus::NewFullOutput
                    | OutputStatus::NewPartialAndFullOutput
                    | OutputStatus::FinalPartialOutput => {
                        self.validator = Some(v);
                    }
                    OutputStatus::FinalFullOutput | OutputStatus::FinalPartialAndFullOutput => {
                        self.permutation =
                            v.full_output().map(|output| output.validated_permutation);
                    }
                }
                Ok(OutputStatus::NoNewOutput)
            }
            None => {
                let image_option = self.input.original_image.take();
                if let Some(ref image) = image_option {
                    let dimensions = ImageDimensions::from_image(image)?;
                    if *system.image_dimensions() != dimensions {
                        return Err(Box::new(DimensionsMismatchError::new(
                            *system.image_dimensions(),
                            dimensions,
                        )));
                    }
                }

                system.operation_permute(&PermuteOperationInput {
                    permutation: self.permutation.as_ref(),
                    image: image_option.as_ref(),
                })?;
                self.input.original_image = image_option;
                self.completion_status = CompletionStatus::Finished;
                Ok(OutputStatus::FinalFullOutput)
            }
        }
    }
}

impl FinalFullOutputHolder<PermuteOutput> for Permute {
    fn has_given_output(&self) -> bool {
        self.has_given_output
    }
    fn set_has_given_output(&mut self) {
        self.has_given_output = true;
    }

    fn unchecked_full_output(&mut self, system: &mut System) -> Option<PermuteOutput> {
        system
            .output_permuted_image()
            .ok()
            .map(|permuted_image| PermuteOutput {
                permutation: self.permutation.take(),
                original_image: self.input.original_image.take(),
                permuted_image,
            })
    }
}
