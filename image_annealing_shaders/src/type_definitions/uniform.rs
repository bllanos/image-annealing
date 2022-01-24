use std::io::Write;

pub fn swap_count_parameters<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "struct InputLayout {{
  do_segment : vec4<u32>;
  segment_start : vec4<u32>;
  segment_end : vec4<u32>;
}};"
    )
}
