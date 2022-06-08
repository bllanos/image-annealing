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
  displacement: vec2<i32>;
  offset: vec2<i32>;
  count_output_offset : u32;
  acceptance_threshold: f32;
}};"
    )
}
