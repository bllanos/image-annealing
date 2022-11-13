use std::io::Write;

pub mod count_swap;
pub mod create_displacement_goal;
pub mod create_permutation;
pub mod permute;
pub mod swap;

fn binding_annotation<W: Write>(mut writer: W, group: u32, binding: u32) -> std::io::Result<()> {
    writeln!(writer, "@group({}) @binding({})", group, binding)
}
