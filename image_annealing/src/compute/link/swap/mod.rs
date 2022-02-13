use super::super::operation::WorkgroupGridDimensions;
use crate::ImageDimensions;
use bytemuck::{Pod, Zeroable};
use image_annealing_shaders::constant;
use image_annealing_shaders::WorkgroupDimensions;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::num::NonZeroU32;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SwapPass {
    Horizontal,
    Vertical,
    OffsetHorizontal,
    OffsetVertical,
}

impl SwapPass {
    pub fn swap_workgroup_grid_dimensions(
        &self,
        image_dimensions: &ImageDimensions,
    ) -> WorkgroupGridDimensions {
        let workgroup_dimensions = WorkgroupDimensions::swap();
        let swap_stride = NonZeroU32::new(constant::swap::STRIDE).unwrap();
        let unit_stride = NonZeroU32::new(1).unwrap();
        match self {
            SwapPass::Horizontal | SwapPass::OffsetHorizontal => {
                WorkgroupGridDimensions::from_image_dimensions_and_stride(
                    &workgroup_dimensions,
                    image_dimensions,
                    swap_stride,
                    unit_stride,
                )
            }
            SwapPass::Vertical | SwapPass::OffsetVertical => {
                WorkgroupGridDimensions::from_image_dimensions_and_stride(
                    &workgroup_dimensions,
                    image_dimensions,
                    unit_stride,
                    swap_stride,
                )
            }
        }
    }

    pub fn total_workgroups(image_dimensions: &ImageDimensions) -> usize {
        Self::PASSES
            .iter()
            .map(|swap_pass| {
                swap_pass
                    .swap_workgroup_grid_dimensions(image_dimensions)
                    .count()
            })
            .sum()
    }

    const PASSES: [SwapPass; constant::count_swap::N_CHANNEL] = [
        SwapPass::Horizontal,
        SwapPass::Vertical,
        SwapPass::OffsetHorizontal,
        SwapPass::OffsetVertical,
    ];
}

bitflags::bitflags! {
    pub struct SwapPassSelection: u32 {
        const HORIZONTAL = 1 << SwapPass::Horizontal as u32;
        const VERTICAL = 1 << SwapPass::Vertical as u32;
        const OFFSET_HORIZONTAL = 1 << SwapPass::OffsetHorizontal as u32;
        const OFFSET_VERTICAL = 1 << SwapPass::OffsetVertical as u32;
    }
}

impl From<SwapPass> for SwapPassSelection {
    fn from(pass: SwapPass) -> Self {
        match pass {
            SwapPass::Horizontal => SwapPassSelection::HORIZONTAL,
            SwapPass::Vertical => SwapPassSelection::VERTICAL,
            SwapPass::OffsetHorizontal => SwapPassSelection::OFFSET_HORIZONTAL,
            SwapPass::OffsetVertical => SwapPassSelection::OFFSET_VERTICAL,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
pub struct CountSwapInputLayout {
    do_segment: [u32; constant::count_swap::N_CHANNEL],
    segment_start: [u32; constant::count_swap::N_CHANNEL],
    segment_end: [u32; constant::count_swap::N_CHANNEL],
}

impl CountSwapInputLayout {
    pub fn new(image_dimensions: &ImageDimensions) -> Self {
        let counts = SwapPass::PASSES.map(|swap_pass| {
            <u32 as TryFrom<usize>>::try_from(
                swap_pass
                    .swap_workgroup_grid_dimensions(image_dimensions)
                    .count(),
            )
            .unwrap()
        });
        let segment_start = counts.iter().skip(1).enumerate().fold(
            [0u32; constant::count_swap::N_CHANNEL],
            |mut acc, (i, count)| {
                acc[i + 1] = acc[i].checked_add(*count).unwrap();
                acc
            },
        );
        let segment_end = counts.iter().enumerate().fold(
            [0u32; constant::count_swap::N_CHANNEL],
            |mut acc, (i, count)| {
                acc[i] = segment_start[i].checked_add(*count).unwrap();
                acc
            },
        );
        Self {
            do_segment: Default::default(),
            segment_start,
            segment_end,
        }
    }

    pub fn include_pass(&mut self, pass: SwapPass) {
        self.do_segment[pass as usize] = u32::from(true);
    }

    pub fn clear_passes(&mut self) {
        self.do_segment = Default::default();
    }
}

pub type CountSwapOutputDataElement = f32;
type CountSwapOutputData = [CountSwapOutputDataElement; constant::count_swap::N_CHANNEL];

#[repr(transparent)]
pub struct CountSwapOutput(CountSwapOutputData);

impl CountSwapOutput {
    pub const SIZE: usize = std::mem::size_of::<Self>();

    pub fn from_ne_bytes(bytes: [u8; Self::SIZE]) -> Self {
        Self(
            bytes
                .as_slice()
                .chunks_exact(std::mem::size_of::<CountSwapOutputDataElement>())
                .enumerate()
                .fold(
                    <CountSwapOutputData as Default>::default(),
                    |mut acc, (i, chunk)| {
                        acc[i] =
                            CountSwapOutputDataElement::from_ne_bytes(chunk.try_into().unwrap());
                        acc
                    },
                ),
        )
    }
}
