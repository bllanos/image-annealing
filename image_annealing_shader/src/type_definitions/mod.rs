use std::io::Write;

mod uniform;

pub fn count_swap<W: Write>(mut writer: W) -> std::io::Result<()> {
    uniform::count_swap_parameters(&mut writer)
}

pub fn swap<W: Write>(mut writer: W) -> std::io::Result<()> {
    uniform::swap_parameters(&mut writer)
}
