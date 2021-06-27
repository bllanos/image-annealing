use std::io::Write;

use crate::binding::create_permutation;
use crate::compute;
use crate::function::main;

pub fn create_permutation<W: Write>(mut writer: W) -> std::io::Result<()> {
    create_permutation::bind_group(&mut writer)?;
    compute::compute_shader_annotation(&mut writer, Default::default())?;
    main::create_permutation(&mut writer)
}
