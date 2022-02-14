#[must_use]
#[derive(Copy, Clone)]
pub struct ResourceStateFlags {
    displacement_goal_input_texture: bool,
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
            displacement_goal_input_texture: false,
            permutation_input_texture: false,
            permutation_output_texture: false,
            permutation_output_buffer: false,
            lossless_image_input_texture: false,
            lossless_image_output_texture: false,
            lossless_image_output_buffer: false,
        }
    }

    pub fn check_displacement_goal_input_texture(&self) -> bool {
        self.displacement_goal_input_texture
    }

    pub fn check_permutation_input_texture(&self) -> bool {
        self.permutation_input_texture
    }

    pub fn check_permutation_output_texture(&self) -> bool {
        self.permutation_output_texture
    }

    pub fn check_permutation_output_buffer(&self) -> bool {
        self.permutation_output_buffer
    }

    pub fn check_lossless_image_input_texture(&self) -> bool {
        self.lossless_image_input_texture
    }

    pub fn check_lossless_image_output_texture(&self) -> bool {
        self.lossless_image_output_texture
    }

    pub fn check_lossless_image_output_buffer(&self) -> bool {
        self.lossless_image_output_buffer
    }

    pub fn clear_output_permutation(&self) -> Self {
        Self {
            permutation_output_texture: false,
            permutation_output_buffer: false,
            ..*self
        }
    }

    pub fn clear_output_lossless_image(&self) -> Self {
        Self {
            lossless_image_output_texture: false,
            lossless_image_output_buffer: false,
            ..*self
        }
    }

    pub fn input_displacement_goal(&self) -> Self {
        Self {
            displacement_goal_input_texture: true,
            ..*self
        }
    }

    pub fn input_permutation(&self) -> Self {
        let mut next = self.clear_output_permutation();
        next.permutation_input_texture = true;
        next
    }

    pub fn input_lossless_image(&self) -> Self {
        let mut next = self.clear_output_lossless_image();
        next.lossless_image_input_texture = true;
        next
    }

    pub fn prepare_create_permutation(&self) -> Self {
        let mut next = self
            .clear_output_permutation()
            .clear_output_lossless_image();
        next.permutation_input_texture = false;
        next
    }

    pub fn finish_create_permutation(&self) -> Self {
        let mut next = self.prepare_create_permutation();
        next.permutation_output_texture = true;
        next
    }

    pub fn permute_lossless_image(&self) -> Self {
        let mut next = self.clear_output_lossless_image();
        next.lossless_image_output_texture = true;
        next
    }

    pub fn recycle_output_permutation(&self) -> Self {
        Self {
            permutation_input_texture: self.permutation_output_texture,
            ..*self
        }
    }

    pub fn output_permutation(&self) -> Self {
        Self {
            permutation_output_buffer: self.permutation_output_texture,
            ..*self
        }
    }

    pub fn output_lossless_image(&self) -> Self {
        Self {
            lossless_image_output_buffer: self.lossless_image_output_texture,
            ..*self
        }
    }
}
