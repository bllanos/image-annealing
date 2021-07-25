use super::output::algorithm::create_permutation::{
    CreatePermutation, CreatePermutationInput, CreatePermutationParameters,
};
use super::output::algorithm::permute::{Permute, PermuteInput, PermuteOutput, PermuteParameters};
use super::output::algorithm::validate_permutation::{
    ValidatePermutation, ValidatePermutationInput, ValidatePermutationParameters,
};
use super::output::format::PermutationImageBuffer;
use super::output::{Algorithm, OutputStatus};
use super::system::System;
use crate::image_utils::validation::ValidatedPermutation;
use crate::image_utils::ImageDimensions;
use std::error::Error;
use std::fmt;

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

impl fmt::Debug for dyn Dispatcher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Dispatcher").finish_non_exhaustive()
    }
}

struct DispatcherImplementation {
    system: System,
    create_permutation: Option<CreatePermutation>,
    permute: Option<Permute>,
    validate_permutation: Option<ValidatePermutation>,
}

impl DispatcherImplementation {
    fn new(image_dimensions: &ImageDimensions) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            system: System::new(image_dimensions)?,
            create_permutation: None,
            permute: None,
            validate_permutation: None,
        })
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
        self.create_permutation
            .as_mut()
            .unwrap()
            .step(&mut self.system)
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
        self.permute.as_mut().unwrap().step(&mut self.system)
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
        self.validate_permutation
            .as_mut()
            .unwrap()
            .step(&self.system)
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
