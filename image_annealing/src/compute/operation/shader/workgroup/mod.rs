#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct WorkgroupDimensions(pub u32, pub u32, pub u32);

const DEFAULT_WORKGROUP_DIMENSIONS: WorkgroupDimensions = WorkgroupDimensions(32, 32, 1);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct WorkgroupGridDimensions(pub u32, pub u32, pub u32);

impl WorkgroupGridDimensions {
    pub fn new(extent: wgpu::Extent3d) -> Self {
        let wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: depth,
        } = extent;
        assert!(!(width == 0 || height == 0 || depth == 0));
        let remainder = (
            width % DEFAULT_WORKGROUP_DIMENSIONS.0,
            height % DEFAULT_WORKGROUP_DIMENSIONS.1,
            depth % DEFAULT_WORKGROUP_DIMENSIONS.2,
        );
        let quotient = (
            width / DEFAULT_WORKGROUP_DIMENSIONS.0,
            height / DEFAULT_WORKGROUP_DIMENSIONS.1,
            depth / DEFAULT_WORKGROUP_DIMENSIONS.2,
        );
        Self(
            if remainder.0 == 0 {
                quotient.0
            } else {
                quotient.0 + 1
            },
            if remainder.1 == 0 {
                quotient.1
            } else {
                quotient.1 + 1
            },
            if remainder.2 == 0 {
                quotient.2
            } else {
                quotient.2 + 1
            },
        )
    }
}

#[cfg(test)]
mod tests;
