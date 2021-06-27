use std::io::Write;

const PERMUTATION_TEXEL_FORMAT: &str = "rgba8uint";

pub fn permutation_output<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "var output_permutation : [[access(write)]] texture_storage_2d<{}>;",
        PERMUTATION_TEXEL_FORMAT
    )
}
