use crate::compute::WorkgroupDimensions;
use std::io::Write;

pub mod count_swap;

pub fn workgroup_invocations<W: Write>(
    mut writer: W,
    workgroup_dimensions: WorkgroupDimensions,
) -> std::io::Result<()> {
    writeln!(
        writer,
        "let workgroup_invocations: u32 = {}u;",
        workgroup_dimensions.invocation_count()
    )
}
