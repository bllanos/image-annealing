use super::super::system::{DevicePollType, System};
use super::{AlreadyFailedError, AlreadyFinishedError, OutputStatus};
use async_trait::async_trait;
use std::error::Error;
use std::num::NonZeroU32;

pub mod create_displacement_goal;
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

#[async_trait]
trait FinalOutputHolder<Output: Send>: CompletionStatusHolder {
    fn has_given_output(&self) -> bool;
    fn set_has_given_output(&mut self);
    async fn unchecked_output(
        &mut self,
        system: &mut System,
        poll_type: DevicePollType,
    ) -> Option<Output>;

    async fn checked_output(
        &mut self,
        system: &mut System,
        poll_type: DevicePollType,
    ) -> Option<Output> {
        if self.has_given_output() {
            None
        } else {
            let status = self.get_status();
            match status {
                CompletionStatus::Finished => {
                    self.set_has_given_output();
                    self.unchecked_output(system, poll_type).await
                }
                _ => None,
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkgroupGridConfig {
    BlockSize {
        width: NonZeroU32,
        height: NonZeroU32,
    },
    Fixed {
        width: NonZeroU32,
        height: NonZeroU32,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PipelineConfig<T: Clone + std::fmt::Debug + Eq + PartialEq> {
    pub shader_config: T,
    pub workgroup_grid: WorkgroupGridConfig,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PipelineOperation<T: Clone + std::fmt::Debug + Eq + PartialEq> {
    Set(PipelineConfig<T>),
    SetDefault,
    Preserve,
}

impl<T: Clone + std::fmt::Debug + Eq + PartialEq> Default for PipelineOperation<T> {
    fn default() -> Self {
        Self::SetDefault
    }
}
