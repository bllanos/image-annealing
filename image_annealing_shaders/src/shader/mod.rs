use crate::binding::{count_swap, create_permutation, permute, swap};
use crate::compute::{self, WorkgroupDimensions};
use crate::constant;
use crate::function::{self, conversion, io, main};
use crate::global;
use crate::type_definitions;
use std::io::Write;

mod create_displacement_goal;

pub use create_displacement_goal::{
    create_displacement_goal_custom, create_displacement_goal_default,
    CreateDisplacementGoalShaderContent,
};

pub fn create_permutation<W: Write>(mut writer: W) -> std::io::Result<()> {
    create_permutation::bind_group(&mut writer)?;
    conversion::i32_to_u16(&mut writer)?;
    io::store_permutation_vector(&mut writer)?;
    compute::compute_shader_annotation(&mut writer, WorkgroupDimensions::create_permutation())?;
    main::create_permutation(&mut writer)
}

fn permute_common<W: Write>(mut writer: W) -> std::io::Result<()> {
    permute::bind_group(&mut writer)?;
    conversion::u16_to_i32(&mut writer)?;
    io::load_permutation_vector(&mut writer)?;
    compute::compute_shader_annotation(&mut writer, WorkgroupDimensions::permute())
}

pub fn permute<W: Write>(mut writer: W) -> std::io::Result<()> {
    permute_common(&mut writer)?;
    main::forward_permute(&mut writer)
}

pub fn swap<W: Write>(mut writer: W) -> std::io::Result<()> {
    let workgroup_dimensions = WorkgroupDimensions::swap();
    type_definitions::swap(&mut writer)?;
    swap::bind_group(&mut writer)?;
    constant::workgroup_invocations(&mut writer, workgroup_dimensions)?;
    global::partial_scalar_sum(&mut writer)?;
    function::workgroup::reduce_partial_sum(&mut writer, workgroup_dimensions.invocation_count())?;
    conversion::u16_to_i32(&mut writer)?;
    conversion::i32_to_u16(&mut writer)?;
    io::load_permutation_vector(&mut writer)?;
    io::store_permutation_vector(&mut writer)?;
    io::load_displacement_goal_vector(&mut writer)?;
    function::swap::potential_energy(&mut writer)?;
    function::swap::displacement_cost(&mut writer)?;
    function::swap::swap_cost(&mut writer)?;
    compute::compute_shader_annotation(&mut writer, workgroup_dimensions)?;
    main::swap(&mut writer)
}

pub fn count_swap<W: Write>(mut writer: W) -> std::io::Result<()> {
    let workgroup_dimensions = WorkgroupDimensions::count_swap();
    type_definitions::count_swap(&mut writer)?;
    count_swap::bind_group(&mut writer)?;
    constant::count_swap::n_channel(&mut writer)?;
    constant::workgroup_invocations(&mut writer, workgroup_dimensions)?;
    global::partial_vector_sum(&mut writer)?;
    function::workgroup::reduce_partial_sum(&mut writer, workgroup_dimensions.invocation_count())?;
    compute::compute_shader_annotation(&mut writer, workgroup_dimensions)?;
    main::count_swap(&mut writer)
}
