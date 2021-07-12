use std::io::Write;

const LOSSLESS_IMAGE_TEXEL_FORMAT: &str = "rgba32uint";

const PERMUTATION_TEXEL_FORMAT: &str = "rgba8uint";

pub fn permutation_input<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "var input_permutation : [[access(read)]] texture_storage_2d<{}>;",
        PERMUTATION_TEXEL_FORMAT
    )
}

pub fn permutation_output<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "var output_permutation : [[access(write)]] texture_storage_2d<{}>;",
        PERMUTATION_TEXEL_FORMAT
    )
}

pub fn lossless_image_input<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "var input_image : [[access(read)]] texture_storage_2d<{}>;",
        LOSSLESS_IMAGE_TEXEL_FORMAT
    )
}

pub fn lossless_image_output<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "var output_image : [[access(write)]] texture_storage_2d<{}>;",
        LOSSLESS_IMAGE_TEXEL_FORMAT
    )
}
