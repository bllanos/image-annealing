use super::dispatch::Dispatcher;
use std::error::Error;
use std::fmt;

pub mod conversion;
pub mod create_permutation;
pub mod format;
pub mod validate_permutation;

#[derive(Debug, Eq, PartialEq)]
pub enum OutputStatus {
    NoNewOutput,
    NewPartialOutput,
    NewFullOutput,
    NewPartialAndFullOutput,
    FinalPartialOutput,
    FinalFullOutput,
    FinalPartialAndFullOutput,
}

#[derive(Debug, Clone)]
struct AlreadyFailedError;

impl fmt::Display for AlreadyFailedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Cannot proceed because of an error during the previous call to Algorithm::step."
        )
    }
}

impl Error for AlreadyFailedError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub trait Algorithm<PartialOutput, FullOutput> {
    fn step(&mut self) -> Result<OutputStatus, Box<dyn Error>>;
    fn partial_output(&self) -> Option<&PartialOutput>;
    fn full_output(&self) -> Option<&FullOutput>;
    fn return_to_dispatcher(self: Box<Self>) -> Box<dyn Dispatcher>;
}
