use std::io::Write;

pub fn count_swap_parameters<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "struct InputLayout {{
  do_segment : vec4<u32>;
  segment_start : vec4<u32>;
  segment_end : vec4<u32>;
}};"
    )
}

pub fn swap_parameters<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "struct Parameters {{
  count_output_offset : u32;
  _padding : vec3<u32>;
}};"
    )
}
