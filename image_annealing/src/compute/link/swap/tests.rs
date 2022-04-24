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

mod swap_pass_selection {
    mod includes_pass {
        use super::super::super::{SwapPass, SwapPassSelection};

        #[test]
        fn does_not_include_pass() {
            assert!(!SwapPassSelection::empty().includes_pass(SwapPass::Horizontal));
        }

        #[test]
        fn includes_pass() {
            assert!(SwapPassSelection::all().includes_pass(SwapPass::Horizontal));
        }
    }

    mod add_pass {
        use super::super::super::{SwapPass, SwapPassSelection};

        #[test]
        fn new_pass() {
            let selection = SwapPassSelection::empty().add_pass(SwapPass::Horizontal);
            assert!(selection.includes_pass(SwapPass::Horizontal));
        }

        #[test]
        fn existing_pass() {
            let mut selection = SwapPassSelection::HORIZONTAL;
            assert!(selection.includes_pass(SwapPass::Horizontal));
            selection = selection.add_pass(SwapPass::Horizontal);
            assert!(selection.includes_pass(SwapPass::Horizontal));
        }
    }

    mod iter {
        use super::super::super::{SwapPass, SwapPassSelection};
        use image_annealing_shaders::constant;

        #[test]
        fn all_passes() {
            let mut counts = [1; constant::count_swap::N_CHANNEL];
            SwapPassSelection::all().iter().for_each(|pass| match pass {
                SwapPass::Horizontal => counts[0] -= 1,
                SwapPass::Vertical => counts[1] -= 1,
                SwapPass::OffsetHorizontal => counts[2] -= 1,
                SwapPass::OffsetVertical => counts[3] -= 1,
            });
            assert!(counts.iter().all(|&count| count == 0));
        }

        #[test]
        fn three_passes() {
            let mut counts = [1, 1, 0, 1];
            (SwapPassSelection::HORIZONTAL
                | SwapPassSelection::VERTICAL
                | SwapPassSelection::OFFSET_VERTICAL)
                .iter()
                .for_each(|pass| match pass {
                    SwapPass::Horizontal => counts[0] -= 1,
                    SwapPass::Vertical => counts[1] -= 1,
                    SwapPass::OffsetHorizontal => counts[2] -= 1,
                    SwapPass::OffsetVertical => counts[3] -= 1,
                });
            assert!(counts.iter().all(|&count| count == 0));
        }

        #[test]
        fn no_passes() {
            let mut counts = [0; constant::count_swap::N_CHANNEL];
            SwapPassSelection::empty()
                .iter()
                .for_each(|pass| match pass {
                    SwapPass::Horizontal => counts[0] -= 1,
                    SwapPass::Vertical => counts[1] -= 1,
                    SwapPass::OffsetHorizontal => counts[2] -= 1,
                    SwapPass::OffsetVertical => counts[3] -= 1,
                });
            assert!(counts.iter().all(|&count| count == 0));
        }
    }

    mod from_swap_pass {
        use super::super::super::{SwapPass, SwapPassSelection};

        #[test]
        fn horizontal() {
            let mut counts = [1, 0, 0, 0];
            SwapPassSelection::from(SwapPass::Horizontal)
                .iter()
                .for_each(|pass| match pass {
                    SwapPass::Horizontal => counts[0] -= 1,
                    SwapPass::Vertical => counts[1] -= 1,
                    SwapPass::OffsetHorizontal => counts[2] -= 1,
                    SwapPass::OffsetVertical => counts[3] -= 1,
                });
            assert!(counts.iter().all(|&count| count == 0));
        }

        #[test]
        fn vertical() {
            let mut counts = [0, 1, 0, 0];
            SwapPassSelection::from(SwapPass::Vertical)
                .iter()
                .for_each(|pass| match pass {
                    SwapPass::Horizontal => counts[0] -= 1,
                    SwapPass::Vertical => counts[1] -= 1,
                    SwapPass::OffsetHorizontal => counts[2] -= 1,
                    SwapPass::OffsetVertical => counts[3] -= 1,
                });
            assert!(counts.iter().all(|&count| count == 0));
        }

        #[test]
        fn offset_horizontal() {
            let mut counts = [0, 0, 1, 0];
            SwapPassSelection::from(SwapPass::OffsetHorizontal)
                .iter()
                .for_each(|pass| match pass {
                    SwapPass::Horizontal => counts[0] -= 1,
                    SwapPass::Vertical => counts[1] -= 1,
                    SwapPass::OffsetHorizontal => counts[2] -= 1,
                    SwapPass::OffsetVertical => counts[3] -= 1,
                });
            assert!(counts.iter().all(|&count| count == 0));
        }

        #[test]
        fn offset_vertical() {
            let mut counts = [0, 0, 0, 1];
            SwapPassSelection::from(SwapPass::OffsetVertical)
                .iter()
                .for_each(|pass| match pass {
                    SwapPass::Horizontal => counts[0] -= 1,
                    SwapPass::Vertical => counts[1] -= 1,
                    SwapPass::OffsetHorizontal => counts[2] -= 1,
                    SwapPass::OffsetVertical => counts[3] -= 1,
                });
            assert!(counts.iter().all(|&count| count == 0));
        }
    }
}

mod count_swap_input_layout {
    use super::super::{CountSwapInputLayout, SwapPass, SwapPassSelection};
    use crate::ImageDimensions;
    use std::convert::TryInto;
    use std::error::Error;

    #[test]
    fn new() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::new(17, 33)?;
        let layout = CountSwapInputLayout::new(&dimensions);
        assert!(layout.do_segment.iter().all(|&flag| flag == 0));
        let mut first = 0_u32;
        let mut second: u32 = first
            + <usize as TryInto<u32>>::try_into(
                SwapPass::Horizontal
                    .swap_workgroup_grid_dimensions(&dimensions)
                    .count(),
            )?;
        let mut third: u32 = second
            + <usize as TryInto<u32>>::try_into(
                SwapPass::Vertical
                    .swap_workgroup_grid_dimensions(&dimensions)
                    .count(),
            )?;
        let mut fourth: u32 = third
            + <usize as TryInto<u32>>::try_into(
                SwapPass::OffsetHorizontal
                    .swap_workgroup_grid_dimensions(&dimensions)
                    .count(),
            )?;
        assert!([first, second, third, fourth]
            .iter()
            .eq(layout.segment_start.iter()));
        first = second;
        second = third;
        third = fourth;
        fourth = third
            + <usize as TryInto<u32>>::try_into(
                SwapPass::OffsetVertical
                    .swap_workgroup_grid_dimensions(&dimensions)
                    .count(),
            )?;
        assert!([first, second, third, fourth]
            .iter()
            .eq(layout.segment_end.iter()));
        Ok(())
    }

    #[test]
    fn get_selection() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::new(17, 33)?;
        let layout = CountSwapInputLayout::new(&dimensions);
        assert_eq!(layout.get_selection(), SwapPassSelection::empty());
        Ok(())
    }

    #[test]
    fn update_selection() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::new(17, 33)?;
        let mut layout = CountSwapInputLayout::new(&dimensions);
        let selection = SwapPassSelection::HORIZONTAL
            | SwapPassSelection::VERTICAL
            | SwapPassSelection::OFFSET_VERTICAL;
        layout.update_selection(selection);
        assert_eq!(layout.get_selection(), selection);
        Ok(())
    }
}

mod count_swap_output {
    use super::super::{CountSwapOutput, SwapPass};
    use std::convert::TryInto;
    use std::error::Error;

    #[test]
    fn from_ne_bytes() -> Result<(), Box<dyn Error>> {
        let counts = [1.0_f32, 2.0_f32, 3.0_f32, 4.0_f32];
        let bytes: Vec<u8> = counts
            .iter()
            .flat_map(|count| count.to_ne_bytes())
            .collect();
        let output = CountSwapOutput::from_ne_bytes(bytes.as_slice().try_into()?);
        assert_eq!(output.0, counts);
        Ok(())
    }

    #[test]
    fn at_pass() -> Result<(), Box<dyn Error>> {
        let counts = [1.0_f32, 2.0_f32, 3.0_f32, 4.0_f32];
        let bytes: Vec<u8> = counts
            .iter()
            .flat_map(|count| count.to_ne_bytes())
            .collect();
        let output = CountSwapOutput::from_ne_bytes(bytes.as_slice().try_into()?);

        SwapPass::PASSES.iter().for_each(|pass| match pass {
            SwapPass::Horizontal => assert_eq!(output.at_pass(*pass), counts[0]),
            SwapPass::Vertical => assert_eq!(output.at_pass(*pass), counts[1]),
            SwapPass::OffsetHorizontal => assert_eq!(output.at_pass(*pass), counts[2]),
            SwapPass::OffsetVertical => assert_eq!(output.at_pass(*pass), counts[3]),
        });
        Ok(())
    }
}
