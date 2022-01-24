use std::io::Write;

pub fn partial_sum<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "var<workgroup> partial_sum : array<vec4<f32>, workgroup_invocations>;"
    )
}
