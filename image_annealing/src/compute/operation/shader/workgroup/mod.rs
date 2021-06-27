use image_annealing_shaders::WorkgroupDimensions;
use std::default::Default;
use std::num::NonZeroU32;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct WorkgroupGridDimensions(NonZeroU32, NonZeroU32, NonZeroU32);

impl From<wgpu::Extent3d> for WorkgroupGridDimensions {
    fn from(extent: wgpu::Extent3d) -> Self {
        let wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: depth,
        } = extent;
        let workgroup_dimensions: WorkgroupDimensions = Default::default();
        let remainder = (
            width % workgroup_dimensions.x(),
            height % workgroup_dimensions.y(),
            depth % workgroup_dimensions.z(),
        );
        let quotient = (
            width / workgroup_dimensions.x(),
            height / workgroup_dimensions.y(),
            depth / workgroup_dimensions.z(),
        );
        Self(
            NonZeroU32::new(if remainder.0 == 0 {
                quotient.0
            } else {
                quotient.0 + 1
            })
            .unwrap(),
            NonZeroU32::new(if remainder.1 == 0 {
                quotient.1
            } else {
                quotient.1 + 1
            })
            .unwrap(),
            NonZeroU32::new(if remainder.2 == 0 {
                quotient.2
            } else {
                quotient.2 + 1
            })
            .unwrap(),
        )
    }
}

impl WorkgroupGridDimensions {
    pub fn x(&self) -> u32 {
        self.0.get()
    }

    pub fn y(&self) -> u32 {
        self.1.get()
    }

    pub fn z(&self) -> u32 {
        self.2.get()
    }

    pub fn dispatch(&self, cpass: &mut wgpu::ComputePass) {
        cpass.dispatch(self.x(), self.y(), self.z());
    }
}

#[cfg(test)]
mod tests;
