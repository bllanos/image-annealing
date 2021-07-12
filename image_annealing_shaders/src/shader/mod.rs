use std::io::Write;

use crate::binding::{create_permutation, permute};
use crate::compute;
use crate::function::{conversion, io, main};

pub fn create_permutation<W: Write>(mut writer: W) -> std::io::Result<()> {
    create_permutation::bind_group(&mut writer)?;
    compute::compute_shader_annotation(&mut writer, Default::default())?;
    main::create_permutation(&mut writer)
}

fn permute_common<W: Write>(mut writer: W) -> std::io::Result<()> {
    permute::bind_group(&mut writer)?;
    conversion::u16_to_i32(&mut writer)?;
    io::load_permutation_vector(&mut writer)?;
    compute::compute_shader_annotation(&mut writer, Default::default())
}

pub fn forward_permute<W: Write>(mut writer: W) -> std::io::Result<()> {
    permute_common(&mut writer)?;
    main::forward_permute(&mut writer)
}
