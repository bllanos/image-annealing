use super::device::DeviceManager;
use super::operation::manager::OperationManager;
use super::output::create_permutation::{
    CreatePermutation, CreatePermutationInput, CreatePermutationParameters,
};
use super::output::format::PermutationImageBuffer;
use super::output::{Algorithm, OutputStatus};
use super::resource::manager::ResourceManager;
use crate::image_utils::ImageDimensions;
use std::error::Error;

pub fn create_dispatcher(
    image_dimensions: &ImageDimensions,
) -> Result<Box<dyn Dispatcher>, Box<dyn Error>> {
    Ok(Box::new(DispatcherImplementation::new(image_dimensions)?))
}

pub type CreatePermutationAlgorithm = dyn Algorithm<(), PermutationImageBuffer>;

pub trait Dispatcher {
    fn create_permutation(
        self: Box<Self>,
        input: CreatePermutationInput,
        parameters: CreatePermutationParameters,
    ) -> Box<CreatePermutationAlgorithm>;
}

pub struct DispatcherImplementation {
    device: DeviceManager,
    resources: ResourceManager,
    operations: OperationManager,
    create_permutation: Option<CreatePermutation>,
}

impl DispatcherImplementation {
    pub fn new(image_dimensions: &ImageDimensions) -> Result<Self, Box<dyn Error>> {
        let device = futures::executor::block_on(DeviceManager::new())?;
        let resources = ResourceManager::new(device.device(), image_dimensions);
        let operations = OperationManager::new(device.device(), &resources);
        Ok(Self {
            device,
            resources,
            operations,
            create_permutation: None,
        })
    }

    pub fn poll_device(&self) {
        self.device.wait_for_device();
    }

    pub fn resources(&self) -> &ResourceManager {
        &self.resources
    }
}

impl Dispatcher for DispatcherImplementation {
    fn create_permutation(
        mut self: Box<Self>,
        input: CreatePermutationInput,
        parameters: CreatePermutationParameters,
    ) -> Box<CreatePermutationAlgorithm> {
        self.operations
            .create_permutation(&self.resources, &self.device);
        self.create_permutation = Some(CreatePermutation::new(input, parameters));
        self
    }
}

impl Algorithm<(), PermutationImageBuffer> for DispatcherImplementation {
    fn step(&mut self) -> Result<OutputStatus, Box<dyn Error>> {
        let mut create_permutation = self.create_permutation.take().unwrap();
        let result = create_permutation.step(self);
        self.create_permutation = Some(create_permutation);
        result
    }
    fn partial_output(&self) -> Option<&()> {
        self.create_permutation.as_ref().unwrap().partial_output()
    }
    fn full_output(&self) -> Option<&PermutationImageBuffer> {
        self.create_permutation.as_ref().unwrap().full_output()
    }
    fn return_to_dispatcher(mut self: Box<Self>) -> Box<dyn Dispatcher> {
        self.create_permutation = None;
        self
    }
}
