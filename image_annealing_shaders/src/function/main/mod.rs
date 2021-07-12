use std::io::Write;

pub const SHADER_ENTRY_POINT: &str = "main";

fn global_invocation_id_header<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "fn {}([[builtin(global_invocation_id)]] global_id: vec3<u32>) {{",
        SHADER_ENTRY_POINT
    )
}

fn footer<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(writer, "}}")
}

pub fn create_permutation<W: Write>(mut writer: W) -> std::io::Result<()> {
    global_invocation_id_header(&mut writer)?;
    writeln!(
        writer,
        "  textureStore(output_permutation, vec2<i32>(global_id.xy), vec4<u32>(0u,0u,0u,0u));",
    )?;
    footer(&mut writer)
}
