use std::io::Write;

mod workgroup;

pub use workgroup::WorkgroupDimensions;

pub fn compute_shader_annotation<W: Write>(
    mut writer: W,
    workgroup_dimensions: WorkgroupDimensions,
) -> std::io::Result<()> {
    writeln!(
        writer,
        "[[stage(compute), workgroup_size({}, {}, {})]]",
        workgroup_dimensions.x(),
        workgroup_dimensions.y(),
        workgroup_dimensions.z()
    )
}
