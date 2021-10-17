use super::super::super::resource::manager::ResourceManager;
use super::{PermuteOperationInput, SwapOperationInput};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum InsufficientInputError {
    Permutation,
    OriginalImage,
}

impl fmt::Display for InsufficientInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "an input {} must be provided as there is none to reuse",
            match self {
                InsufficientInputError::Permutation => "permutation",
                InsufficientInputError::OriginalImage => "image",
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

struct ResourceStateFlags {
    permutation_input_texture: bool,
    permutation_output_texture: bool,
    permutation_output_buffer: bool,
    lossless_image_input_texture: bool,
    lossless_image_output_texture: bool,
    lossless_image_output_buffer: bool,
}

impl ResourceStateFlags {
    pub fn new() -> Self {
        Self {
            permutation_input_texture: false,
            permutation_output_texture: false,
            permutation_output_buffer: false,
            lossless_image_input_texture: false,
            lossless_image_output_texture: false,
            lossless_image_output_buffer: false,
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

    pub fn prepare_create_permutation(&mut self) -> Result<(), Box<dyn Error>> {
        self.flags.permutation_input_texture = false;
        self.flags.permutation_output_texture = false;
        self.flags.permutation_output_buffer = false;
        self.flags.lossless_image_output_texture = false;
        self.flags.lossless_image_output_buffer = false;
        Ok(())
    }

    pub fn finish_create_permutation(&mut self) -> Result<(), Box<dyn Error>> {
        self.flags.permutation_output_texture = true;
        Ok(())
    }

    pub fn prepare_permute(
        &mut self,
        resources: &ResourceManager,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        input: &PermuteOperationInput,
    ) -> Result<(), Box<dyn Error>> {
        self.flags.lossless_image_output_texture = false;
        self.flags.lossless_image_output_buffer = false;
        match input.permutation {
            Some(permutation) => {
                resources
                    .permutation_input_texture()
                    .load(queue, permutation);
                self.flags.permutation_input_texture = true;
                self.flags.permutation_output_texture = false;
                self.flags.permutation_output_buffer = false;
            }
            None => {
                if !self.flags.permutation_input_texture {
                    if self.flags.permutation_output_texture {
                        resources
                            .permutation_input_texture()
                            .copy(encoder, resources.permutation_output_texture());
                        self.flags.permutation_input_texture = true;
                    } else {
                        return Err(Box::new(InsufficientInputError::Permutation));
                    }
                }
            }
        }
        match input.image {
            Some(image) => {
                resources.lossless_image_input_texture().load(queue, image);
                self.flags.lossless_image_input_texture = true;
            }
            None => {
                if !self.flags.lossless_image_input_texture {
                    return Err(Box::new(InsufficientInputError::OriginalImage));
                }
            }
        }
        Ok(())
    }

    pub fn finish_permute(&mut self) -> Result<(), Box<dyn Error>> {
        self.flags.lossless_image_output_texture = true;
        Ok(())
    }

    pub fn prepare_swap(
        &mut self,
        resources: &ResourceManager,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        input: &SwapOperationInput,
    ) -> Result<(), Box<dyn Error>> {
        self.flags.lossless_image_output_texture = false;
        self.flags.lossless_image_output_buffer = false;
        let had_output_permutation = self.flags.permutation_output_texture;
        self.flags.permutation_output_texture = false;
        self.flags.permutation_output_buffer = false;
        match input.permutation {
            Some(permutation) => {
                resources
                    .permutation_input_texture()
                    .load(queue, permutation);
                self.flags.permutation_input_texture = true;
            }
            None => {
                if !self.flags.permutation_input_texture {
                    if had_output_permutation {
                        resources
                            .permutation_input_texture()
                            .copy(encoder, resources.permutation_output_texture());
                        self.flags.permutation_input_texture = true;
                    } else {
                        return Err(Box::new(InsufficientInputError::Permutation));
                    }
                }
            }
        }
        Ok(())
    }

    pub fn finish_swap(&mut self) -> Result<(), Box<dyn Error>> {
        self.flags.permutation_output_texture = true;
        Ok(())
    }

    pub fn output_permutation(
        &mut self,
        resources: &ResourceManager,
        encoder: &mut wgpu::CommandEncoder,
    ) -> Result<(), Box<dyn Error>> {
        if self.flags.permutation_output_texture {
            if !self.flags.permutation_output_buffer {
                resources
                    .permutation_output_buffer()
                    .load(encoder, resources.permutation_output_texture());
                self.flags.permutation_output_buffer = true;
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
        if self.flags.lossless_image_output_texture {
            if !self.flags.lossless_image_output_buffer {
                resources
                    .lossless_image_output_buffer()
                    .load(encoder, resources.lossless_image_output_texture());
                self.flags.lossless_image_output_buffer = true;
            }
            Ok(())
        } else {
            Err(Box::new(InsufficientOutputError::PermutedImage))
        }
    }
}
