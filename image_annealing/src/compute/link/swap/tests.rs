mod swap_pass {
    use super::super::SwapPass;
    use crate::ImageDimensions;
    use image_annealing_shaders::constant;
    use std::error::Error;

    #[test]
    fn passes_array_contains_all_passes_once() {
        let mut counts = [1; constant::count_swap::N_CHANNEL];
        SwapPass::PASSES.iter().for_each(|pass| match pass {
            SwapPass::Horizontal => counts[0] -= 1,
            SwapPass::Vertical => counts[1] -= 1,
            SwapPass::OffsetHorizontal => counts[2] -= 1,
            SwapPass::OffsetVertical => counts[3] -= 1,
        });
        assert!(counts.iter().all(|&count| count == 0));
    }

    mod swap_workgroup_grid_dimensions {
        use super::super::super::super::super::operation::WorkgroupGridDimensions;
        use super::super::super::SwapPass;
        use crate::ImageDimensions;
        use image_annealing_shaders::WorkgroupDimensions;
        use std::error::Error;

        #[test]
        fn horizontal() -> Result<(), Box<dyn Error>> {
            assert_eq!(
                SwapPass::Horizontal.swap_workgroup_grid_dimensions(&ImageDimensions::new(33, 33)?),
                WorkgroupGridDimensions::from_extent(
                    &WorkgroupDimensions::swap(),
                    wgpu::Extent3d {
                        width: 17,
                        height: 33,
                        depth_or_array_layers: 1,
                    }
                )
            );
            Ok(())
        }

        #[test]
        fn vertical() -> Result<(), Box<dyn Error>> {
            assert_eq!(
                SwapPass::Vertical.swap_workgroup_grid_dimensions(&ImageDimensions::new(33, 33)?),
                WorkgroupGridDimensions::from_extent(
                    &WorkgroupDimensions::swap(),
                    wgpu::Extent3d {
                        width: 33,
                        height: 17,
                        depth_or_array_layers: 1,
                    }
                )
            );
            Ok(())
        }

        #[test]
        fn offset_horizontal() -> Result<(), Box<dyn Error>> {
            assert_eq!(
                SwapPass::OffsetHorizontal
                    .swap_workgroup_grid_dimensions(&ImageDimensions::new(33, 33)?),
                WorkgroupGridDimensions::from_extent(
                    &WorkgroupDimensions::swap(),
                    wgpu::Extent3d {
                        width: 17,
                        height: 33,
                        depth_or_array_layers: 1,
                    }
                )
            );
            Ok(())
        }

        #[test]
        fn offset_vertical() -> Result<(), Box<dyn Error>> {
            assert_eq!(
                SwapPass::OffsetVertical
                    .swap_workgroup_grid_dimensions(&ImageDimensions::new(33, 33)?),
                WorkgroupGridDimensions::from_extent(
                    &WorkgroupDimensions::swap(),
                    wgpu::Extent3d {
                        width: 33,
                        height: 17,
                        depth_or_array_layers: 1,
                    }
                )
            );
            Ok(())
        }
    }

    mod total_swaps {
        use super::super::super::SwapPass;
        use crate::ImageDimensions;
        use std::error::Error;

        #[test]
        fn horizontal() -> Result<(), Box<dyn Error>> {
            assert_eq!(
                SwapPass::Horizontal.total_swaps(&ImageDimensions::new(33, 16)?),
                256
            );
            Ok(())
        }

        #[test]
        fn vertical() -> Result<(), Box<dyn Error>> {
            assert_eq!(
                SwapPass::Vertical.total_swaps(&ImageDimensions::new(16, 33)?),
                256
            );
            Ok(())
        }

        #[test]
        fn offset_horizontal() -> Result<(), Box<dyn Error>> {
            assert_eq!(
                SwapPass::OffsetHorizontal.total_swaps(&ImageDimensions::new(33, 16)?),
                256
            );
            Ok(())
        }

        #[test]
        fn offset_vertical() -> Result<(), Box<dyn Error>> {
            assert_eq!(
                SwapPass::OffsetVertical.total_swaps(&ImageDimensions::new(16, 33)?),
                256
            );
            Ok(())
        }
    }

    #[test]
    fn total_workgroups() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            SwapPass::total_workgroups(&ImageDimensions::new(17, 33)?),
            14
        );
        Ok(())
    }

    mod display {
        use super::super::super::SwapPass;

        #[test]
        fn horizontal() {
            let str = SwapPass::Horizontal.to_string();
            assert!(str.contains("horizontal"));
            assert!(str.contains("no offset"));
        }

        #[test]
        fn vertical() {
            let str = SwapPass::Vertical.to_string();
            assert!(str.contains("vertical"));
            assert!(str.contains("no offset"));
        }

        #[test]
        fn offset_horizontal() {
            let str = SwapPass::OffsetHorizontal.to_string();
            assert!(str.contains("horizontal"));
            assert!(str.contains("with offset"));
        }

        #[test]
        fn offset_vertical() {
            let str = SwapPass::OffsetVertical.to_string();
            assert!(str.contains("vertical"));
            assert!(str.contains("with offset"));
        }
    }
}
