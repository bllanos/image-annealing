use std::io::Write;

pub fn count_swap_input<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(writer, "var<storage, read> input : Input;")
}

pub fn count_swap_output<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(writer, "var<storage, read_write> output : Output;")
}
