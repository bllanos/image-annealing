use super::super::super::super::resource::manager::ResourceManager;
use super::super::{PermuteOperationInput, SwapOperationInput};
use super::data::ResourceStateFlags;
use crate::{DisplacementGoal, ValidatedPermutation};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum InsufficientInputError {
    Permutation,
    OriginalImage,
    DisplacementGoal,
}

impl fmt::Display for InsufficientInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "an input {} must be provided as there is none to reuse",
            match self {
                InsufficientInputError::Permutation => "permutation",
                InsufficientInputError::OriginalImage => "image",
                InsufficientInputError::DisplacementGoal => "displacement goal field",
            }
        )
    }
}

impl Error for InsufficientInputError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone)]
pub enum InsufficientOutputError {
    Permutation,
    PermutedImage,
}

impl fmt::Display for InsufficientOutputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "an output {} does not exist or has been invalidated",
            match self {
                InsufficientOutputError::Permutation => "permutation",
                InsufficientOutputError::PermutedImage => "image",
            }
        )
    }
}

impl Error for InsufficientOutputError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[must_use]
pub struct ResourceStateTransaction<'a> {
    manager: &'a mut ResourceStateManager,
    rollback_state: ResourceStateFlags,
    commit_state: ResourceStateFlags,
    commit: bool,
}

impl<'a> ResourceStateTransaction<'a> {
    fn new(
        manager: &'a mut ResourceStateManager,
        rollback_state: ResourceStateFlags,
        commit_state: ResourceStateFlags,
    ) -> Self {
        Self {
            manager,
            rollback_state,
            commit_state,
            commit: false,
        }
    }

    pub fn set_commit(&mut self) {
        self.commit = true;
    }
}

impl Drop for ResourceStateTransaction<'_> {
    fn drop(&mut self) {
        if self.commit {
            self.manager.flags = self.commit_state;
        } else {
            self.manager.flags = self.rollback_state;
        }
    }
}

pub struct ResourceStateManager {
    flags: ResourceStateFlags,
}

impl ResourceStateManager {
    pub fn new() -> Self {
        Self {
            flags: ResourceStateFlags::new(),
        }
    }

    fn input_permutation<'a>(
        &self,
        commit_state: ResourceStateFlags,
        resources: &ResourceManager,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        permutation: &Option<&'a ValidatedPermutation>,
    ) -> Result<ResourceStateFlags, InsufficientInputError> {
        match permutation {
            Some(permutation) => {
                resources
                    .permutation_input_texture()
                    .load(queue, permutation);
                Ok(commit_state.input_permutation())
            }
            None => {
                if self.flags.check_permutation_output_texture() {
                    resources
                        .permutation_input_texture()
                        .copy(encoder, resources.permutation_output_texture());
                    Ok(commit_state.recycle_output_permutation())
                } else if self.flags.check_permutation_input_texture() {
                    Ok(commit_state)
                } else {
                    Err(InsufficientInputError::Permutation)
                }
            }
        }
    }

    fn input_image<'a>(
        &self,
        commit_state: ResourceStateFlags,
        resources: &ResourceManager,
        queue: &wgpu::Queue,
        image: &Option<&'a image::DynamicImage>,
    ) -> Result<ResourceStateFlags, InsufficientInputError> {
        match image {
            Some(image) => {
                resources.lossless_image_input_texture().load(queue, image);
                Ok(commit_state.input_lossless_image())
            }
            None => {
                if self.flags.check_lossless_image_input_texture() {
                    Ok(commit_state)
                } else {
                    Err(InsufficientInputError::OriginalImage)
                }
            }
        }
    }

    fn input_displacement_goal<'a>(
        &self,
        commit_state: ResourceStateFlags,
        resources: &ResourceManager,
        queue: &wgpu::Queue,
        image: &Option<&'a DisplacementGoal>,
    ) -> Result<ResourceStateFlags, InsufficientInputError> {
        match image {
            Some(displacement_goal) => {
                resources
                    .displacement_goal_input_texture()
                    .load(queue, displacement_goal);
                Ok(commit_state.input_displacement_goal())
            }
            None => {
                if self.flags.check_displacement_goal_input_texture() {
                    Ok(commit_state)
                } else {
                    Err(InsufficientInputError::DisplacementGoal)
                }
            }
        }
    }

    pub fn create_permutation(&mut self) -> Result<ResourceStateTransaction, Box<dyn Error>> {
        let rollback_state = self.flags.prepare_create_permutation();
        let commit_state = self.flags.finish_create_permutation();
        Ok(ResourceStateTransaction::new(
            self,
            rollback_state,
            commit_state,
        ))
    }

    pub fn permute(
        &mut self,
        resources: &ResourceManager,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        input: &PermuteOperationInput,
    ) -> Result<ResourceStateTransaction, Box<dyn Error>> {
        let rollback_state = self.flags.clear_output_lossless_image();
        let mut commit_state = rollback_state;
        commit_state =
            self.input_permutation(commit_state, resources, queue, encoder, &input.permutation)?;
        commit_state = self.input_image(commit_state, resources, queue, &input.image)?;
        commit_state = commit_state.permute_lossless_image();
        Ok(ResourceStateTransaction::new(
            self,
            rollback_state,
            commit_state,
        ))
    }

    pub fn swap(
        &mut self,
        resources: &ResourceManager,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        input: &SwapOperationInput,
    ) -> Result<ResourceStateTransaction, Box<dyn Error>> {
        let rollback_state = self
            .flags
            .clear_output_lossless_image()
            .clear_output_permutation();
        let mut commit_state = rollback_state;
        commit_state =
            self.input_permutation(commit_state, resources, queue, encoder, &input.permutation)?;
        commit_state =
            self.input_displacement_goal(commit_state, resources, queue, &input.displacement_goal)?;
        commit_state = commit_state.finish_create_permutation();
        Ok(ResourceStateTransaction::new(
            self,
            rollback_state,
            commit_state,
        ))
    }

    pub fn output_permutation(
        &mut self,
        resources: &ResourceManager,
        encoder: &mut wgpu::CommandEncoder,
    ) -> Result<ResourceStateTransaction, Box<dyn Error>> {
        let rollback_state = self.flags;
        let mut commit_state = rollback_state;
        if self.flags.check_permutation_output_texture() {
            if !self.flags.check_permutation_output_buffer() {
                resources
                    .permutation_output_buffer()
                    .load(encoder, resources.permutation_output_texture());
                commit_state = commit_state.output_permutation();
            }
            Ok(ResourceStateTransaction::new(
                self,
                rollback_state,
                commit_state,
            ))
        } else {
            Err(Box::new(InsufficientOutputError::Permutation))
        }
    }

    pub fn output_permuted_image(
        &mut self,
        resources: &ResourceManager,
        encoder: &mut wgpu::CommandEncoder,
    ) -> Result<ResourceStateTransaction, Box<dyn Error>> {
        let rollback_state = self.flags;
        let mut commit_state = rollback_state;
        if self.flags.check_lossless_image_output_texture() {
            if !self.flags.check_lossless_image_output_buffer() {
                resources
                    .lossless_image_output_buffer()
                    .load(encoder, resources.lossless_image_output_texture());
                commit_state = commit_state.output_lossless_image();
            }
            Ok(ResourceStateTransaction::new(
                self,
                rollback_state,
                commit_state,
            ))
        } else {
            Err(Box::new(InsufficientOutputError::PermutedImage))
        }
    }
}
