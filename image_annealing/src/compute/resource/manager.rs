use super::buffer::{
    CountSwapInputBuffer, CountSwapInputLayoutBuffer, CountSwapOutputBuffer,
    CountSwapOutputStorageBuffer, LosslessImageOutputBuffer, PermutationOutputBuffer,
    SwapParametersBuffer,
};
use super::texture::{
    DisplacementGoalInputTexture, LosslessImageInputTexture, LosslessImageOutputTexture,
    PermutationInputTexture, PermutationOutputTexture,
};
use crate::ImageDimensions;

pub struct ResourceManager {
    count_swap_input_buffer: CountSwapInputBuffer,
    count_swap_input_layout_buffer: CountSwapInputLayoutBuffer,
    count_swap_output_buffer: CountSwapOutputBuffer,
    count_swap_output_storage_buffer: CountSwapOutputStorageBuffer,
    displacement_goal_input_texture: DisplacementGoalInputTexture,
    permutation_input_texture: PermutationInputTexture,
    permutation_output_texture: PermutationOutputTexture,
    permutation_output_buffer: PermutationOutputBuffer,
    lossless_image_input_texture: LosslessImageInputTexture,
    lossless_image_output_texture: LosslessImageOutputTexture,
    lossless_image_output_buffer: LosslessImageOutputBuffer,
    swap_parameters_buffer: SwapParametersBuffer,
}

impl ResourceManager {
    pub fn new(device: &wgpu::Device, image_dimensions: &ImageDimensions) -> Self {
        let count_swap_output_buffer = CountSwapOutputBuffer::new(device);
        let count_swap_output_storage_buffer =
            CountSwapOutputStorageBuffer::new(device, &count_swap_output_buffer);
        Self {
            count_swap_input_buffer: CountSwapInputBuffer::new(device, image_dimensions),
            count_swap_input_layout_buffer: CountSwapInputLayoutBuffer::new(device),
            count_swap_output_buffer,
            count_swap_output_storage_buffer,
            displacement_goal_input_texture: DisplacementGoalInputTexture::new(
                device,
                image_dimensions,
            ),
            permutation_input_texture: PermutationInputTexture::new(device, image_dimensions),
            permutation_output_texture: PermutationOutputTexture::new(device, image_dimensions),
            permutation_output_buffer: PermutationOutputBuffer::new(device, image_dimensions),
            lossless_image_input_texture: LosslessImageInputTexture::new(device, image_dimensions),
            lossless_image_output_texture: LosslessImageOutputTexture::new(
                device,
                image_dimensions,
            ),
            lossless_image_output_buffer: LosslessImageOutputBuffer::new(device, image_dimensions),
            swap_parameters_buffer: SwapParametersBuffer::new(device),
        }
    }

    pub fn count_swap_input_buffer(&self) -> &CountSwapInputBuffer {
        &self.count_swap_input_buffer
    }

    pub fn count_swap_input_layout_buffer(&self) -> &CountSwapInputLayoutBuffer {
        &self.count_swap_input_layout_buffer
    }

    pub fn count_swap_output_buffer(&self) -> &CountSwapOutputBuffer {
        &self.count_swap_output_buffer
    }

    pub fn count_swap_output_storage_buffer(&self) -> &CountSwapOutputStorageBuffer {
        &self.count_swap_output_storage_buffer
    }

    pub fn displacement_goal_input_texture(&self) -> &DisplacementGoalInputTexture {
        &self.displacement_goal_input_texture
    }

    pub fn permutation_input_texture(&self) -> &PermutationInputTexture {
        &self.permutation_input_texture
    }

    pub fn permutation_output_texture(&self) -> &PermutationOutputTexture {
        &self.permutation_output_texture
    }

    pub fn permutation_output_buffer(&self) -> &PermutationOutputBuffer {
        &self.permutation_output_buffer
    }

    pub fn lossless_image_input_texture(&self) -> &LosslessImageInputTexture {
        &self.lossless_image_input_texture
    }

    pub fn lossless_image_output_texture(&self) -> &LosslessImageOutputTexture {
        &self.lossless_image_output_texture
    }

    pub fn lossless_image_output_buffer(&self) -> &LosslessImageOutputBuffer {
        &self.lossless_image_output_buffer
    }

    pub fn swap_parameters_buffer(&self) -> &SwapParametersBuffer {
        &self.swap_parameters_buffer
    }
}
