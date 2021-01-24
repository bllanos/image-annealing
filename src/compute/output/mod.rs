use super::dispatch::Dispatcher;
use std::error::Error;

pub mod conversion;
pub mod create_permutation;
pub mod format;

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

pub trait Algorithm<PartialOutput, FullOutput> {
    fn step(&mut self) -> Result<OutputStatus, Box<dyn Error>>;
    fn partial_output(&self) -> Option<&PartialOutput>;
    fn full_output(&self) -> Option<&FullOutput>;
    fn return_to_dispatcher(self: Box<Self>) -> Box<dyn Dispatcher>;
}
