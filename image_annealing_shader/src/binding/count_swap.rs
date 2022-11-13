use crate::resource::{buffer, uniform};
use std::io::Write;

pub const GROUP_INDEX: u32 = 0;

pub const PARAMETERS_INDEX: u32 = 0;

pub const INPUT_BUFFER_INDEX: u32 = 1;

pub const OUTPUT_BUFFER_INDEX: u32 = 2;

pub(crate) fn bind_group<W: Write>(mut writer: W) -> std::io::Result<()> {
    super::binding_annotation(&mut writer, GROUP_INDEX, PARAMETERS_INDEX)?;
    uniform::count_swap_parameters(&mut writer)?;
    super::binding_annotation(&mut writer, GROUP_INDEX, INPUT_BUFFER_INDEX)?;
    buffer::count_swap_input(&mut writer)?;
    super::binding_annotation(&mut writer, GROUP_INDEX, OUTPUT_BUFFER_INDEX)?;
    buffer::count_swap_output(&mut writer)
}
