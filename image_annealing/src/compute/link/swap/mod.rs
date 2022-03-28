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
    pub(in super::super) fn swap_workgroup_grid_dimensions(
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

    pub fn total_swaps(&self, image_dimensions: &ImageDimensions) -> usize {
        let stride: usize = constant::swap::STRIDE.try_into().unwrap();
        match self {
            SwapPass::Horizontal => (image_dimensions.width() / stride)
                .checked_mul(image_dimensions.height())
                .unwrap(),
            SwapPass::Vertical => (image_dimensions.height() / stride)
                .checked_mul(image_dimensions.width())
                .unwrap(),
            SwapPass::OffsetHorizontal => ((image_dimensions.width() - constant::swap::OFFSET)
                / stride)
                .checked_mul(image_dimensions.height())
                .unwrap(),
            SwapPass::OffsetVertical => ((image_dimensions.height() - constant::swap::OFFSET)
                / stride)
                .checked_mul(image_dimensions.width())
                .unwrap(),
        }
    }

    pub(in super::super) fn total_workgroups(image_dimensions: &ImageDimensions) -> usize {
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
    #[derive(Default)]
    pub struct SwapPassSelection: u32 {
        const HORIZONTAL = 1 << SwapPass::Horizontal as u32;
        const VERTICAL = 1 << SwapPass::Vertical as u32;
        const OFFSET_HORIZONTAL = 1 << SwapPass::OffsetHorizontal as u32;
        const OFFSET_VERTICAL = 1 << SwapPass::OffsetVertical as u32;
    }
}

impl SwapPassSelection {
    pub fn includes_pass(&self, pass: SwapPass) -> bool {
        Self::from(pass).intersects(*self)
    }

    pub fn add_pass(&self, pass: SwapPass) -> Self {
        *self | Self::from(pass)
    }

    pub fn iter(&self) -> impl Iterator<Item = &SwapPass> {
        SwapPass::PASSES
            .iter()
            .filter(move |&&pass| self.includes_pass(pass))
    }
}

impl From<SwapPass> for SwapPassSelection {
    fn from(pass: SwapPass) -> Self {
        match pass {
            SwapPass::Horizontal => Self::HORIZONTAL,
            SwapPass::Vertical => Self::VERTICAL,
            SwapPass::OffsetHorizontal => Self::OFFSET_HORIZONTAL,
            SwapPass::OffsetVertical => Self::OFFSET_VERTICAL,
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

    pub fn get_selection(&self) -> SwapPassSelection {
        self.do_segment.iter().zip(SwapPass::PASSES.iter()).fold(
            SwapPassSelection::empty(),
            |acc, (&flag, &pass)| {
                if flag == 0 {
                    acc
                } else {
                    acc.add_pass(pass)
                }
            },
        )
    }

    pub fn update_selection(&mut self, selection: SwapPassSelection) -> bool {
        if self.get_selection() == selection {
            false
        } else {
            self.do_segment =
                SwapPass::PASSES.map(|swap_pass| u32::from(selection.includes_pass(swap_pass)));
            true
        }
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

    pub fn at_pass(&self, pass: SwapPass) -> CountSwapOutputDataElement {
        self.0[pass as usize]
    }
}

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
pub struct SwapShaderParameters {
    count_output_offset: u32,
    _padding: [u32; 3],
}

impl SwapShaderParameters {
    pub fn new() -> Self {
        Self {
            count_output_offset: 0,
            _padding: Default::default(),
        }
    }

    pub fn set_pass(&mut self, pass: SwapPass, layout: &CountSwapInputLayout) {
        self.count_output_offset = layout.segment_start[pass as usize];
    }
}
