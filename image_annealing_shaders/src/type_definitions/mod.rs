use std::io::Write;

mod buffer;
mod uniform;

pub fn count_swap<W: Write>(mut writer: W) -> std::io::Result<()> {
    uniform::swap_count_parameters(&mut writer)?;
    buffer::count_swap_input(&mut writer)?;
    buffer::count_swap_output(&mut writer)
}
