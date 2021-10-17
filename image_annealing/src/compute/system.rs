use super::device::DeviceManager;
use super::operation::manager::OperationManager;
use super::output::format::LosslessImageBuffer;
use crate::image_utils::validation::ValidatedPermutation;
use crate::image_utils::ImageDimensions;
use std::error::Error;
use std::fmt;

pub use super::operation::manager::PermuteOperationInput;
pub use super::operation::manager::SwapOperationInput;

#[derive(Debug, Clone)]
pub struct DimensionsMismatchError(ImageDimensions, ImageDimensions);

impl DimensionsMismatchError {
    pub fn new(first: ImageDimensions, second: ImageDimensions) -> Self {
        Self(first, second)
    }
}

impl fmt::Display for DimensionsMismatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "mismatch in image dimensions, {} and {}", self.0, self.1)
    }
}

impl Error for DimensionsMismatchError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

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

    pub fn image_dimensions(&self) -> &ImageDimensions {
        &self.image_dimensions
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

    pub fn output_permutation(&mut self) -> Result<ValidatedPermutation, Box<dyn Error>> {
        self.operations.output_permutation(&self.device)
    }

    pub fn output_permuted_image(&mut self) -> Result<LosslessImageBuffer, Box<dyn Error>> {
        self.operations.output_permuted_image(&self.device)
    }
}
