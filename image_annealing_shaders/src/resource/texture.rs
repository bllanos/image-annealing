use std::io::Write;

const LOSSLESS_IMAGE_TEXEL_FORMAT: &str = "rgba32uint";

const PERMUTATION_TEXEL_FORMAT: &str = "rgba8uint";

pub fn permutation_input<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "var input_permutation : texture_storage_2d<{},read>;",
        PERMUTATION_TEXEL_FORMAT
    )
}

pub fn permutation_output<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "var output_permutation : texture_storage_2d<{},write>;",
        PERMUTATION_TEXEL_FORMAT
    )
}

pub fn lossless_image_input<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "var input_image : texture_storage_2d<{},read>;",
        LOSSLESS_IMAGE_TEXEL_FORMAT
    )
}

pub fn lossless_image_output<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "var output_image : texture_storage_2d<{},write>;",
        LOSSLESS_IMAGE_TEXEL_FORMAT
    )
}
