use super::device::DeviceManager;
use super::operation::manager::OperationManager;
use super::resource::manager::ResourceManager;
use crate::image_utils::ImageDimensions;
use std::error::Error;
use std::fmt;

pub use super::operation::manager::PermuteOperationInput;

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
    resources: ResourceManager,
    operations: OperationManager,
    image_dimensions: ImageDimensions,
}

impl System {
    pub fn new(image_dimensions: &ImageDimensions) -> Result<Self, Box<dyn Error>> {
        let device = futures::executor::block_on(DeviceManager::new())?;
        let resources = ResourceManager::new(device.device(), image_dimensions);
        let operations = OperationManager::new(device.device(), &resources);
        Ok(Self {
            device,
            resources,
            operations,
            image_dimensions: *image_dimensions,
        })
    }

    pub fn poll_device(&self) {
        self.device.wait_for_device();
    }

    pub fn resources(&self) -> &ResourceManager {
        &self.resources
    }

    pub fn image_dimensions(&self) -> &ImageDimensions {
        &self.image_dimensions
    }

    pub fn operation_create_permutation(&mut self) -> Result<(), Box<dyn Error>> {
        self.operations
            .create_permutation(&self.resources, &self.device)
    }

    pub fn operation_permute(
        &mut self,
        input: &PermuteOperationInput,
    ) -> Result<(), Box<dyn Error>> {
        self.operations
            .permute(&self.resources, &self.device, input)
    }
}
