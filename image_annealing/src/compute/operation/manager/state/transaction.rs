use super::super::super::super::link::swap::{
    CountSwapInputLayout, SwapPassSequence, SwapShaderParameters,
};
use super::super::super::super::output::format::LosslessImage;
use super::super::super::super::resource::manager::ResourceManager;
use super::super::{PermuteOperationInput, SwapOperationInput};
use super::data::ResourceStateFlags;
use crate::{DisplacementGoal, ImageDimensions, ValidatedPermutation};
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
        match self {
            Self::Permutation => write!(
                f,
                "an input permutation must be provided as there is none to reuse"
            ),
            Self::OriginalImage => write!(
                f,
                "an input image must be provided as there is none to reuse"
            ),
            Self::DisplacementGoal => write!(
                f,
                "an input displacement goal field must be provided as there is none to reuse"
            ),
        }
    }
}

impl Error for InsufficientInputError {}

#[derive(Debug, Clone)]
pub enum InsufficientOutputError {
    CountSwap,
    CountSwapPass,
    SwapPass,
    Permutation,
    PermutedImage,
}

impl fmt::Display for InsufficientOutputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CountSwap => write!(f, "no current output swap counts exist",),
            Self::CountSwapPass => write!(
                f,
                "not all selected swap passes were counted during the last count swap operation, if one was performed"
            ),
            Self::SwapPass => write!(
                f,
                "not all selected swap passes have occurred since the last count swap operation"
            ),
            Self::Permutation => write!(
                f,
                "an output permutation does not exist or has been invalidated",
            ),
            Self::PermutedImage => {
                write!(f, "an output image does not exist or has been invalidated",)
            }
        }
    }
}

impl Error for InsufficientOutputError {}

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
    count_swap_parameters: CountSwapInputLayout,
    swap_parameters: SwapShaderParameters,
}

impl ResourceStateManager {
    pub fn new(image_dimensions: &ImageDimensions) -> Self {
        Self {
            flags: ResourceStateFlags::new(),
            count_swap_parameters: CountSwapInputLayout::new(image_dimensions),
            swap_parameters: SwapShaderParameters::new(),
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
        image: &Option<&'a LosslessImage>,
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

    pub fn count_swap(
        &mut self,
        resources: &ResourceManager,
        queue: &wgpu::Queue,
        sequence: SwapPassSequence,
    ) -> Result<ResourceStateTransaction, Box<dyn Error>> {
        if self
            .flags
            .check_count_swap_pass_set()
            .contains_set(&sequence)
        {
            let rollback_state = self.flags.clear_count_swap_pass_set();
            let mut commit_state = rollback_state;
            if self.count_swap_parameters.update_set(sequence.into()) {
                resources
                    .count_swap_input_layout_buffer()
                    .load(queue, &self.count_swap_parameters)
            }
            commit_state = commit_state.finish_count_swap();
            Ok(ResourceStateTransaction::new(
                self,
                rollback_state,
                commit_state,
            ))
        } else {
            Err(Box::new(InsufficientOutputError::SwapPass))
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
            .clear_output_permutation()
            .clear_output_count_swap();
        let mut commit_state = rollback_state;
        commit_state =
            self.input_permutation(commit_state, resources, queue, encoder, &input.permutation)?;
        commit_state =
            self.input_displacement_goal(commit_state, resources, queue, &input.displacement_goal)?;
        commit_state = commit_state.finish_swap(input.pass);

        self.swap_parameters
            .set_pass(input.pass, &self.count_swap_parameters);
        self.swap_parameters
            .set_acceptance_threshold(input.acceptance_threshold);
        resources
            .swap_parameters_buffer()
            .load(queue, &self.swap_parameters);

        Ok(ResourceStateTransaction::new(
            self,
            rollback_state,
            commit_state,
        ))
    }

    pub fn output_count_swap(
        &mut self,
        resources: &ResourceManager,
        encoder: &mut wgpu::CommandEncoder,
        sequence: &SwapPassSequence,
    ) -> Result<ResourceStateTransaction, Box<dyn Error>> {
        if self.count_swap_parameters.get_set().contains_set(sequence) {
            let rollback_state = self.flags;
            let mut commit_state = rollback_state;
            if self.flags.check_count_swap_output_storage_buffer() {
                if !self.flags.check_count_swap_output_buffer() {
                    resources
                        .count_swap_output_buffer()
                        .load(encoder, resources.count_swap_output_storage_buffer());
                    commit_state = commit_state.output_count_swap();
                }
                Ok(ResourceStateTransaction::new(
                    self,
                    rollback_state,
                    commit_state,
                ))
            } else {
                Err(Box::new(InsufficientOutputError::CountSwap))
            }
        } else {
            Err(Box::new(InsufficientOutputError::CountSwapPass))
        }
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
