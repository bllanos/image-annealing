use crate::resource::texture;
use std::io::Write;

pub const GROUP_INDEX: u32 = 0;

pub const OUTPUT_PERMUTATION_INDEX: u32 = 0;

pub(crate) fn bind_group<W: Write>(mut writer: W) -> std::io::Result<()> {
    super::binding_annotation(&mut writer, GROUP_INDEX, OUTPUT_PERMUTATION_INDEX)?;
    texture::permutation_output(&mut writer)
}
