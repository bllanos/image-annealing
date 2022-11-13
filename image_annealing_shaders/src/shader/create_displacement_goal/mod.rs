use crate::binding;
use crate::compute::{self, WorkgroupDimensions};
use crate::function::{conversion, io, main};
use std::borrow::Cow;
use std::io::Write;

fn create_displacement_goal_common<W: Write>(mut writer: W) -> std::io::Result<()> {
    binding::create_displacement_goal::bind_group(&mut writer)?;
    conversion::i32_to_u16(&mut writer)?;
    io::store_displacement_goal_vector(&mut writer)
}

fn create_displacement_goal_preamble<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "// Begin preamble provided by {}",
        crate::crate_name()
    )?;
    create_displacement_goal_common(&mut writer)?;
    writeln!(writer, "// End preamble")
}

pub fn create_displacement_goal_default<W: Write>(mut writer: W) -> std::io::Result<()> {
    create_displacement_goal_common(&mut writer)?;
    compute::compute_shader_annotation(
        &mut writer,
        WorkgroupDimensions::create_displacement_goal_default(),
    )?;
    main::create_displacement_goal_default(&mut writer)
}

#[derive(Debug, Eq, PartialEq)]
pub struct CreateDisplacementGoalShaderContent<'a> {
    body: Cow<'a, str>,
}

pub fn create_displacement_goal_custom<W: Write>(
    mut writer: W,
    content: &CreateDisplacementGoalShaderContent,
) -> std::io::Result<()> {
    create_displacement_goal_preamble(&mut writer)?;
    writer.write_all(content.body.as_bytes())?;
    writeln!(writer)
}
