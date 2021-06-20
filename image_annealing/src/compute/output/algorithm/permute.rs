use super::super::super::dispatch::{
    DimensionsMismatchError, DispatcherImplementation, PermuteOperationInput,
};
use super::super::super::resource::buffer::ReadMappableBuffer;
use super::super::super::resource::texture::{LosslessImageTexture, TextureDatatype};
use super::super::format::PermutationImageBuffer;
use super::super::format::{LosslessImageBuffer, LosslessImageBufferComponent};
use super::super::OutputStatus;
use super::validate_permutation::{
    ValidatePermutation, ValidatePermutationInput, ValidatePermutationParameters,
};
use super::CompletionStatus;
use crate::image_utils::validation::ValidatedPermutation;
use crate::image_utils::ImageDimensions;
use image::DynamicImage;
use std::convert::TryInto;
use std::error::Error;

pub struct PermuteParameters {}

pub struct PermuteInput {
    pub candidate_permutation: Option<PermutationImageBuffer>,
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
    invoked_operation: bool,
    permutation: Option<ValidatedPermutation>,
    image_output: Option<LosslessImageBuffer>,
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
            invoked_operation: false,
            permutation: None,
            image_output: None,
        }
    }

    pub fn step(
        &mut self,
        dispatcher: &mut DispatcherImplementation,
    ) -> Result<OutputStatus, Box<dyn Error>> {
        self.completion_status.ok_if_pending()?;
        debug_assert!(self.image_output.is_none());

        match self.validator.take() {
            Some(mut v) => {
                debug_assert!(!self.invoked_operation);
                debug_assert!(self.permutation.is_none());

                match v.step(dispatcher) {
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
                                self.permutation = v.full_output();
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
                if !self.invoked_operation {
                    let image_option = self.input.original_image.take();
                    if let Some(ref image) = image_option {
                        match ImageDimensions::from_image(image) {
                            Ok(dimensions) => {
                                if *dispatcher.image_dimensions() != dimensions {
                                    self.completion_status = CompletionStatus::Failed;
                                    return Err(Box::new(DimensionsMismatchError));
                                }
                            }
                            Err(e) => {
                                self.completion_status = CompletionStatus::Failed;
                                return Err(Box::new(e));
                            }
                        }
                    }

                    self.invoked_operation = true;
                    match dispatcher.operation_permute(&PermuteOperationInput {
                        permutation: self.permutation.as_ref(),
                        image: image_option.as_ref(),
                    }) {
                        Ok(_) => {
                            self.input.original_image = image_option;
                            Ok(OutputStatus::NoNewOutput)
                        }
                        Err(e) => {
                            self.completion_status = CompletionStatus::Failed;
                            Err(e)
                        }
                    }
                } else {
                    let mut mapped_buffer = dispatcher
                        .resources()
                        .lossless_image_output_buffer()
                        .request_map_read();

                    dispatcher.poll_device();

                    let buffer_dimensions = mapped_buffer.buffer_dimensions();
                    let data = mapped_buffer.collect_mapped_buffer();
                    let result: Vec<LosslessImageBufferComponent> = data
                        .chunks(buffer_dimensions.padded_bytes_per_row)
                        .flat_map(|c| {
                            c[..buffer_dimensions.unpadded_bytes_per_row].chunks_exact(
                                std::mem::size_of::<
                                    <LosslessImageTexture as TextureDatatype>::Component,
                                >(),
                            )
                        })
                        .map(|b| {
                            let val =
                                <LosslessImageTexture as TextureDatatype>::Component::from_ne_bytes(
                                    b.try_into().unwrap(),
                                );
                            val.try_into().unwrap_or(0)
                        })
                        .collect::<Vec<LosslessImageBufferComponent>>();

                    self.image_output = Some(
                        LosslessImageBuffer::from_vec(
                            buffer_dimensions.width as u32,
                            buffer_dimensions.height as u32,
                            result,
                        )
                        .unwrap(),
                    );
                    self.completion_status = CompletionStatus::Finished;
                    Ok(OutputStatus::FinalFullOutput)
                }
            }
        }
    }

    pub fn partial_output(&self) -> Option<()> {
        None
    }

    pub fn full_output(&mut self) -> Option<PermuteOutput> {
        match self.image_output.take() {
            Some(image) => Some(PermuteOutput {
                permutation: self.permutation.take(),
                original_image: self.input.original_image.take(),
                permuted_image: image,
            }),
            None => None,
        }
    }
}
