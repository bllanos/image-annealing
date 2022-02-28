use super::super::system::System;
use super::{AlreadyFailedError, AlreadyFinishedError, OutputStatus};
use std::error::Error;

pub mod create_permutation;
pub mod permute;
pub mod swap;
pub mod validate_permutation;

enum CompletionStatus {
    Pending,
    Finished,
    Failed,
}

impl CompletionStatus {
    fn new() -> Self {
        Self::Pending
    }

    fn ok_if_pending(&self) -> Result<(), Box<dyn Error>> {
        match self {
            CompletionStatus::Pending => Ok(()),
            CompletionStatus::Finished => Err(Box::new(AlreadyFinishedError)),
            CompletionStatus::Failed => Err(Box::new(AlreadyFailedError)),
        }
    }
}

trait CompletionStatusHolder {
    fn get_status(&self) -> &CompletionStatus;
    fn set_status(&mut self, status: CompletionStatus);
    fn unchecked_step(&mut self, system: &mut System) -> Result<OutputStatus, Box<dyn Error>>;

    fn checked_step(&mut self, system: &mut System) -> Result<OutputStatus, Box<dyn Error>> {
        self.get_status().ok_if_pending()?;
        match self.unchecked_step(system) {
            Ok(status) => Ok(status),
            Err(err) => {
                self.set_status(CompletionStatus::Failed);
                Err(err)
            }
        }
    }
}

trait FinalOutputHolder<Output>: CompletionStatusHolder {
    fn has_given_output(&self) -> bool;
    fn set_has_given_output(&mut self);
    fn unchecked_output(&mut self, system: &mut System) -> Option<Output>;

    fn checked_output(&mut self, system: &mut System) -> Option<Output> {
        if self.has_given_output() {
            None
        } else {
            match self.get_status() {
                CompletionStatus::Finished => {
                    self.set_has_given_output();
                    self.unchecked_output(system)
                }
                _ => None,
            }
        }
    }
}
