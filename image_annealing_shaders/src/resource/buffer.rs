use std::io::Write;

pub fn count_swap_input<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(writer, "var<storage, read> input : array<f32>;")
}

pub fn count_swap_output<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "var<storage, read_write> output : array<vec4<f32>>;"
    )
}

pub fn swap_count_output<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "var<storage, read_write> count_output : array<f32>;"
    )
}
