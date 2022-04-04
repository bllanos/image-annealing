use super::output::algorithm::create_permutation::{
    CreatePermutation, CreatePermutationInput, CreatePermutationOutput, CreatePermutationParameters,
};
use super::output::algorithm::permute::{Permute, PermuteInput, PermuteOutput, PermuteParameters};
use super::output::algorithm::swap::{
    Swap, SwapFullOutput, SwapInput, SwapParameters, SwapPartialOutput,
};
use super::output::algorithm::validate_permutation::{
    ValidatePermutation, ValidatePermutationInput, ValidatePermutationOutput,
    ValidatePermutationParameters,
};
use super::output::{Algorithm, OutputStatus};
use super::system::System;
use crate::ImageDimensions;
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    pub image_dimensions: ImageDimensions,
}

pub fn create_dispatcher(config: &Config) -> Result<Box<dyn Dispatcher>, Box<dyn Error>> {
    Ok(Box::new(DispatcherImplementation::new(config)?))
}

pub type CreatePermutationAlgorithm = dyn Algorithm<(), CreatePermutationOutput>;
pub type PermuteAlgorithm = dyn Algorithm<(), PermuteOutput>;
pub type SwapAlgorithm = dyn Algorithm<SwapPartialOutput, SwapFullOutput>;
pub type ValidatePermutationAlgorithm = dyn Algorithm<(), ValidatePermutationOutput>;

pub trait Dispatcher {
    fn create_permutation(
        self: Box<Self>,
        input: CreatePermutationInput,
        parameters: &CreatePermutationParameters,
    ) -> Box<CreatePermutationAlgorithm>;
    fn permute(
        self: Box<Self>,
        input: PermuteInput,
        parameters: &PermuteParameters,
    ) -> Box<PermuteAlgorithm>;
    fn swap(self: Box<Self>, input: SwapInput, parameters: &SwapParameters) -> Box<SwapAlgorithm>;
    fn validate_permutation(
        self: Box<Self>,
        input: ValidatePermutationInput,
        parameters: &ValidatePermutationParameters,
    ) -> Box<ValidatePermutationAlgorithm>;
}

impl fmt::Debug for dyn Dispatcher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Dispatcher").finish_non_exhaustive()
    }
}

#[allow(clippy::large_enum_variant)]
enum AlgorithmChoice {
    None,
    CreatePermutation(CreatePermutation),
    Permute(Permute),
    Swap(Swap),
    ValidatePermutation(ValidatePermutation),
}

impl AlgorithmChoice {
    fn as_ref_create_permutation(&self) -> &CreatePermutation {
        match self {
            AlgorithmChoice::CreatePermutation(inner) => inner,
            _ => panic!("expected AlgorithmChoice::CreatePermutation"),
        }
    }
    fn as_mut_create_permutation(&mut self) -> &mut CreatePermutation {
        match self {
            AlgorithmChoice::CreatePermutation(ref mut inner) => inner,
            _ => panic!("expected AlgorithmChoice::CreatePermutation"),
        }
    }
    fn as_ref_permute(&self) -> &Permute {
        match self {
            AlgorithmChoice::Permute(inner) => inner,
            _ => panic!("expected AlgorithmChoice::Permute"),
        }
    }
    fn as_mut_permute(&mut self) -> &mut Permute {
        match self {
            AlgorithmChoice::Permute(ref mut inner) => inner,
            _ => panic!("expected AlgorithmChoice::Permute"),
        }
    }
    fn as_mut_swap(&mut self) -> &mut Swap {
        match self {
            AlgorithmChoice::Swap(ref mut inner) => inner,
            _ => panic!("expected AlgorithmChoice::Swap"),
        }
    }
    fn as_ref_validate_permutation(&self) -> &ValidatePermutation {
        match self {
            AlgorithmChoice::ValidatePermutation(inner) => inner,
            _ => panic!("expected AlgorithmChoice::ValidatePermutation"),
        }
    }
    fn as_mut_validate_permutation(&mut self) -> &mut ValidatePermutation {
        match self {
            AlgorithmChoice::ValidatePermutation(ref mut inner) => inner,
            _ => panic!("expected AlgorithmChoice::ValidatePermutation"),
        }
    }
}

struct DispatcherImplementation {
    system: System,
    algorithm: AlgorithmChoice,
}

impl DispatcherImplementation {
    fn new(config: &Config) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            system: System::new(&config.image_dimensions)?,
            algorithm: AlgorithmChoice::None,
        })
    }

    fn clear_algorithm(&mut self) {
        self.algorithm = AlgorithmChoice::None;
    }
}

impl Dispatcher for DispatcherImplementation {
    fn create_permutation(
        mut self: Box<Self>,
        input: CreatePermutationInput,
        parameters: &CreatePermutationParameters,
    ) -> Box<CreatePermutationAlgorithm> {
        self.algorithm =
            AlgorithmChoice::CreatePermutation(CreatePermutation::new(input, parameters));
        self
    }

    fn permute(
        mut self: Box<Self>,
        input: PermuteInput,
        parameters: &PermuteParameters,
    ) -> Box<PermuteAlgorithm> {
        self.algorithm = AlgorithmChoice::Permute(Permute::new(input, parameters));
        self
    }

    fn swap(
        mut self: Box<Self>,
        input: SwapInput,
        parameters: &SwapParameters,
    ) -> Box<SwapAlgorithm> {
        self.algorithm = AlgorithmChoice::Swap(Swap::new(input, parameters));
        self
    }

    fn validate_permutation(
        mut self: Box<Self>,
        input: ValidatePermutationInput,
        parameters: &ValidatePermutationParameters,
    ) -> Box<ValidatePermutationAlgorithm> {
        self.algorithm =
            AlgorithmChoice::ValidatePermutation(ValidatePermutation::new(input, parameters));
        self
    }
}

impl Algorithm<(), CreatePermutationOutput> for DispatcherImplementation {
    fn step(&mut self) -> Result<OutputStatus, Box<dyn Error>> {
        self.algorithm
            .as_mut_create_permutation()
            .step(&mut self.system)
    }
    fn partial_output(&mut self) -> Option<()> {
        self.algorithm.as_ref_create_permutation().partial_output()
    }
    fn full_output(&mut self) -> Option<CreatePermutationOutput> {
        self.algorithm
            .as_mut_create_permutation()
            .full_output(&mut self.system)
    }
    fn return_to_dispatcher(mut self: Box<Self>) -> Box<dyn Dispatcher> {
        self.clear_algorithm();
        self
    }
}

impl Algorithm<(), PermuteOutput> for DispatcherImplementation {
    fn step(&mut self) -> Result<OutputStatus, Box<dyn Error>> {
        self.algorithm.as_mut_permute().step(&mut self.system)
    }
    fn partial_output(&mut self) -> Option<()> {
        self.algorithm.as_ref_permute().partial_output()
    }
    fn full_output(&mut self) -> Option<PermuteOutput> {
        self.algorithm
            .as_mut_permute()
            .full_output(&mut self.system)
    }
    fn return_to_dispatcher(mut self: Box<Self>) -> Box<dyn Dispatcher> {
        self.clear_algorithm();
        self
    }
}

impl Algorithm<SwapPartialOutput, SwapFullOutput> for DispatcherImplementation {
    fn step(&mut self) -> Result<OutputStatus, Box<dyn Error>> {
        self.algorithm.as_mut_swap().step(&mut self.system)
    }
    fn partial_output(&mut self) -> Option<SwapPartialOutput> {
        self.algorithm
            .as_mut_swap()
            .partial_output(&mut self.system)
    }
    fn full_output(&mut self) -> Option<SwapFullOutput> {
        self.algorithm.as_mut_swap().full_output(&mut self.system)
    }
    fn return_to_dispatcher(mut self: Box<Self>) -> Box<dyn Dispatcher> {
        self.clear_algorithm();
        self
    }
}

impl Algorithm<(), ValidatePermutationOutput> for DispatcherImplementation {
    fn step(&mut self) -> Result<OutputStatus, Box<dyn Error>> {
        self.algorithm
            .as_mut_validate_permutation()
            .step(&mut self.system)
    }
    fn partial_output(&mut self) -> Option<()> {
        self.algorithm
            .as_ref_validate_permutation()
            .partial_output()
    }
    fn full_output(&mut self) -> Option<ValidatePermutationOutput> {
        self.algorithm.as_mut_validate_permutation().full_output()
    }
    fn return_to_dispatcher(mut self: Box<Self>) -> Box<dyn Dispatcher> {
        self.clear_algorithm();
        self
    }
}
