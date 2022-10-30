use image_annealing_shaders::WorkgroupDimensions;

fn create_workgroup_dimensions() -> WorkgroupDimensions {
    let workgroup_dimensions = WorkgroupDimensions::create_permutation();
    assert!(workgroup_dimensions.x() >= 2);
    assert!(workgroup_dimensions.y() >= 2);
    assert!(workgroup_dimensions.z() >= 1);
    workgroup_dimensions
}

mod from_image_dimensions_and_stride {
    use super::super::WorkgroupGridDimensions;
    use crate::ImageDimensions;
    use std::error::Error;
    use std::num::NonZeroU32;

    #[test]
    fn nondivisible_small_extent() -> Result<(), Box<dyn Error>> {
        let workgroup_dimensions = super::create_workgroup_dimensions();
        let image_dimensions = ImageDimensions::try_new(
            workgroup_dimensions.x() * 2 - 1,
            workgroup_dimensions.y() * 2 - 1,
        )?;
        let x_stride = NonZeroU32::new(2).unwrap();
        let y_stride = NonZeroU32::new(2).unwrap();
        let expected = WorkgroupGridDimensions::from_extent_and_stride(
            &workgroup_dimensions,
            image_dimensions.to_extent(),
            x_stride,
            y_stride,
        );
        let actual = WorkgroupGridDimensions::from_image_dimensions_and_stride(
            &workgroup_dimensions,
            &image_dimensions,
            x_stride,
            y_stride,
        );
        assert_eq!(actual, expected);
        Ok(())
    }
}

mod from_extent_and_stride {
    use super::super::WorkgroupGridDimensions;
    use std::num::NonZeroU32;
    use wgpu::Extent3d;

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn zero_depth() {
        let workgroup_dimensions = super::create_workgroup_dimensions();
        let _ = WorkgroupGridDimensions::from_extent_and_stride(
            &workgroup_dimensions,
            Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 0,
            },
            NonZeroU32::new(1).unwrap(),
            NonZeroU32::new(1).unwrap(),
        );
    }

    #[test]
    fn stride_ones() {
        let workgroup_dimensions = super::create_workgroup_dimensions();
        let extent = Extent3d {
            width: workgroup_dimensions.x() * 2,
            height: workgroup_dimensions.y() * 2,
            depth_or_array_layers: workgroup_dimensions.z(),
        };
        let expected = WorkgroupGridDimensions::from_extent(&workgroup_dimensions, extent);
        let actual = WorkgroupGridDimensions::from_extent_and_stride(
            &workgroup_dimensions,
            extent,
            NonZeroU32::new(1).unwrap(),
            NonZeroU32::new(1).unwrap(),
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn stride_divisible_x() {
        let workgroup_dimensions = super::create_workgroup_dimensions();
        let dim = WorkgroupGridDimensions::from_extent_and_stride(
            &workgroup_dimensions,
            Extent3d {
                width: workgroup_dimensions.x() * 4,
                height: workgroup_dimensions.y() * 4,
                depth_or_array_layers: workgroup_dimensions.z(),
            },
            NonZeroU32::new(2).unwrap(),
            NonZeroU32::new(5).unwrap(),
        );
        assert_eq!(dim.x(), 2);
        assert_eq!(dim.y(), 1);
        assert_eq!(dim.z(), 1);
    }

    #[test]
    fn stride_divisible_y() {
        let workgroup_dimensions = super::create_workgroup_dimensions();
        let dim = WorkgroupGridDimensions::from_extent_and_stride(
            &workgroup_dimensions,
            Extent3d {
                width: workgroup_dimensions.x() * 4,
                height: workgroup_dimensions.y() * 4,
                depth_or_array_layers: workgroup_dimensions.z(),
            },
            NonZeroU32::new(5).unwrap(),
            NonZeroU32::new(2).unwrap(),
        );
        assert_eq!(dim.x(), 1);
        assert_eq!(dim.y(), 2);
        assert_eq!(dim.z(), 1);
    }

    #[test]
    fn near_small_extent() {
        let workgroup_dimensions = super::create_workgroup_dimensions();
        let dim = WorkgroupGridDimensions::from_extent_and_stride(
            &workgroup_dimensions,
            Extent3d {
                width: workgroup_dimensions.x() * 2 + 1,
                height: workgroup_dimensions.y() * 2 + 1,
                depth_or_array_layers: workgroup_dimensions.z(),
            },
            NonZeroU32::new(2).unwrap(),
            NonZeroU32::new(2).unwrap(),
        );
        assert_eq!(dim.x(), 2);
        assert_eq!(dim.y(), 2);
        assert_eq!(dim.z(), 1);
    }

    #[test]
    fn divisible_small_extent() {
        let workgroup_dimensions = super::create_workgroup_dimensions();
        let dim = WorkgroupGridDimensions::from_extent_and_stride(
            &workgroup_dimensions,
            Extent3d {
                width: workgroup_dimensions.x() * 2,
                height: workgroup_dimensions.y() * 2,
                depth_or_array_layers: workgroup_dimensions.z(),
            },
            NonZeroU32::new(2).unwrap(),
            NonZeroU32::new(2).unwrap(),
        );
        assert_eq!(dim.x(), 1);
        assert_eq!(dim.y(), 1);
        assert_eq!(dim.z(), 1);
    }

    #[test]
    fn nondivisible_small_extent() {
        let workgroup_dimensions = super::create_workgroup_dimensions();
        let dim = WorkgroupGridDimensions::from_extent_and_stride(
            &workgroup_dimensions,
            Extent3d {
                width: workgroup_dimensions.x() * 2 - 1,
                height: workgroup_dimensions.y() * 2 - 1,
                depth_or_array_layers: workgroup_dimensions.z(),
            },
            NonZeroU32::new(2).unwrap(),
            NonZeroU32::new(2).unwrap(),
        );
        assert_eq!(dim.x(), 1);
        assert_eq!(dim.y(), 1);
        assert_eq!(dim.z(), 1);
    }
}

mod from_extent {
    use super::super::WorkgroupGridDimensions;
    use wgpu::Extent3d;

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn zero_width() {
        let workgroup_dimensions = super::create_workgroup_dimensions();
        let _ = WorkgroupGridDimensions::from_extent(
            &workgroup_dimensions,
            Extent3d {
                width: 0,
                height: 1,
                depth_or_array_layers: 1,
            },
        );
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn zero_height() {
        let workgroup_dimensions = super::create_workgroup_dimensions();
        let _ = WorkgroupGridDimensions::from_extent(
            &workgroup_dimensions,
            Extent3d {
                width: 1,
                height: 0,
                depth_or_array_layers: 1,
            },
        );
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn zero_depth() {
        let workgroup_dimensions = super::create_workgroup_dimensions();
        let _ = WorkgroupGridDimensions::from_extent(
            &workgroup_dimensions,
            Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 0,
            },
        );
    }

    #[test]
    fn no_remainder() {
        let workgroup_dimensions = super::create_workgroup_dimensions();
        let dim = WorkgroupGridDimensions::from_extent(
            &workgroup_dimensions,
            Extent3d {
                width: workgroup_dimensions.x() * 2,
                height: workgroup_dimensions.y() * 2,
                depth_or_array_layers: workgroup_dimensions.z(),
            },
        );
        assert_eq!(dim.x(), 2);
        assert_eq!(dim.y(), 2);
        assert_eq!(dim.z(), 1);
    }

    #[test]
    fn remainder() {
        let workgroup_dimensions = super::create_workgroup_dimensions();
        let dim = WorkgroupGridDimensions::from_extent(
            &workgroup_dimensions,
            Extent3d {
                width: workgroup_dimensions.x() * 2 - 1,
                height: workgroup_dimensions.y() * 2 + 1,
                depth_or_array_layers: workgroup_dimensions.z(),
            },
        );
        assert_eq!(dim.x(), 2);
        assert_eq!(dim.y(), 3);
        assert_eq!(dim.z(), 1);
    }
}

mod count_swap {
    use super::super::WorkgroupGridDimensions;

    #[test]
    fn count_swap() {
        let dim = WorkgroupGridDimensions::count_swap();
        assert_eq!(dim.x(), 1);
        assert_eq!(dim.y(), 1);
        assert_eq!(dim.z(), 1);
    }
}

mod count {
    use super::super::WorkgroupGridDimensions;
    use std::error::Error;
    use wgpu::Extent3d;

    #[test]
    fn count() -> Result<(), Box<dyn Error>> {
        let workgroup_dimensions = super::create_workgroup_dimensions();
        let dim = WorkgroupGridDimensions::from_extent(
            &workgroup_dimensions,
            Extent3d {
                width: workgroup_dimensions.x() * 2 - 1,
                height: workgroup_dimensions.y() * 2 + 1,
                depth_or_array_layers: workgroup_dimensions.z(),
            },
        );
        assert_eq!(dim.x(), 2);
        assert_eq!(dim.y(), 3);
        assert_eq!(dim.z(), 1);
        assert_eq!(
            dim.count(),
            <usize as TryFrom<u32>>::try_from(dim.x() * dim.y() * dim.z())?
        );
        Ok(())
    }
}
