use super::device::DeviceManager;
use super::operation::manager::OperationManager;
use super::output::format::{ImageFormat, LosslessImage};
use crate::{ImageDimensions, ImageDimensionsHolder, ValidatedPermutation};
use std::error::Error;

pub use super::operation::manager::PermuteOperationInput;
pub use super::operation::manager::{
    CountSwapOperationOutput, CountSwapOperationOutputPass, SwapOperationInput,
};

pub struct System {
    device: DeviceManager,
    operations: OperationManager,
    image_dimensions: ImageDimensions,
}

impl System {
    pub fn new(image_dimensions: &ImageDimensions) -> Result<Self, Box<dyn Error>> {
        let device = futures::executor::block_on(DeviceManager::new())?;
        let operations = OperationManager::new(device.device(), image_dimensions);
        Ok(Self {
            device,
            operations,
            image_dimensions: *image_dimensions,
        })
    }

    pub fn operation_count_swap(&mut self) -> Result<(), Box<dyn Error>> {
        self.operations.count_swap(&self.device)
    }

    pub fn operation_create_permutation(&mut self) -> Result<(), Box<dyn Error>> {
        self.operations.create_permutation(&self.device)
    }

    pub fn operation_permute(
        &mut self,
        input: &PermuteOperationInput,
    ) -> Result<(), Box<dyn Error>> {
        self.operations.permute(&self.device, input)
    }

    pub fn operation_swap(&mut self, input: &SwapOperationInput) -> Result<(), Box<dyn Error>> {
        self.operations.swap(&self.device, input)
    }

    pub fn output_count_swap(&mut self) -> Result<CountSwapOperationOutput, Box<dyn Error>> {
        self.operations.output_count_swap(&self.device)
    }

    pub fn output_permutation(&mut self) -> Result<ValidatedPermutation, Box<dyn Error>> {
        self.operations.output_permutation(&self.device)
    }

    pub fn output_permuted_image(
        &mut self,
        format: ImageFormat,
    ) -> Result<LosslessImage, Box<dyn Error>> {
        self.operations.output_permuted_image(&self.device, format)
    }
}

impl ImageDimensionsHolder for System {
    fn dimensions(&self) -> &ImageDimensions {
        &self.image_dimensions
    }
}

#[cfg(test)]
mod tests;
