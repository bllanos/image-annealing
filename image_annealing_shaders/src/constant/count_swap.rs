use crate::compute::WorkgroupDimensions;
use std::io::Write;

pub const N_CHANNEL: u32 = 4;

pub fn n_channel<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(writer, "let n_channel: u32 = {}u;", N_CHANNEL)
}

pub fn workgroup_invocations<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "let workgroup_invocations: u32 = {}u;",
        WorkgroupDimensions::count_swap().invocation_count()
    )
}
