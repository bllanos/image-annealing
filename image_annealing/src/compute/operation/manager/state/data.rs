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

    pub fn clear_output_permutation(&mut self) {
        self.permutation_output_texture = false;
        self.permutation_output_buffer = false;
    }

    pub fn clear_output_lossless_image(&mut self) {
        self.lossless_image_output_texture = false;
        self.lossless_image_output_buffer = false;
    }

    pub fn input_displacement_goal(&mut self) {
        self.displacement_goal_input_texture = true;
    }

    pub fn input_permutation(&mut self) {
        self.clear_output_permutation();
        self.permutation_input_texture = true;
    }

    pub fn input_lossless_image(&mut self) {
        self.clear_output_lossless_image();
        self.lossless_image_input_texture = true;
    }

    pub fn create_permutation(&mut self) {
        self.clear_output_permutation();
        self.clear_output_lossless_image();
        self.permutation_input_texture = false;
        self.permutation_output_texture = true;
    }

    pub fn permute_lossless_image(&mut self) {
        self.clear_output_lossless_image();
        self.lossless_image_output_texture = true;
    }

    pub fn recycle_output_permutation(&mut self) {
        self.permutation_input_texture = self.permutation_output_texture;
    }

    pub fn output_permutation(&mut self) {
        self.permutation_output_buffer = self.permutation_output_texture;
    }

    pub fn output_lossless_image(&mut self) {
        self.lossless_image_output_buffer = self.lossless_image_output_texture;
    }
}
