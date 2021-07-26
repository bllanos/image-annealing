use super::super::super::resource::buffer::ReadMappableBuffer;
use super::super::super::system::System;
use super::super::format::{PermutationImageBuffer, PermutationImageBufferComponent};
use super::super::OutputStatus;
use super::CompletionStatus;
use std::convert::TryInto;
use std::error::Error;

pub struct CreatePermutationParameters {}

pub struct CreatePermutationInput {}

pub struct CreatePermutation {
    completion_status: CompletionStatus,
    invoked_operation: bool,
    full_output: Option<PermutationImageBuffer>,
}

impl CreatePermutation {
    pub fn new(_input: CreatePermutationInput, _parameters: CreatePermutationParameters) -> Self {
        Self {
            completion_status: CompletionStatus::new(),
            invoked_operation: false,
            full_output: None,
        }
    }

    pub fn step(&mut self, system: &mut System) -> Result<OutputStatus, Box<dyn Error>> {
        self.completion_status.ok_if_pending()?;
        debug_assert!(self.full_output.is_none());
        if !self.invoked_operation {
            self.invoked_operation = true;
            match system.operation_create_permutation() {
                Ok(_) => Ok(OutputStatus::NoNewOutput),
                Err(e) => {
                    self.completion_status = CompletionStatus::Failed;
                    Err(e)
                }
            }
        } else {
            let mut mapped_buffer = system
                .resources()
                .permutation_output_buffer()
                .request_map_read();

            system.poll_device();

            let buffer_dimensions = mapped_buffer.buffer_dimensions();
            let data = mapped_buffer.collect_mapped_buffer();
            let result: Vec<PermutationImageBufferComponent> = data
                .chunks(buffer_dimensions.padded_bytes_per_row)
                .flat_map(|c| {
                    c[..buffer_dimensions.unpadded_bytes_per_row]
                        .chunks_exact(std::mem::size_of::<PermutationImageBufferComponent>())
                })
                .map(|b| PermutationImageBufferComponent::from_be_bytes(b.try_into().unwrap()))
                .collect::<Vec<PermutationImageBufferComponent>>();

            self.full_output = Some(
                PermutationImageBuffer::from_vec(
                    buffer_dimensions.width.try_into().unwrap(),
                    buffer_dimensions.height.try_into().unwrap(),
                    result,
                )
                .unwrap(),
            );
            self.completion_status = CompletionStatus::Finished;
            Ok(OutputStatus::FinalFullOutput)
        }
    }

    pub fn partial_output(&self) -> Option<()> {
        None
    }

    pub fn full_output(&mut self) -> Option<PermutationImageBuffer> {
        self.full_output.take()
    }
}
