use std::num::NonZeroU32;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct WorkgroupDimensions(pub NonZeroU32, pub NonZeroU32, pub NonZeroU32);

impl WorkgroupDimensions {
    fn texture_patch() -> Self {
        Self(
            NonZeroU32::new(16).unwrap(),
            NonZeroU32::new(16).unwrap(),
            NonZeroU32::new(1).unwrap(),
        )
    }

    fn horizontal_line(length: u32) -> Self {
        Self(
            NonZeroU32::new(length).unwrap(),
            NonZeroU32::new(1).unwrap(),
            NonZeroU32::new(1).unwrap(),
        )
    }

    pub fn count_swap() -> Self {
        Self::horizontal_line(256u32)
    }

    pub fn create_displacement_goal_default() -> Self {
        Self::texture_patch()
    }

    pub fn create_permutation() -> Self {
        Self::texture_patch()
    }

    pub fn permute() -> Self {
        Self::texture_patch()
    }

    pub fn swap() -> Self {
        Self::texture_patch()
    }

    pub fn x(&self) -> u32 {
        self.0.get()
    }

    pub fn y(&self) -> u32 {
        self.1.get()
    }

    pub fn z(&self) -> u32 {
        self.2.get()
    }

    pub fn invocation_count(&self) -> u32 {
        self.x() * self.y() * self.z()
    }
}

#[cfg(test)]
mod tests;
