mod from_extent_and_stride {
    use super::super::WorkgroupGridDimensions;
    use std::num::NonZeroU32;
    use wgpu::Extent3d;

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn zero_depth() {
        let _ = WorkgroupGridDimensions::from_extent_and_stride(
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
        let extent = Extent3d {
            width: 64,
            height: 64,
            depth_or_array_layers: 1,
        };
        let expected = WorkgroupGridDimensions::from(extent);
        let actual = WorkgroupGridDimensions::from_extent_and_stride(
            extent,
            NonZeroU32::new(1).unwrap(),
            NonZeroU32::new(1).unwrap(),
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn stride_divisible_x() {
        let dim = WorkgroupGridDimensions::from_extent_and_stride(
            Extent3d {
                width: 128,
                height: 128,
                depth_or_array_layers: 1,
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
        let dim = WorkgroupGridDimensions::from_extent_and_stride(
            Extent3d {
                width: 128,
                height: 128,
                depth_or_array_layers: 1,
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
        let dim = WorkgroupGridDimensions::from_extent_and_stride(
            Extent3d {
                width: 65,
                height: 65,
                depth_or_array_layers: 1,
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
        let dim = WorkgroupGridDimensions::from_extent_and_stride(
            Extent3d {
                width: 64,
                height: 64,
                depth_or_array_layers: 1,
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
        let dim = WorkgroupGridDimensions::from_extent_and_stride(
            Extent3d {
                width: 63,
                height: 63,
                depth_or_array_layers: 1,
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
        let _ = WorkgroupGridDimensions::from(Extent3d {
            width: 0,
            height: 1,
            depth_or_array_layers: 1,
        });
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn zero_height() {
        let _ = WorkgroupGridDimensions::from(Extent3d {
            width: 1,
            height: 0,
            depth_or_array_layers: 1,
        });
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn zero_depth() {
        let _ = WorkgroupGridDimensions::from(Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 0,
        });
    }

    #[test]
    fn no_remainder() {
        let dim = WorkgroupGridDimensions::from(Extent3d {
            width: 64,
            height: 64,
            depth_or_array_layers: 1,
        });
        assert_eq!(dim.x(), 2);
        assert_eq!(dim.y(), 2);
        assert_eq!(dim.z(), 1);
    }

    #[test]
    fn remainder() {
        let dim = WorkgroupGridDimensions::from(Extent3d {
            width: 63,
            height: 65,
            depth_or_array_layers: 1,
        });
        assert_eq!(dim.x(), 2);
        assert_eq!(dim.y(), 3);
        assert_eq!(dim.z(), 1);
    }
}
