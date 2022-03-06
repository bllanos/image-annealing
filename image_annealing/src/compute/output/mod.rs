use super::dispatch::Dispatcher;
use std::error::Error;
use std::fmt;

pub mod algorithm;
pub mod conversion;
pub mod format;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
            "cannot proceed because of an error during the previous call to Algorithm::step"
        )
    }
}

impl Error for AlreadyFailedError {}

#[derive(Debug, Clone)]
struct AlreadyFinishedError;

impl fmt::Display for AlreadyFinishedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Algorithm::step cannot be called after the final output has been computed"
        )
    }
}

impl Error for AlreadyFinishedError {}

pub trait Algorithm<PartialOutput, FullOutput> {
    fn step(&mut self) -> Result<OutputStatus, Box<dyn Error>>;
    fn partial_output(&mut self) -> Option<PartialOutput>;
    fn full_output(&mut self) -> Option<FullOutput>;
    fn return_to_dispatcher(self: Box<Self>) -> Box<dyn Dispatcher>;

    fn step_until(&mut self, status: OutputStatus) -> Result<(), Box<dyn Error>> {
        while self.step()? != status {}
        Ok(())
    }
}
