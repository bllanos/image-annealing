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

mod swap_pass_set {
    mod includes_pass {
        use super::super::super::{SwapPass, SwapPassSet};

        #[test]
        fn does_not_include_pass() {
            assert!(!SwapPassSet::empty().includes_pass(SwapPass::Horizontal));
        }

        #[test]
        fn includes_pass() {
            assert!(SwapPassSet::all().includes_pass(SwapPass::Horizontal));
        }
    }

    mod add_pass {
        use super::super::super::{SwapPass, SwapPassSet};

        #[test]
        fn new_pass() {
            let mut set = SwapPassSet::empty();
            assert!(!set.includes_pass(SwapPass::Horizontal));
            set = set.add_pass(SwapPass::Horizontal);
            assert!(set.includes_pass(SwapPass::Horizontal));
        }

        #[test]
        fn existing_pass() {
            let mut set = SwapPassSet::HORIZONTAL;
            assert!(set.includes_pass(SwapPass::Horizontal));
            set = set.add_pass(SwapPass::Horizontal);
            assert!(set.includes_pass(SwapPass::Horizontal));
        }
    }

    mod iter {
        use super::super::super::{SwapPass, SwapPassSet};
        use image_annealing_shaders::constant;

        #[test]
        fn all_passes() {
            let mut counts = [1; constant::count_swap::N_CHANNEL];
            SwapPassSet::all().iter().for_each(|pass| match pass {
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
            (SwapPassSet::HORIZONTAL | SwapPassSet::VERTICAL | SwapPassSet::OFFSET_VERTICAL)
                .iter()
                .for_each(|pass| match pass {
                    SwapPass::Horizontal => counts[0] -= 1,
                    SwapPass::Vertical => counts[1] -= 1,
                    SwapPass::OffsetHorizontal => unreachable!(),
                    SwapPass::OffsetVertical => counts[3] -= 1,
                });
            assert!(counts.iter().all(|&count| count == 0));
        }

        #[test]
        fn no_passes() {
            assert!(SwapPassSet::empty().iter().next().is_none());
        }
    }

    mod from_swap_pass {
        use super::super::super::{SwapPass, SwapPassSet};

        #[test]
        fn horizontal() {
            let mut count = 1;
            SwapPassSet::from(SwapPass::Horizontal)
                .iter()
                .for_each(|pass| match pass {
                    SwapPass::Horizontal => count -= 1,
                    SwapPass::Vertical => unreachable!(),
                    SwapPass::OffsetHorizontal => unreachable!(),
                    SwapPass::OffsetVertical => unreachable!(),
                });
            assert!(count == 0);
        }

        #[test]
        fn vertical() {
            let mut count = 1;
            SwapPassSet::from(SwapPass::Vertical)
                .iter()
                .for_each(|pass| match pass {
                    SwapPass::Horizontal => unreachable!(),
                    SwapPass::Vertical => count -= 1,
                    SwapPass::OffsetHorizontal => unreachable!(),
                    SwapPass::OffsetVertical => unreachable!(),
                });
            assert!(count == 0);
        }

        #[test]
        fn offset_horizontal() {
            let mut count = 1;
            SwapPassSet::from(SwapPass::OffsetHorizontal)
                .iter()
                .for_each(|pass| match pass {
                    SwapPass::Horizontal => unreachable!(),
                    SwapPass::Vertical => unreachable!(),
                    SwapPass::OffsetHorizontal => count -= 1,
                    SwapPass::OffsetVertical => unreachable!(),
                });
            assert!(count == 0);
        }

        #[test]
        fn offset_vertical() {
            let mut count = 1;
            SwapPassSet::from(SwapPass::OffsetVertical)
                .iter()
                .for_each(|pass| match pass {
                    SwapPass::Horizontal => unreachable!(),
                    SwapPass::Vertical => unreachable!(),
                    SwapPass::OffsetHorizontal => unreachable!(),
                    SwapPass::OffsetVertical => count -= 1,
                });
            assert!(count == 0);
        }
    }
}

mod count_swap_input_layout {
    use super::super::{CountSwapInputLayout, SwapPass, SwapPassSet};
    use crate::ImageDimensions;
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
    fn get_set() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::new(17, 33)?;
        let layout = CountSwapInputLayout::new(&dimensions);
        assert_eq!(layout.get_set(), SwapPassSet::empty());
        Ok(())
    }

    #[test]
    fn update_set() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::new(17, 33)?;
        let mut layout = CountSwapInputLayout::new(&dimensions);
        let set = SwapPassSet::HORIZONTAL | SwapPassSet::VERTICAL | SwapPassSet::OFFSET_VERTICAL;
        layout.update_set(set);
        assert_eq!(layout.get_set(), set);
        Ok(())
    }
}

mod count_swap_output {
    use super::super::{CountSwapOutput, SwapPass};
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

        SwapPass::PASSES
            .iter()
            .zip(counts.iter())
            .for_each(|(pass, count)| assert_eq!(output.at_pass(*pass), *count));
        Ok(())
    }
}

mod swap_shader_parameters {
    use super::super::{CountSwapInputLayout, SwapPass, SwapShaderParameters};
    use crate::ImageDimensions;
    use std::error::Error;

    #[test]
    fn new() -> Result<(), Box<dyn Error>> {
        let parameters = SwapShaderParameters::new();
        let dimensions = ImageDimensions::new(17, 33)?;
        let layout = CountSwapInputLayout::new(&dimensions);
        assert_eq!(parameters.displacement, [0, 0]);
        assert_eq!(parameters.offset, [0, 0]);
        assert_eq!(parameters.count_output_offset, layout.segment_start[0]);
        assert_eq!(parameters.acceptance_threshold, Default::default());
        Ok(())
    }

    #[test]
    fn set_acceptance_threshold() {
        let mut parameters = SwapShaderParameters::new();
        assert_eq!(parameters.acceptance_threshold, 0.0);
        let new_threshold = 1.0;
        parameters.set_acceptance_threshold(new_threshold);
        assert_eq!(parameters.acceptance_threshold, new_threshold);
    }

    #[test]
    fn set_pass() -> Result<(), Box<dyn Error>> {
        let mut parameters = SwapShaderParameters::new();
        let dimensions = ImageDimensions::new(17, 33)?;
        let layout = CountSwapInputLayout::new(&dimensions);

        SwapPass::PASSES
            .iter()
            .zip(layout.segment_start.iter())
            .for_each(|(pass, offset)| {
                parameters.set_pass(*pass, &layout);
                assert_eq!(parameters.count_output_offset, *offset);
                assert_eq!(parameters.displacement, pass.displacement_vector());
                assert_eq!(parameters.offset, pass.offset_vector());
            });
        Ok(())
    }
}
