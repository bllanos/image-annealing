use std::default::Default;
use std::num::NonZeroU32;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct WorkgroupDimensions(NonZeroU32, NonZeroU32, NonZeroU32);

impl WorkgroupDimensions {
    pub fn x(&self) -> u32 {
        self.0.get()
    }

    pub fn y(&self) -> u32 {
        self.1.get()
    }

    pub fn z(&self) -> u32 {
        self.2.get()
    }
}

impl Default for WorkgroupDimensions {
    fn default() -> Self {
        Self(
            NonZeroU32::new(16).unwrap(),
            NonZeroU32::new(16).unwrap(),
            NonZeroU32::new(1).unwrap(),
        )
    }
}
