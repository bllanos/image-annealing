use std::io::Write;

pub mod create_permutation;
pub mod permute;

fn binding_annotation<W: Write>(mut writer: W, group: u32, binding: u32) -> std::io::Result<()> {
    writeln!(writer, "[[group({}), binding({})]]", group, binding)
}
