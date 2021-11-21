use std::io::Write;

use crate::binding::{create_permutation, permute, swap};
use crate::compute;
use crate::function::{self, conversion, io, main};

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

pub fn permute<W: Write>(mut writer: W) -> std::io::Result<()> {
    permute_common(&mut writer)?;
    main::forward_permute(&mut writer)
}

pub fn swap<W: Write>(mut writer: W) -> std::io::Result<()> {
    swap::bind_group(&mut writer)?;
    conversion::u16_to_i32(&mut writer)?;
    conversion::i32_to_u16(&mut writer)?;
    io::load_permutation_vector(&mut writer)?;
    io::store_permutation_vector(&mut writer)?;
    io::load_displacement_goal_vector(&mut writer)?;
    function::swap::potential_energy(&mut writer)?;
    function::swap::displacement_cost(&mut writer)?;
    function::swap::swap_cost(&mut writer)?;
    compute::compute_shader_annotation(&mut writer, Default::default())?;
    main::swap(&mut writer)
}
