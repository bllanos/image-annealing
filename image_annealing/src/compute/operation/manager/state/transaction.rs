use super::super::super::super::link::swap::{
    CountSwapInputLayout, SwapPassSequence, SwapShaderParameters,
};
use super::super::super::super::output::format::LosslessImage;
use super::super::super::super::resource::manager::ResourceManager;
use super::super::{
    CreateDisplacementGoalOperationInput, PermuteOperationInput, SwapOperationInput,
};
use super::data::AllResourcesState;
use crate::{DisplacementGoal, ImageDimensions, ValidatedPermutation};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum InsufficientInputError {
    OriginalImage,
    DisplacementGoal,
}

impl fmt::Display for InsufficientInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
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
    SwapPass,
    DisplacementGoal,
    Permutation,
    PermutedImage,
}

impl fmt::Display for InsufficientOutputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CountSwap => write!(
                f,
                "not all selected swap passes were counted during the last count swap operation, if one was performed"
            ),
            Self::SwapPass => write!(
                f,
                "not all selected swap passes have occurred since the last count swap operation"
            ),
            Self::DisplacementGoal => write!(
                f,
                "an output displacement goal field does not exist or has been invalidated",
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

pub struct ResourceStateManager {
    flags: AllResourcesState,
    count_swap_parameters: CountSwapInputLayout,
    swap_parameters: SwapShaderParameters,
}

impl ResourceStateManager {
    pub fn new(image_dimensions: &ImageDimensions) -> Self {
        Self {
            flags: AllResourcesState::new(),
            count_swap_parameters: CountSwapInputLayout::new(image_dimensions),
            swap_parameters: SwapShaderParameters::new(),
        }
    }

    fn input_permutation(
        &self,
        commit_state: AllResourcesState,
        resources: &ResourceManager,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        permutation: &Option<&ValidatedPermutation>,
    ) -> Result<AllResourcesState, InsufficientInputError> {
        match permutation {
            Some(permutation) => {
                resources
                    .permutation_input_texture()
                    .load(queue, permutation);
                Ok(commit_state.input_permutation())
            }
            None => {
                if self.flags.check_permutation_output_texture().is_valid() {
                    if !self.flags.check_permutation_output_texture().is_zero()
                        || !self.flags.check_permutation_input_texture().is_zero()
                    {
                        resources
                            .permutation_input_texture()
                            .copy(encoder, resources.permutation_output_texture());
                    }
                    Ok(commit_state.recycle_output_permutation())
                } else if self.flags.check_permutation_input_texture().is_written() {
                    Ok(commit_state)
                } else {
                    unreachable!("there should always be a permutation prior to any operations that require one");
                }
            }
        }
    }

    fn input_image(
        &self,
        commit_state: AllResourcesState,
        resources: &ResourceManager,
        queue: &wgpu::Queue,
        image: &Option<&LosslessImage>,
        accept_missing: bool,
    ) -> Result<AllResourcesState, InsufficientInputError> {
        match image {
            Some(image) => {
                resources.lossless_image_input_texture().load(queue, image);
                Ok(commit_state.input_lossless_image())
            }
            None => {
                if self.flags.check_lossless_image_input_texture() || accept_missing {
                    Ok(commit_state)
                } else {
                    Err(InsufficientInputError::OriginalImage)
                }
            }
        }
    }

    fn input_displacement_goal(
        &self,
        commit_state: AllResourcesState,
        resources: &ResourceManager,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        displacement_goal: &Option<&DisplacementGoal>,
        accept_missing: bool,
    ) -> Result<AllResourcesState, InsufficientInputError> {
        match displacement_goal {
            Some(displacement_goal) => {
                resources
                    .displacement_goal_input_texture()
                    .load(queue, displacement_goal);
                Ok(commit_state.input_displacement_goal())
            }
            None => {
                if self.flags.check_displacement_goal_output_texture() {
                    resources
                        .displacement_goal_input_texture()
                        .copy(encoder, resources.displacement_goal_output_texture());
                    Ok(commit_state.recycle_output_displacement_goal())
                } else if self.flags.check_displacement_goal_input_texture() || accept_missing {
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
    ) -> Result<(), Box<dyn Error>> {
        if self
            .flags
            .check_count_swap_pass_set()
            .contains_set(&sequence)
        {
            if self.count_swap_parameters.update_set(sequence.into()) {
                resources
                    .count_swap_input_layout_buffer()
                    .load(queue, &self.count_swap_parameters)
            }
            self.flags = self
                .flags
                .clone()
                .clear_count_swap_pass_set()
                .finish_count_swap();
            Ok(())
        } else {
            Err(Box::new(InsufficientOutputError::SwapPass))
        }
    }

    pub fn create_displacement_goal(
        &mut self,
        resources: &ResourceManager,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        input: &CreateDisplacementGoalOperationInput,
    ) -> Result<(), Box<dyn Error>> {
        let mut commit_state = self.flags.clone();
        commit_state = self.input_displacement_goal(
            commit_state,
            resources,
            queue,
            encoder,
            &input.displacement_goal,
            true,
        )?;
        commit_state =
            self.input_permutation(commit_state, resources, queue, encoder, &input.permutation)?;
        commit_state = self.input_image(commit_state, resources, queue, &input.image, true)?;
        self.flags = commit_state.create_displacement_goal();
        Ok(())
    }

    pub fn can_skip_create_permutation(&self) -> bool {
        self.flags.check_permutation_input_texture().is_zero()
    }

    pub fn create_permutation(&mut self) -> Result<(), Box<dyn Error>> {
        self.flags = self.flags.clone().finish_create_permutation();
        Ok(())
    }

    pub fn permute(
        &mut self,
        resources: &ResourceManager,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        input: &PermuteOperationInput,
    ) -> Result<(), Box<dyn Error>> {
        let mut commit_state = self.flags.clone().clear_output_lossless_image();
        commit_state =
            self.input_permutation(commit_state, resources, queue, encoder, &input.permutation)?;
        commit_state = self.input_image(commit_state, resources, queue, &input.image, false)?;
        self.flags = commit_state.permute_lossless_image();
        Ok(())
    }

    pub fn swap(
        &mut self,
        resources: &ResourceManager,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        input: &SwapOperationInput,
    ) -> Result<(), Box<dyn Error>> {
        let mut commit_state = self
            .flags
            .clone()
            .clear_output_lossless_image()
            .clear_output_permutation()
            .clear_output_count_swap();
        commit_state =
            self.input_permutation(commit_state, resources, queue, encoder, &input.permutation)?;
        commit_state = self.input_displacement_goal(
            commit_state,
            resources,
            queue,
            encoder,
            &input.displacement_goal,
            false,
        )?;
        self.flags = commit_state.finish_swap(input.pass);

        self.swap_parameters
            .set_pass(input.pass, &self.count_swap_parameters);
        self.swap_parameters
            .set_acceptance_threshold(input.acceptance_threshold);
        resources
            .swap_parameters_buffer()
            .load(queue, &self.swap_parameters);

        Ok(())
    }

    pub fn output_count_swap(
        &mut self,
        resources: &ResourceManager,
        encoder: &mut wgpu::CommandEncoder,
        sequence: &SwapPassSequence,
    ) -> Result<(), Box<dyn Error>> {
        if self.count_swap_parameters.get_set().contains_set(sequence) {
            if self.flags.check_count_swap_output_storage_buffer() {
                if !self.flags.check_count_swap_output_buffer() {
                    resources
                        .count_swap_output_buffer()
                        .load(encoder, resources.count_swap_output_storage_buffer());
                    self.flags = self.flags.clone().output_count_swap();
                }
                Ok(())
            } else {
                unreachable!("no current output swap counts exist");
            }
        } else {
            Err(Box::new(InsufficientOutputError::CountSwap))
        }
    }

    pub fn output_displacement_goal(
        &mut self,
        resources: &ResourceManager,
        encoder: &mut wgpu::CommandEncoder,
    ) -> Result<(), Box<dyn Error>> {
        if self.flags.check_displacement_goal_output_texture() {
            if !self.flags.check_displacement_goal_output_buffer() {
                resources
                    .displacement_goal_output_buffer()
                    .load(encoder, resources.displacement_goal_output_texture());
                self.flags = self.flags.clone().output_displacement_goal();
            }
            Ok(())
        } else {
            Err(Box::new(InsufficientOutputError::DisplacementGoal))
        }
    }

    pub fn output_permutation(
        &mut self,
        resources: &ResourceManager,
        encoder: &mut wgpu::CommandEncoder,
    ) -> Result<(), Box<dyn Error>> {
        if self.flags.check_permutation_output_texture().is_valid() {
            if !self.flags.check_permutation_output_buffer() {
                resources
                    .permutation_output_buffer()
                    .load(encoder, resources.permutation_output_texture());
                self.flags = self.flags.clone().output_permutation();
            }
            Ok(())
        } else {
            Err(Box::new(InsufficientOutputError::Permutation))
        }
    }

    pub fn output_permuted_image(
        &mut self,
        resources: &ResourceManager,
        encoder: &mut wgpu::CommandEncoder,
    ) -> Result<(), Box<dyn Error>> {
        if self.flags.check_lossless_image_output_texture() {
            if !self.flags.check_lossless_image_output_buffer() {
                resources
                    .lossless_image_output_buffer()
                    .load(encoder, resources.lossless_image_output_texture());
                self.flags = self.flags.clone().output_lossless_image();
            }
            Ok(())
        } else {
            Err(Box::new(InsufficientOutputError::PermutedImage))
        }
    }
}
