use super::dispatch::Dispatcher;
use std::error::Error;
use std::fmt;

pub mod algorithm;
pub mod conversion;
pub mod format;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum OutputStatus {
    NoNewOutput,
    NewPartialOutput,
    NewFullOutput,
    NewPartialAndFullOutput,
    FinalPartialOutput,
    FinalFullOutput,
    FinalPartialAndFullOutput,
}

impl OutputStatus {
    pub fn is_final(&self) -> bool {
        match self {
            Self::NoNewOutput
            | Self::NewFullOutput
            | Self::NewPartialOutput
            | Self::NewPartialAndFullOutput => false,
            Self::FinalPartialOutput | Self::FinalFullOutput | Self::FinalPartialAndFullOutput => {
                true
            }
        }
    }

    pub fn is_partial(&self) -> bool {
        match self {
            Self::NoNewOutput | Self::NewFullOutput | Self::FinalFullOutput => false,
            Self::NewPartialOutput
            | Self::NewPartialAndFullOutput
            | Self::FinalPartialOutput
            | Self::FinalPartialAndFullOutput => true,
        }
    }

    pub fn is_full(&self) -> bool {
        match self {
            Self::NoNewOutput | Self::NewPartialOutput | Self::FinalPartialOutput => false,
            Self::NewFullOutput
            | Self::NewPartialAndFullOutput
            | Self::FinalFullOutput
            | Self::FinalPartialAndFullOutput => true,
        }
    }
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

    fn step_until_finished(&mut self) -> Result<OutputStatus, Box<dyn Error>> {
        let mut status = self.step()?;
        while !status.is_final() {
            status = self.step()?;
        }
        Ok(status)
    }
}

#[cfg(test)]
mod tests;
