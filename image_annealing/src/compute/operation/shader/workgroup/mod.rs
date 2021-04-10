#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct WorkgroupDimensions(pub u32, pub u32, pub u32);

const WORKGROUP_SIZE: WorkgroupDimensions = WorkgroupDimensions(32, 32, 1);

impl WorkgroupDimensions {
    pub fn new(width: u32, height: u32) -> Self {
        debug_assert!(!(width == 0 || height == 0));
        let remainder = (width % WORKGROUP_SIZE.0, height % WORKGROUP_SIZE.1);
        Self(
            if remainder.0 == 0 {
                width
            } else {
                (width - remainder.0)
                    .checked_add(WORKGROUP_SIZE.0)
                    .expect("Integer overflow")
            },
            if remainder.1 == 0 {
                height
            } else {
                (height - remainder.1)
                    .checked_add(WORKGROUP_SIZE.1)
                    .expect("Integer overflow")
            },
            1,
        )
    }
}

#[cfg(test)]
mod tests;
