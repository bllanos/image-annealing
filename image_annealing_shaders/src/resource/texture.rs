use std::io::Write;

pub fn permutation_input<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(writer, "var input_permutation : texture_2d<u32>;",)
}

pub fn permutation_output<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "var output_permutation : [[access(write)]] texture_storage_2d<rgba8uint>;"
    )
}

pub fn lossless_image_input<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(writer, "var input_image : texture_2d<u32>;")
}

pub fn lossless_image_output<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "var output_image : [[access(write)]] texture_storage_2d<rgba32uint>;"
    )
}
