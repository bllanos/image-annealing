mod swap_pass {
    use super::super::SwapPass;
    use crate::ImageDimensions;
    use image_annealing_shader::constant;
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

    mod snake_case_name {
        use super::super::super::SwapPass;

        #[test]
        fn horizontal() {
            let str = SwapPass::Horizontal.snake_case_name();
            assert_eq!(str, "horizontal");
        }

        #[test]
        fn vertical() {
            let str = SwapPass::Vertical.snake_case_name();
            assert_eq!(str, "vertical");
        }

        #[test]
        fn offset_horizontal() {
            let str = SwapPass::OffsetHorizontal.snake_case_name();
            assert_eq!(str, "offset_horizontal");
        }

        #[test]
        fn offset_vertical() {
            let str = SwapPass::OffsetVertical.snake_case_name();
            assert_eq!(str, "offset_vertical");
        }
    }

    mod swap_workgroup_grid_dimensions {
        use super::super::super::super::super::operation::WorkgroupGridDimensions;
        use super::super::super::SwapPass;
        use crate::ImageDimensions;
        use image_annealing_shader::WorkgroupDimensions;
        use std::error::Error;

        #[test]
        fn horizontal() -> Result<(), Box<dyn Error>> {
            assert_eq!(
                SwapPass::Horizontal
                    .swap_workgroup_grid_dimensions(&ImageDimensions::try_new(33, 33)?),
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
                SwapPass::Vertical
                    .swap_workgroup_grid_dimensions(&ImageDimensions::try_new(33, 33)?),
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
                    .swap_workgroup_grid_dimensions(&ImageDimensions::try_new(33, 33)?),
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
                    .swap_workgroup_grid_dimensions(&ImageDimensions::try_new(33, 33)?),
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
                SwapPass::Horizontal.total_swaps(&ImageDimensions::try_new(33, 16)?),
                256
            );
            Ok(())
        }

        #[test]
        fn vertical() -> Result<(), Box<dyn Error>> {
            assert_eq!(
                SwapPass::Vertical.total_swaps(&ImageDimensions::try_new(16, 33)?),
                256
            );
            Ok(())
        }

        #[test]
        fn offset_horizontal() -> Result<(), Box<dyn Error>> {
            assert_eq!(
                SwapPass::OffsetHorizontal.total_swaps(&ImageDimensions::try_new(33, 16)?),
                256
            );
            Ok(())
        }

        #[test]
        fn offset_vertical() -> Result<(), Box<dyn Error>> {
            assert_eq!(
                SwapPass::OffsetVertical.total_swaps(&ImageDimensions::try_new(16, 33)?),
                256
            );
            Ok(())
        }
    }

    #[test]
    fn total_workgroups() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            SwapPass::total_workgroups(&ImageDimensions::try_new(17, 33)?),
            14
        );
        Ok(())
    }

    mod displacement_vector {
        use super::super::super::SwapPass;

        #[test]
        fn horizontal() {
            assert_eq!(SwapPass::Horizontal.displacement_vector(), [1, 0]);
        }

        #[test]
        fn vertical() {
            assert_eq!(SwapPass::Vertical.displacement_vector(), [0, 1]);
        }

        #[test]
        fn offset_horizontal() {
            assert_eq!(SwapPass::OffsetHorizontal.displacement_vector(), [1, 0]);
        }

        #[test]
        fn offset_vertical() {
            assert_eq!(SwapPass::OffsetVertical.displacement_vector(), [0, 1]);
        }
    }

    mod offset_vector {
        use super::super::super::SwapPass;

        #[test]
        fn horizontal() {
            assert_eq!(SwapPass::Horizontal.offset_vector(), [0, 0]);
        }

        #[test]
        fn vertical() {
            assert_eq!(SwapPass::Vertical.offset_vector(), [0, 0]);
        }

        #[test]
        fn offset_horizontal() {
            assert_eq!(SwapPass::OffsetHorizontal.offset_vector(), [-1, 0]);
        }

        #[test]
        fn offset_vertical() {
            assert_eq!(SwapPass::OffsetVertical.offset_vector(), [0, -1]);
        }
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
    mod from_passes {
        use super::super::super::{SwapPass, SwapPassSet};

        #[test]
        fn empty() {
            assert_eq!(SwapPassSet::from_passes([]), SwapPassSet::empty());
        }

        #[test]
        fn single_pass() {
            assert_eq!(
                SwapPassSet::from_passes([SwapPass::Vertical]),
                SwapPassSet::VERTICAL
            );
        }

        #[test]
        fn three_passes_in_order() {
            assert_eq!(
                SwapPassSet::from_passes([
                    SwapPass::Vertical,
                    SwapPass::OffsetHorizontal,
                    SwapPass::OffsetVertical
                ]),
                SwapPassSet::VERTICAL
                    | SwapPassSet::OFFSET_HORIZONTAL
                    | SwapPassSet::OFFSET_VERTICAL
            );
        }

        #[test]
        fn three_passes_reverse_order() {
            assert_eq!(
                SwapPassSet::from_passes([
                    SwapPass::OffsetVertical,
                    SwapPass::OffsetHorizontal,
                    SwapPass::Vertical,
                ]),
                SwapPassSet::VERTICAL
                    | SwapPassSet::OFFSET_HORIZONTAL
                    | SwapPassSet::OFFSET_VERTICAL
            );
        }

        #[test]
        fn all() {
            assert_eq!(
                SwapPassSet::from_passes(SwapPass::PASSES),
                SwapPassSet::all()
            );
        }
    }

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

    mod equal_set {
        use super::super::super::{SwapPass, SwapPassSequence, SwapPassSet};
        use std::error::Error;

        #[test]
        fn subset() -> Result<(), Box<dyn Error>> {
            assert!(!SwapPassSet::from(SwapPass::Horizontal).equal_set(
                &SwapPassSequence::from_passes([SwapPass::Horizontal, SwapPass::OffsetHorizontal])?
            ));
            Ok(())
        }

        #[test]
        fn superset() -> Result<(), Box<dyn Error>> {
            assert!(!(SwapPassSet::HORIZONTAL
                | SwapPassSet::VERTICAL
                | SwapPassSet::OFFSET_HORIZONTAL)
                .equal_set(&SwapPassSequence::from_passes([
                    SwapPass::Horizontal,
                    SwapPass::OffsetHorizontal
                ])?));
            Ok(())
        }

        #[test]
        fn disjoint() {
            assert!(!SwapPassSet::from(SwapPass::Horizontal)
                .equal_set(&SwapPassSequence::from(SwapPass::OffsetHorizontal)));
        }

        #[test]
        fn equal() -> Result<(), Box<dyn Error>> {
            assert!(
                (SwapPassSet::HORIZONTAL | SwapPassSet::OFFSET_HORIZONTAL).equal_set(
                    &SwapPassSequence::from_passes([
                        SwapPass::Horizontal,
                        SwapPass::OffsetHorizontal
                    ])?
                )
            );
            Ok(())
        }
    }

    mod contains_set {
        use super::super::super::{SwapPass, SwapPassSequence, SwapPassSet};
        use std::error::Error;

        #[test]
        fn subset() -> Result<(), Box<dyn Error>> {
            assert!(!SwapPassSet::from(SwapPass::Horizontal).contains_set(
                &SwapPassSequence::from_passes([SwapPass::Horizontal, SwapPass::OffsetHorizontal])?
            ));
            Ok(())
        }

        #[test]
        fn superset() -> Result<(), Box<dyn Error>> {
            assert!((SwapPassSet::HORIZONTAL
                | SwapPassSet::VERTICAL
                | SwapPassSet::OFFSET_HORIZONTAL)
                .contains_set(&SwapPassSequence::from_passes([
                    SwapPass::Horizontal,
                    SwapPass::OffsetHorizontal
                ])?));
            Ok(())
        }

        #[test]
        fn disjoint() {
            assert!(!SwapPassSet::from(SwapPass::Horizontal)
                .contains_set(&SwapPassSequence::from(SwapPass::OffsetHorizontal)));
        }

        #[test]
        fn equal() -> Result<(), Box<dyn Error>> {
            assert!(
                (SwapPassSet::HORIZONTAL | SwapPassSet::OFFSET_HORIZONTAL).contains_set(
                    &SwapPassSequence::from_passes([
                        SwapPass::Horizontal,
                        SwapPass::OffsetHorizontal
                    ])?
                )
            );
            Ok(())
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
        use image_annealing_shader::constant;

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

    mod from_sequence {
        use super::super::super::{SwapPass, SwapPassSequence, SwapPassSet};
        use std::error::Error;

        #[test]
        fn single_pass() {
            assert_eq!(
                SwapPassSet::from(SwapPassSequence::from(SwapPass::Vertical)),
                SwapPassSet::VERTICAL
            );
        }

        #[test]
        fn three_passes_in_order() -> Result<(), Box<dyn Error>> {
            assert_eq!(
                SwapPassSet::from(SwapPassSequence::from_passes([
                    SwapPass::Vertical,
                    SwapPass::OffsetHorizontal,
                    SwapPass::OffsetVertical
                ])?),
                SwapPassSet::VERTICAL
                    | SwapPassSet::OFFSET_HORIZONTAL
                    | SwapPassSet::OFFSET_VERTICAL
            );
            Ok(())
        }

        #[test]
        fn three_passes_reverse_order() -> Result<(), Box<dyn Error>> {
            assert_eq!(
                SwapPassSet::from(SwapPassSequence::from_passes([
                    SwapPass::OffsetVertical,
                    SwapPass::OffsetHorizontal,
                    SwapPass::Vertical,
                ])?),
                SwapPassSet::VERTICAL
                    | SwapPassSet::OFFSET_HORIZONTAL
                    | SwapPassSet::OFFSET_VERTICAL
            );
            Ok(())
        }

        #[test]
        fn all() {
            assert_eq!(
                SwapPassSet::from(SwapPassSequence::all()),
                SwapPassSet::all()
            );
        }
    }
}

mod swap_pass_sequence {
    use super::super::{SwapPass, SwapPassSequence};

    mod from_passes {
        use super::super::super::{SwapPass, SwapPassSequence};
        use std::error::Error;

        #[test]
        fn empty() {
            test_util::assert_error_contains(
                SwapPassSequence::from_passes([]),
                "selection of swap passes is empty",
            );
        }

        #[test]
        fn single_pass() -> Result<(), Box<dyn Error>> {
            let passes = [SwapPass::Vertical];
            assert!(SwapPassSequence::from_passes(passes)?
                .into_iter()
                .eq(passes));
            Ok(())
        }

        #[test]
        fn three_passes_in_order() -> Result<(), Box<dyn Error>> {
            let passes = [
                SwapPass::Vertical,
                SwapPass::OffsetHorizontal,
                SwapPass::OffsetVertical,
            ];
            assert!(SwapPassSequence::from_passes(passes)?
                .into_iter()
                .eq(passes));
            Ok(())
        }

        #[test]
        fn three_passes_reverse_order() -> Result<(), Box<dyn Error>> {
            let passes = [
                SwapPass::OffsetVertical,
                SwapPass::OffsetHorizontal,
                SwapPass::Vertical,
            ];
            assert!(SwapPassSequence::from_passes(passes)?
                .into_iter()
                .eq(passes));
            Ok(())
        }

        #[test]
        fn duplicate() {
            let passes = [
                SwapPass::OffsetHorizontal,
                SwapPass::OffsetVertical,
                SwapPass::OffsetHorizontal,
                SwapPass::Vertical,
            ];
            test_util::assert_error_contains(
                SwapPassSequence::from_passes(passes),
                "attempt to select horizontal swaps, with offset pass multiple times",
            );
        }

        #[test]
        fn all() -> Result<(), Box<dyn Error>> {
            assert_eq!(
                SwapPassSequence::from_passes(SwapPass::PASSES)?,
                SwapPassSequence::all()
            );
            Ok(())
        }
    }

    #[test]
    fn all() {
        assert!(SwapPassSequence::all().into_iter().eq(SwapPass::PASSES));
    }

    mod includes_pass {
        use super::super::super::{SwapPass, SwapPassSequence};

        #[test]
        fn does_not_include_pass() {
            assert!(!SwapPassSequence::from(SwapPass::Horizontal).includes_pass(SwapPass::Vertical));
        }

        #[test]
        fn includes_pass() {
            assert!(SwapPassSequence::all().includes_pass(SwapPass::Vertical));
        }
    }

    mod equal_set {
        use super::super::super::{SwapPass, SwapPassSequence, SwapPassSet};
        use std::error::Error;

        #[test]
        fn empty() {
            assert!(!SwapPassSequence::from(SwapPass::Horizontal).equal_set(&SwapPassSet::empty()));
        }

        #[test]
        fn subset() {
            assert!(!SwapPassSequence::from(SwapPass::Horizontal)
                .equal_set(&(SwapPassSet::HORIZONTAL | SwapPassSet::OFFSET_HORIZONTAL)));
        }

        #[test]
        fn superset() -> Result<(), Box<dyn Error>> {
            assert!(!SwapPassSequence::from_passes([
                SwapPass::Horizontal,
                SwapPass::Vertical,
                SwapPass::OffsetHorizontal
            ])?
            .equal_set(&(SwapPassSet::HORIZONTAL | SwapPassSet::OFFSET_HORIZONTAL)));
            Ok(())
        }

        #[test]
        fn disjoint() {
            assert!(!SwapPassSequence::from(SwapPass::Horizontal)
                .equal_set(&SwapPassSet::OFFSET_HORIZONTAL));
        }

        #[test]
        fn equal() -> Result<(), Box<dyn Error>> {
            assert!(SwapPassSequence::from_passes([
                SwapPass::Horizontal,
                SwapPass::OffsetHorizontal
            ])?
            .equal_set(&(SwapPassSet::HORIZONTAL | SwapPassSet::OFFSET_HORIZONTAL)));
            Ok(())
        }
    }

    mod contains_set {
        use super::super::super::{SwapPass, SwapPassSequence, SwapPassSet};
        use std::error::Error;

        #[test]
        fn empty() {
            assert!(
                SwapPassSequence::from(SwapPass::Horizontal).contains_set(&SwapPassSet::empty())
            );
        }

        #[test]
        fn subset() {
            assert!(!SwapPassSequence::from(SwapPass::Horizontal)
                .contains_set(&(SwapPassSet::HORIZONTAL | SwapPassSet::OFFSET_HORIZONTAL)));
        }

        #[test]
        fn superset() -> Result<(), Box<dyn Error>> {
            assert!(SwapPassSequence::from_passes([
                SwapPass::Horizontal,
                SwapPass::Vertical,
                SwapPass::OffsetHorizontal
            ])?
            .contains_set(&(SwapPassSet::HORIZONTAL | SwapPassSet::OFFSET_HORIZONTAL)));
            Ok(())
        }

        #[test]
        fn disjoint() {
            assert!(!SwapPassSequence::from(SwapPass::Horizontal)
                .contains_set(&SwapPassSet::OFFSET_HORIZONTAL));
        }

        #[test]
        fn equal() -> Result<(), Box<dyn Error>> {
            assert!(SwapPassSequence::from_passes([
                SwapPass::Horizontal,
                SwapPass::OffsetHorizontal
            ])?
            .contains_set(&(SwapPassSet::HORIZONTAL | SwapPassSet::OFFSET_HORIZONTAL)));
            Ok(())
        }
    }

    mod add_pass {
        use super::super::super::{SwapPass, SwapPassSequence};
        use std::error::Error;

        #[test]
        fn new_pass() -> Result<(), Box<dyn Error>> {
            let mut sequence = SwapPassSequence::from(SwapPass::Vertical);
            assert!(!sequence.includes_pass(SwapPass::Horizontal));
            sequence = sequence.add_pass(SwapPass::Horizontal)?;
            assert!(sequence.includes_pass(SwapPass::Horizontal));
            Ok(())
        }

        #[test]
        fn existing_pass() {
            let sequence = SwapPassSequence::from(SwapPass::Vertical);
            assert!(sequence.includes_pass(SwapPass::Vertical));
            test_util::assert_error_contains(
                sequence.add_pass(SwapPass::Vertical),
                "attempt to select vertical swaps, no offset pass multiple times",
            );
            assert!(sequence.includes_pass(SwapPass::Vertical));
        }
    }

    mod iter {
        use super::super::super::{SwapPass, SwapPassSequence};
        use image_annealing_shader::constant;
        use std::error::Error;

        #[test]
        fn all_passes() {
            let mut counts = [1; constant::count_swap::N_CHANNEL];
            SwapPassSequence::all().iter().for_each(|pass| match pass {
                SwapPass::Horizontal => counts[0] -= 1,
                SwapPass::Vertical => counts[1] -= 1,
                SwapPass::OffsetHorizontal => counts[2] -= 1,
                SwapPass::OffsetVertical => counts[3] -= 1,
            });
            assert!(counts.iter().all(|&count| count == 0));
        }

        #[test]
        fn three_passes() -> Result<(), Box<dyn Error>> {
            let mut counts = [1, 1, 0, 1];
            SwapPassSequence::from_passes([
                SwapPass::Horizontal,
                SwapPass::Vertical,
                SwapPass::OffsetVertical,
            ])?
            .iter()
            .for_each(|pass| match pass {
                SwapPass::Horizontal => counts[0] -= 1,
                SwapPass::Vertical => counts[1] -= 1,
                SwapPass::OffsetHorizontal => unreachable!(),
                SwapPass::OffsetVertical => counts[3] -= 1,
            });
            assert!(counts.iter().all(|&count| count == 0));
            Ok(())
        }
    }

    mod from_swap_pass {
        use super::super::super::{SwapPass, SwapPassSequence};

        #[test]
        fn horizontal() {
            let mut count = 1;
            SwapPassSequence::from(SwapPass::Horizontal)
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
            SwapPassSequence::from(SwapPass::Vertical)
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
            SwapPassSequence::from(SwapPass::OffsetHorizontal)
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
            SwapPassSequence::from(SwapPass::OffsetVertical)
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

    mod try_from_set {
        use super::super::super::{SwapPass, SwapPassSequence, SwapPassSet};
        use std::error::Error;

        #[test]
        fn empty() {
            test_util::assert_error_contains(
                SwapPassSequence::try_from(SwapPassSet::empty()),
                "selection of swap passes is empty",
            );
        }

        #[test]
        fn single_pass() -> Result<(), Box<dyn Error>> {
            assert!(SwapPassSequence::try_from(SwapPassSet::VERTICAL)?
                .into_iter()
                .eq([SwapPass::Vertical]));
            Ok(())
        }

        #[test]
        fn three_passes() -> Result<(), Box<dyn Error>> {
            assert!(SwapPassSequence::try_from(
                SwapPassSet::VERTICAL
                    | SwapPassSet::OFFSET_HORIZONTAL
                    | SwapPassSet::OFFSET_VERTICAL
            )?
            .into_iter()
            .eq([
                SwapPass::Vertical,
                SwapPass::OffsetHorizontal,
                SwapPass::OffsetVertical
            ]));
            Ok(())
        }

        #[test]
        fn all() -> Result<(), Box<dyn Error>> {
            assert_eq!(
                SwapPassSequence::try_from(SwapPassSet::all())?,
                SwapPassSequence::all()
            );
            Ok(())
        }
    }
}

mod count_swap_input_layout {
    use super::super::{CountSwapInputLayout, SwapPass, SwapPassSet};
    use crate::ImageDimensions;
    use std::error::Error;

    #[test]
    fn new() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::try_new(17, 33)?;
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
        let dimensions = ImageDimensions::try_new(17, 33)?;
        let layout = CountSwapInputLayout::new(&dimensions);
        assert_eq!(layout.get_set(), SwapPassSet::empty());
        Ok(())
    }

    #[test]
    fn update_set() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::try_new(17, 33)?;
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
        let dimensions = ImageDimensions::try_new(17, 33)?;
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
        let dimensions = ImageDimensions::try_new(17, 33)?;
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
