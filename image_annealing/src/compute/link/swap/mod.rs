use super::super::operation::WorkgroupGridDimensions;
use crate::ImageDimensions;
use bytemuck::{Pod, Zeroable};
use image_annealing_shaders::constant;
use image_annealing_shaders::WorkgroupDimensions;
use std::fmt;
use std::num::NonZeroU32;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SwapPass {
    Horizontal,
    Vertical,
    OffsetHorizontal,
    OffsetVertical,
}

impl SwapPass {
    pub const STRIDE: usize = 2;

    const OFFSET: usize = 1;

    pub(in super::super) fn swap_workgroup_grid_dimensions(
        &self,
        image_dimensions: &ImageDimensions,
    ) -> WorkgroupGridDimensions {
        let workgroup_dimensions = WorkgroupDimensions::swap();
        let swap_stride = NonZeroU32::new(Self::STRIDE.try_into().unwrap()).unwrap();
        let unit_stride = NonZeroU32::new(1).unwrap();
        match self {
            Self::Horizontal | Self::OffsetHorizontal => {
                WorkgroupGridDimensions::from_image_dimensions_and_stride(
                    &workgroup_dimensions,
                    image_dimensions,
                    swap_stride,
                    unit_stride,
                )
            }
            Self::Vertical | Self::OffsetVertical => {
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
        match self {
            Self::Horizontal => image_dimensions
                .width()
                .checked_div_euclid(Self::STRIDE)
                .unwrap()
                .checked_mul(image_dimensions.height())
                .unwrap(),
            Self::Vertical => image_dimensions
                .height()
                .checked_div_euclid(Self::STRIDE)
                .unwrap()
                .checked_mul(image_dimensions.width())
                .unwrap(),
            Self::OffsetHorizontal => (image_dimensions.width() - Self::OFFSET)
                .checked_div_euclid(Self::STRIDE)
                .unwrap()
                .checked_mul(image_dimensions.height())
                .unwrap(),
            Self::OffsetVertical => (image_dimensions.height() - Self::OFFSET)
                .checked_div_euclid(Self::STRIDE)
                .unwrap()
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

    fn displacement_vector(&self) -> [i32; 2] {
        match self {
            Self::Horizontal | Self::OffsetHorizontal => [1, 0],
            Self::Vertical | Self::OffsetVertical => [0, 1],
        }
    }

    fn offset_vector(&self) -> [i32; 2] {
        match self {
            Self::Horizontal | Self::Vertical => [0, 0],
            Self::OffsetHorizontal => [1, 0],
            Self::OffsetVertical => [0, 1],
        }
    }

    const PASSES: [Self; constant::count_swap::N_CHANNEL] = [
        Self::Horizontal,
        Self::Vertical,
        Self::OffsetHorizontal,
        Self::OffsetVertical,
    ];
}

impl fmt::Display for SwapPass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Horizontal => write!(f, "horizontal swaps, no offset"),
            Self::Vertical => write!(f, "vertical swaps, no offset"),
            Self::OffsetHorizontal => write!(f, "horizontal swaps, with offset"),
            Self::OffsetVertical => write!(f, "vertical swaps, with offset"),
        }
    }
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
        let segment_start = counts
            .iter()
            .take(counts.as_slice().len() - 1)
            .enumerate()
            .fold(
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
#[derive(Clone, Copy, Default, Zeroable, Pod)]
pub struct SwapShaderParameters {
    displacement: [i32; 2],
    offset: [i32; 2],
    count_output_offset: u32,
    acceptance_threshold: f32,
    _padding: [u32; 2],
}

impl SwapShaderParameters {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_acceptance_threshold(&mut self, threshold: f32) {
        self.acceptance_threshold = threshold;
    }

    pub fn set_pass(&mut self, pass: SwapPass, layout: &CountSwapInputLayout) {
        self.displacement = pass.displacement_vector();
        self.offset = pass.offset_vector();
        self.count_output_offset = layout.segment_start[pass as usize];
    }
}

#[cfg(test)]
mod tests;
