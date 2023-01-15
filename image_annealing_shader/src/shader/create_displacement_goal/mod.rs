use crate::binding;
use crate::compute::{self, WorkgroupDimensions};
use crate::function::{conversion, io, main};
use std::borrow::Cow;
use std::io::Write;

fn create_displacement_goal_resources<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "// Begin resources provided by {}",
        crate::crate_name()
    )?;
    binding::create_displacement_goal::bind_group(&mut writer)?;
    writeln!(writer, "// End resources")
}

pub fn create_displacement_goal_default<W: Write>(mut writer: W) -> std::io::Result<()> {
    create_displacement_goal_resources(&mut writer)?;
    conversion::i32_to_u16(&mut writer)?;
    io::store_displacement_goal_vector(&mut writer)?;
    compute::compute_shader_annotation(
        &mut writer,
        WorkgroupDimensions::create_displacement_goal_default(),
    )?;
    main::create_displacement_goal_default(&mut writer)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateDisplacementGoalShaderContent<'a> {
    pub body: Cow<'a, str>,
}

impl CreateDisplacementGoalShaderContent<'_> {
    // Ideally, this would be a method, but it may not be possible to return a `Self` type parameterized with a lifetime.
    // (See https://stackoverflow.com/questions/57701914/trait-method-which-returns-self-type-with-a-different-type-and-or-lifetime-par)
    pub fn to_owned<'a>(
        instance: &CreateDisplacementGoalShaderContent<'a>,
    ) -> CreateDisplacementGoalShaderContent<'static> {
        CreateDisplacementGoalShaderContent {
            body: Cow::Owned(instance.body.clone().into_owned()),
        }
    }
}

pub fn create_displacement_goal_custom<W: Write>(
    mut writer: W,
    content: &CreateDisplacementGoalShaderContent,
) -> std::io::Result<()> {
    create_displacement_goal_resources(&mut writer)?;
    writer.write_all(content.body.as_bytes())?;
    writeln!(writer)
}
