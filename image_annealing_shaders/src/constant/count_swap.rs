use std::io::Write;

pub const N_CHANNEL: usize = 4;

pub fn n_channel<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(writer, "let n_channel: u32 = {}u;", N_CHANNEL)
}
