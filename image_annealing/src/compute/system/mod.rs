use super::device::DeviceManager;
use super::link::swap::SwapPassSequence;
use super::operation::manager::CountSwapOperationOutput;
use super::operation::manager::OperationManager;
use super::output::format::{ImageFormat, LosslessImage};
use crate::{DisplacementGoal, ImageDimensions, ImageDimensionsHolder, ValidatedPermutation};
use std::error::Error;

pub use super::device::DevicePollType;
pub use super::operation::manager::{
    CreateDisplacementGoalOperationInput, PermuteOperationInput, SwapOperationInput,
};

pub struct System {
    device: DeviceManager,
    operations: OperationManager,
    image_dimensions: ImageDimensions,
}

impl System {
    pub async fn new(image_dimensions: &ImageDimensions) -> Result<Self, Box<dyn Error>> {
        let device = DeviceManager::new().await?;
        let operations = OperationManager::new(device.device(), image_dimensions);
        Ok(Self {
            device,
            operations,
            image_dimensions: *image_dimensions,
        })
    }

    pub fn operation_count_swap(
        &mut self,
        sequence: SwapPassSequence,
    ) -> Result<(), Box<dyn Error>> {
        self.operations.count_swap(&self.device, sequence)
    }

    pub fn operation_create_displacement_goal(
        &mut self,
        input: &CreateDisplacementGoalOperationInput,
    ) -> Result<(), Box<dyn Error>> {
        self.operations
            .create_displacement_goal(&self.device, input)
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

    pub async fn output_count_swap(
        &mut self,
        poll_type: DevicePollType,
        sequence: &SwapPassSequence,
    ) -> Result<CountSwapOperationOutput, Box<dyn Error>> {
        self.operations
            .output_count_swap(&self.device, poll_type, sequence)
            .await
    }

    pub async fn output_displacement_goal(
        &mut self,
        poll_type: DevicePollType,
    ) -> Result<DisplacementGoal, Box<dyn Error>> {
        self.operations
            .output_displacement_goal(&self.device, poll_type)
            .await
    }

    pub async fn output_permutation(
        &mut self,
        poll_type: DevicePollType,
    ) -> Result<ValidatedPermutation, Box<dyn Error>> {
        self.operations
            .output_permutation(&self.device, poll_type)
            .await
    }

    pub async fn output_permuted_image(
        &mut self,
        poll_type: DevicePollType,
        format: ImageFormat,
    ) -> Result<LosslessImage, Box<dyn Error>> {
        self.operations
            .output_permuted_image(&self.device, poll_type, format)
            .await
    }
}

impl ImageDimensionsHolder for System {
    fn dimensions(&self) -> &ImageDimensions {
        &self.image_dimensions
    }
}

#[cfg(test)]
mod tests;
