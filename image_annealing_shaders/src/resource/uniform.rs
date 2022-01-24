use std::io::Write;

pub fn count_swap_parameters<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(writer, "var<uniform> parameters : InputLayout;")
}
