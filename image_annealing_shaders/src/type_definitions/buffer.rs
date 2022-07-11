use std::io::Write;

pub fn count_swap_input<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "struct Input {{
  arr: array<f32>,
}}"
    )
}

pub fn count_swap_output<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "struct Output {{
  arr: array<vec4<f32>>,
}}"
    )
}

pub fn swap_count_output<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "struct CountOutput {{
  arr: array<f32>,
}}"
    )
}
