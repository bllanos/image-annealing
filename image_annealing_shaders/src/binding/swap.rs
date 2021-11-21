use crate::resource::texture;
use std::io::Write;

pub const PERMUTATION_GROUP_INDEX: u32 = 0;

pub const INPUT_PERMUTATION_INDEX: u32 = 0;

pub const OUTPUT_PERMUTATION_INDEX: u32 = 1;

pub const GUIDE_GROUP_INDEX: u32 = 0;

pub const DISPLACEMENT_GOAL_INDEX: u32 = 0;

pub(crate) fn bind_group<W: Write>(mut writer: W) -> std::io::Result<()> {
    super::binding_annotation(
        &mut writer,
        PERMUTATION_GROUP_INDEX,
        INPUT_PERMUTATION_INDEX,
    )?;
    texture::permutation_input(&mut writer)?;
    super::binding_annotation(
        &mut writer,
        PERMUTATION_GROUP_INDEX,
        OUTPUT_PERMUTATION_INDEX,
    )?;
    texture::permutation_output(&mut writer)?;
    super::binding_annotation(&mut writer, GUIDE_GROUP_INDEX, DISPLACEMENT_GOAL_INDEX)?;
    texture::displacement_goal(&mut writer)
}
