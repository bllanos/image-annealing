use std::io::Write;

mod buffer;
mod uniform;

pub fn count_swap<W: Write>(mut writer: W) -> std::io::Result<()> {
    uniform::count_swap_parameters(&mut writer)?;
    buffer::count_swap_input(&mut writer)?;
    buffer::count_swap_output(&mut writer)
}

pub fn swap<W: Write>(mut writer: W) -> std::io::Result<()> {
    uniform::swap_parameters(&mut writer)?;
    buffer::swap_count_output(&mut writer)
}
