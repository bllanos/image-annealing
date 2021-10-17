use super::{AlreadyFailedError, AlreadyFinishedError};
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
