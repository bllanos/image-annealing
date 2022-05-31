use crate::ImageDimensions;
use image_annealing_shaders::WorkgroupDimensions;
use std::num::NonZeroU32;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct WorkgroupGridDimensions(NonZeroU32, NonZeroU32, NonZeroU32);

impl WorkgroupGridDimensions {
    pub fn from_image_dimensions_and_stride(
        workgroup_dimensions: &WorkgroupDimensions,
        image_dimensions: &ImageDimensions,
        x_stride: NonZeroU32,
        y_stride: NonZeroU32,
    ) -> Self {
        Self::from_extent_and_stride(
            workgroup_dimensions,
            image_dimensions.to_extent(),
            x_stride,
            y_stride,
        )
    }

    pub fn from_extent_and_stride(
        workgroup_dimensions: &WorkgroupDimensions,
        extent: wgpu::Extent3d,
        x_stride: NonZeroU32,
        y_stride: NonZeroU32,
    ) -> Self {
        let wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers,
        } = extent;
        let remainder = (width % x_stride, height % y_stride);
        let quotient = (width / x_stride, height / y_stride);
        Self::from_extent(
            workgroup_dimensions,
            wgpu::Extent3d {
                width: if remainder.0 == 0 {
                    quotient.0
                } else {
                    quotient.0 + 1
                },
                height: if remainder.1 == 0 {
                    quotient.1
                } else {
                    quotient.1 + 1
                },
                depth_or_array_layers,
            },
        )
    }

    pub fn from_extent(workgroup_dimensions: &WorkgroupDimensions, extent: wgpu::Extent3d) -> Self {
        let wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: depth,
        } = extent;
        let remainder = (
            width.checked_rem_euclid(workgroup_dimensions.x()).unwrap(),
            height.checked_rem_euclid(workgroup_dimensions.y()).unwrap(),
            depth.checked_rem_euclid(workgroup_dimensions.z()).unwrap(),
        );
        let quotient = (
            width.checked_div_euclid(workgroup_dimensions.x()).unwrap(),
            height.checked_div_euclid(workgroup_dimensions.y()).unwrap(),
            depth.checked_div_euclid(workgroup_dimensions.z()).unwrap(),
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

    pub fn count_swap() -> Self {
        let one = NonZeroU32::new(1).unwrap();
        Self(one, one, one)
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

    pub fn count(&self) -> usize {
        <usize as TryFrom<u32>>::try_from(self.x())
            .unwrap()
            .checked_mul(<usize as TryFrom<u32>>::try_from(self.y()).unwrap())
            .unwrap()
            .checked_mul(<usize as TryFrom<u32>>::try_from(self.z()).unwrap())
            .unwrap()
    }

    pub fn dispatch(&self, cpass: &mut wgpu::ComputePass) {
        cpass.dispatch(self.x(), self.y(), self.z());
    }
}

#[cfg(test)]
mod tests;
