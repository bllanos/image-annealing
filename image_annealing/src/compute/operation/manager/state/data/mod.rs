use super::super::super::super::link::swap::{SwapPass, SwapPassSet};

mod resource;

use resource::ResourceStateMachineWrapper;

#[must_use]
#[derive(Clone)]
pub struct AllResourcesState {
    count_swap_pass_set: SwapPassSet,
    count_swap_output_storage_buffer: ResourceStateMachineWrapper,
    count_swap_output_buffer: ResourceStateMachineWrapper,
    displacement_goal_input_texture: ResourceStateMachineWrapper,
    permutation_input_texture: ResourceStateMachineWrapper,
    permutation_output_texture: ResourceStateMachineWrapper,
    permutation_output_buffer: ResourceStateMachineWrapper,
    lossless_image_input_texture: ResourceStateMachineWrapper,
    lossless_image_output_texture: ResourceStateMachineWrapper,
    lossless_image_output_buffer: ResourceStateMachineWrapper,
}

impl AllResourcesState {
    pub fn new() -> Self {
        Self {
            count_swap_pass_set: Default::default(),
            count_swap_output_storage_buffer: ResourceStateMachineWrapper::new(),
            count_swap_output_buffer: ResourceStateMachineWrapper::new(),
            displacement_goal_input_texture: ResourceStateMachineWrapper::new(),
            permutation_input_texture: ResourceStateMachineWrapper::new(),
            permutation_output_texture: ResourceStateMachineWrapper::new(),
            permutation_output_buffer: ResourceStateMachineWrapper::new(),
            lossless_image_input_texture: ResourceStateMachineWrapper::new(),
            lossless_image_output_texture: ResourceStateMachineWrapper::new(),
            lossless_image_output_buffer: ResourceStateMachineWrapper::new(),
        }
    }

    pub fn check_count_swap_pass_set(&self) -> SwapPassSet {
        self.count_swap_pass_set
    }

    pub fn check_count_swap_output_storage_buffer(&self) -> bool {
        self.count_swap_output_storage_buffer.is_written()
    }

    pub fn check_count_swap_output_buffer(&self) -> bool {
        self.count_swap_output_buffer.is_written()
    }

    pub fn check_displacement_goal_input_texture(&self) -> &ResourceStateMachineWrapper {
        &self.displacement_goal_input_texture
    }

    pub fn check_permutation_input_texture(&self) -> &ResourceStateMachineWrapper {
        &self.permutation_input_texture
    }

    pub fn check_permutation_output_texture(&self) -> &ResourceStateMachineWrapper {
        &self.permutation_output_texture
    }

    pub fn check_permutation_output_buffer(&self) -> bool {
        self.permutation_output_buffer.is_written()
            || (self.permutation_output_texture.is_zero()
                && self.permutation_output_buffer.is_zero())
    }

    pub fn check_lossless_image_input_texture(&self) -> bool {
        self.lossless_image_input_texture.is_written()
    }

    pub fn check_lossless_image_output_texture(&self) -> bool {
        self.lossless_image_output_texture.is_written()
    }

    pub fn check_lossless_image_output_buffer(&self) -> bool {
        self.lossless_image_output_buffer.is_written()
    }

    pub fn clear_output_count_swap(self) -> Self {
        Self {
            count_swap_output_storage_buffer: self.count_swap_output_storage_buffer.clear(),
            count_swap_output_buffer: self.count_swap_output_buffer.clear(),
            ..self
        }
    }

    pub fn clear_count_swap_pass_set(self) -> Self {
        Self {
            count_swap_pass_set: Default::default(),
            ..self.clear_output_count_swap()
        }
    }

    pub fn clear_output_permutation(self) -> Self {
        Self {
            permutation_output_texture: self.permutation_output_texture.clear(),
            permutation_output_buffer: self.permutation_output_buffer.clear(),
            ..self
        }
    }

    pub fn clear_output_lossless_image(self) -> Self {
        Self {
            lossless_image_output_texture: self.lossless_image_output_texture.clear(),
            lossless_image_output_buffer: self.lossless_image_output_buffer.clear(),
            ..self
        }
    }

    pub fn input_displacement_goal(self) -> Self {
        Self {
            displacement_goal_input_texture: self.displacement_goal_input_texture.write(),
            ..self
        }
    }

    pub fn input_permutation(self) -> Self {
        let mut next = self.clear_output_permutation().clear_count_swap_pass_set();
        next.permutation_input_texture = next.permutation_input_texture.write();
        next
    }

    pub fn input_lossless_image(self) -> Self {
        let mut next = self.clear_output_lossless_image();
        next.lossless_image_input_texture = next.lossless_image_input_texture.write();
        next
    }

    pub fn finish_count_swap(self) -> Self {
        let mut next = self.clear_count_swap_pass_set();
        next.count_swap_output_storage_buffer = next.count_swap_output_storage_buffer.write();
        next
    }

    fn prepare_create_permutation(self) -> Self {
        let mut next = self
            .clear_output_lossless_image()
            .clear_count_swap_pass_set();
        if next.permutation_output_texture.is_written() {
            next.permutation_output_texture = next.permutation_output_texture.clear();
        }
        if next.permutation_output_buffer.is_written() {
            next.permutation_output_buffer = next.permutation_output_buffer.clear();
        }
        if !next.permutation_input_texture.is_zero() {
            next.permutation_input_texture = next.permutation_input_texture.clear();
        }
        next
    }

    pub fn finish_create_permutation(self) -> Self {
        let mut next = self.prepare_create_permutation();
        next.permutation_output_texture = ResourceStateMachineWrapper::new();
        next
    }

    pub fn permute_lossless_image(self) -> Self {
        let mut next = self.clear_output_lossless_image();
        next.lossless_image_output_texture = next.lossless_image_output_texture.write();
        next
    }

    pub fn finish_swap(self, pass: SwapPass) -> Self {
        let mut next = self
            .clear_output_permutation()
            .clear_output_lossless_image()
            .clear_output_count_swap();
        next.permutation_input_texture = next.permutation_input_texture.clear();
        next.permutation_output_texture = next.permutation_output_texture.write();
        next.count_swap_pass_set = next.count_swap_pass_set.add_pass(pass);
        next
    }

    pub fn recycle_output_permutation(self) -> Self {
        Self {
            permutation_input_texture: self.permutation_output_texture.clone(),
            ..self
        }
    }

    pub fn output_count_swap(self) -> Self {
        Self {
            count_swap_output_buffer: self.count_swap_output_storage_buffer.clone(),
            ..self
        }
    }

    pub fn output_permutation(self) -> Self {
        Self {
            permutation_output_buffer: self.permutation_output_texture.clone(),
            ..self
        }
    }

    pub fn output_lossless_image(self) -> Self {
        Self {
            lossless_image_output_buffer: self.lossless_image_output_texture.clone(),
            ..self
        }
    }
}
