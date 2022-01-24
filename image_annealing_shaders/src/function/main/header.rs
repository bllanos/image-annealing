use std::io::Write;

pub const SHADER_ENTRY_POINT: &str = "main";

pub fn global_invocation_id_header<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "fn {}([[builtin(global_invocation_id)]] global_id: vec3<u32>) {{",
        SHADER_ENTRY_POINT
    )
}

pub fn count_swap_header<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
    writer,
    "fn {}([[builtin(workgroup_id)]] workgroup_id: vec3<u32>, [[builtin(local_invocation_index)]] local_id: u32, [[builtin(num_workgroups)]] num_workgroups: vec3<u32>) {{",
    SHADER_ENTRY_POINT
  )
}
