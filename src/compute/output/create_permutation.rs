use super::super::dispatch::DispatcherImplementation;
use super::super::resource::buffer::ReadMappableBuffer;
use super::format::{PermutationImageBuffer, PermutationImageBufferComponent};
use super::OutputStatus;
use std::convert::TryInto;
use std::error::Error;

pub struct CreatePermutationParameters {}

pub struct CreatePermutationInput {}

pub struct CreatePermutation {
    full_output: Option<PermutationImageBuffer>,
}

impl CreatePermutation {
    pub fn new(_input: &CreatePermutationInput, _parameters: &CreatePermutationParameters) -> Self {
        Self { full_output: None }
    }

    pub fn step(
        &mut self,
        dispatcher: &DispatcherImplementation,
    ) -> Result<OutputStatus, Box<dyn Error>> {
        let mut mapped_buffer = dispatcher
            .resources()
            .permutation_output_buffer()
            .request_map_read();

        dispatcher.poll_device();

        let buffer_dimensions = mapped_buffer.buffer_dimensions();
        let data = mapped_buffer.collect_mapped_buffer();
        let result: Vec<PermutationImageBufferComponent> = data
            .chunks(buffer_dimensions.padded_bytes_per_row)
            .flat_map(|c| {
                c[..buffer_dimensions.unpadded_bytes_per_row]
                    .chunks_exact(std::mem::size_of::<PermutationImageBufferComponent>())
            })
            .map(|b| PermutationImageBufferComponent::from_ne_bytes(b.try_into().unwrap()))
            .collect::<Vec<PermutationImageBufferComponent>>();

        self.full_output = Some(
            PermutationImageBuffer::from_vec(
                buffer_dimensions.width as u32,
                buffer_dimensions.height as u32,
                result,
            )
            .unwrap(),
        );

        Ok(OutputStatus::FinalFullOutput)
    }

    pub fn partial_output(&self) -> Option<&()> {
        None
    }

    pub fn full_output(&self) -> Option<&PermutationImageBuffer> {
        self.full_output.as_ref()
    }
}
