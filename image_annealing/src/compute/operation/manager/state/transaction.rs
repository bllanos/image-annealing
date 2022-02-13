use super::super::super::super::resource::manager::ResourceManager;
use super::super::{PermuteOperationInput, SwapOperationInput};
use super::data::ResourceStateFlags;
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

pub struct ResourceStateManager {
    flags: ResourceStateFlags,
}

impl ResourceStateManager {
    pub fn new() -> Self {
        Self {
            flags: ResourceStateFlags::new(),
        }
    }

    pub fn prepare_create_permutation(&mut self) -> Result<(), Box<dyn Error>> {
        self.flags.create_permutation();
        Ok(())
    }

    pub fn finish_create_permutation(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn prepare_permute(
        &mut self,
        resources: &ResourceManager,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        input: &PermuteOperationInput,
    ) -> Result<(), Box<dyn Error>> {
        self.flags.clear_output_lossless_image();
        let mut new_flags = self.flags;
        match input.permutation {
            Some(permutation) => {
                resources
                    .permutation_input_texture()
                    .load(queue, permutation);
                new_flags.input_permutation();
            }
            None => {
                if self.flags.check_permutation_output_texture() {
                    resources
                        .permutation_input_texture()
                        .copy(encoder, resources.permutation_output_texture());
                    new_flags.recycle_output_permutation();
                } else if !self.flags.check_permutation_input_texture() {
                    return Err(Box::new(InsufficientInputError::Permutation));
                }
            }
        }
        match input.image {
            Some(image) => {
                resources.lossless_image_input_texture().load(queue, image);
                new_flags.input_lossless_image();
            }
            None => {
                if !self.flags.check_lossless_image_input_texture() {
                    return Err(Box::new(InsufficientInputError::OriginalImage));
                }
            }
        }
        self.flags = new_flags;
        Ok(())
    }

    pub fn finish_permute(&mut self) -> Result<(), Box<dyn Error>> {
        self.flags.permute_lossless_image();
        Ok(())
    }

    pub fn prepare_swap(
        &mut self,
        resources: &ResourceManager,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        input: &SwapOperationInput,
    ) -> Result<(), Box<dyn Error>> {
        self.flags.clear_output_lossless_image();
        let had_output_permutation = self.flags.check_permutation_output_texture();
        self.flags.clear_output_permutation();
        let mut new_flags = self.flags;
        match input.permutation {
            Some(permutation) => {
                resources
                    .permutation_input_texture()
                    .load(queue, permutation);
                new_flags.input_permutation();
            }
            None => {
                if had_output_permutation {
                    resources
                        .permutation_input_texture()
                        .copy(encoder, resources.permutation_output_texture());
                    new_flags.recycle_output_permutation();
                } else if !self.flags.check_permutation_input_texture() {
                    return Err(Box::new(InsufficientInputError::Permutation));
                }
            }
        }
        match input.displacement_goal {
            Some(displacement_goal) => {
                resources
                    .displacement_goal_input_texture()
                    .load(queue, displacement_goal);
                new_flags.input_displacement_goal();
            }
            None => {
                if !self.flags.check_displacement_goal_input_texture() {
                    return Err(Box::new(InsufficientInputError::DisplacementGoal));
                }
            }
        }
        self.flags = new_flags;
        Ok(())
    }

    pub fn finish_swap(&mut self) -> Result<(), Box<dyn Error>> {
        self.flags.create_permutation();
        Ok(())
    }

    pub fn output_permutation(
        &mut self,
        resources: &ResourceManager,
        encoder: &mut wgpu::CommandEncoder,
    ) -> Result<(), Box<dyn Error>> {
        if self.flags.check_permutation_output_texture() {
            if !self.flags.check_permutation_output_buffer() {
                resources
                    .permutation_output_buffer()
                    .load(encoder, resources.permutation_output_texture());
                self.flags.output_permutation();
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
                self.flags.output_lossless_image();
            }
            Ok(())
        } else {
            Err(Box::new(InsufficientOutputError::PermutedImage))
        }
    }
}
