use super::device::DeviceManager;
use super::operation::manager::OperationManager;
use super::output::algorithm::create_permutation::{
    CreatePermutation, CreatePermutationInput, CreatePermutationParameters,
};
use super::output::algorithm::permute::{Permute, PermuteInput, PermuteOutput, PermuteParameters};
use super::output::algorithm::validate_permutation::{
    ValidatePermutation, ValidatePermutationInput, ValidatePermutationParameters,
};
use super::output::format::PermutationImageBuffer;
use super::output::{Algorithm, OutputStatus};
use super::resource::manager::ResourceManager;
use crate::image_utils::validation::ValidatedPermutation;
use crate::image_utils::ImageDimensions;
use std::error::Error;
use std::fmt;

pub use super::operation::manager::PermuteOperationInput;

#[derive(Debug, Clone)]
pub struct DimensionsMismatchError;

impl fmt::Display for DimensionsMismatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mismatch in image dimensions")
    }
}

impl Error for DimensionsMismatchError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub fn create_dispatcher(
    image_dimensions: &ImageDimensions,
) -> Result<Box<dyn Dispatcher>, Box<dyn Error>> {
    Ok(Box::new(DispatcherImplementation::new(image_dimensions)?))
}

pub type CreatePermutationAlgorithm = dyn Algorithm<(), PermutationImageBuffer>;
pub type PermuteAlgorithm = dyn Algorithm<(), PermuteOutput>;
pub type ValidatePermutationAlgorithm = dyn Algorithm<(), ValidatedPermutation>;

pub trait Dispatcher {
    fn create_permutation(
        self: Box<Self>,
        input: CreatePermutationInput,
        parameters: CreatePermutationParameters,
    ) -> Box<CreatePermutationAlgorithm>;
    fn permute(
        self: Box<Self>,
        input: PermuteInput,
        parameters: PermuteParameters,
    ) -> Box<PermuteAlgorithm>;
    fn validate_permutation(
        self: Box<Self>,
        input: ValidatePermutationInput,
        parameters: ValidatePermutationParameters,
    ) -> Box<ValidatePermutationAlgorithm>;
}

pub struct DispatcherImplementation {
    device: DeviceManager,
    resources: ResourceManager,
    operations: OperationManager,
    image_dimensions: ImageDimensions,
    create_permutation: Option<CreatePermutation>,
    permute: Option<Permute>,
    validate_permutation: Option<ValidatePermutation>,
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
            image_dimensions: *image_dimensions,
            create_permutation: None,
            permute: None,
            validate_permutation: None,
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

impl Dispatcher for DispatcherImplementation {
    fn create_permutation(
        mut self: Box<Self>,
        input: CreatePermutationInput,
        parameters: CreatePermutationParameters,
    ) -> Box<CreatePermutationAlgorithm> {
        self.create_permutation = Some(CreatePermutation::new(input, parameters));
        self
    }

    fn permute(
        mut self: Box<Self>,
        input: PermuteInput,
        parameters: PermuteParameters,
    ) -> Box<PermuteAlgorithm> {
        self.permute = Some(Permute::new(input, parameters));
        self
    }

    fn validate_permutation(
        mut self: Box<Self>,
        input: ValidatePermutationInput,
        parameters: ValidatePermutationParameters,
    ) -> Box<ValidatePermutationAlgorithm> {
        self.validate_permutation = Some(ValidatePermutation::new(input, parameters));
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
    fn partial_output(&mut self) -> Option<()> {
        self.create_permutation.as_ref().unwrap().partial_output()
    }
    fn full_output(&mut self) -> Option<PermutationImageBuffer> {
        self.create_permutation.as_mut().unwrap().full_output()
    }
    fn return_to_dispatcher(mut self: Box<Self>) -> Box<dyn Dispatcher> {
        self.create_permutation = None;
        self
    }
}

impl Algorithm<(), PermuteOutput> for DispatcherImplementation {
    fn step(&mut self) -> Result<OutputStatus, Box<dyn Error>> {
        let mut permute = self.permute.take().unwrap();
        let result = permute.step(self);
        self.permute = Some(permute);
        result
    }
    fn partial_output(&mut self) -> Option<()> {
        self.permute.as_ref().unwrap().partial_output()
    }
    fn full_output(&mut self) -> Option<PermuteOutput> {
        self.permute.as_mut().unwrap().full_output()
    }
    fn return_to_dispatcher(mut self: Box<Self>) -> Box<dyn Dispatcher> {
        self.permute = None;
        self
    }
}

impl Algorithm<(), ValidatedPermutation> for DispatcherImplementation {
    fn step(&mut self) -> Result<OutputStatus, Box<dyn Error>> {
        let mut validate_permutation = self.validate_permutation.take().unwrap();
        let result = validate_permutation.step(self);
        self.validate_permutation = Some(validate_permutation);
        result
    }
    fn partial_output(&mut self) -> Option<()> {
        self.validate_permutation.as_ref().unwrap().partial_output()
    }
    fn full_output(&mut self) -> Option<ValidatedPermutation> {
        self.validate_permutation.as_mut().unwrap().full_output()
    }
    fn return_to_dispatcher(mut self: Box<Self>) -> Box<dyn Dispatcher> {
        self.validate_permutation = None;
        self
    }
}
