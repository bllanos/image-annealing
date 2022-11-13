use std::io::Write;

fn load_vector_field_vector<W: Write>(mut writer: W, subtype: &str) -> std::io::Result<()> {
    writeln!(
        writer,
        "fn load_{subtype}_vector(coords : vec2<i32>) -> vec2<i32> {{
  let raw_texel : vec4<u32> = textureLoad(input_{subtype}, coords, 0);
  return vec2<i32>(u16_to_i32(raw_texel.r, raw_texel.g), u16_to_i32(raw_texel.b, raw_texel.a));
}}",
        subtype = subtype
    )
}

pub fn load_displacement_goal_vector<W: Write>(mut writer: W) -> std::io::Result<()> {
    load_vector_field_vector(&mut writer, "displacement_goal")
}

pub fn load_permutation_vector<W: Write>(mut writer: W) -> std::io::Result<()> {
    load_vector_field_vector(&mut writer, "permutation")
}

fn store_vector_field_vector<W: Write>(mut writer: W, subtype: &str) -> std::io::Result<()> {
    writeln!(
        writer,
        "fn store_{subtype}_vector(coords : vec2<i32>, displacement : vec2<i32>) {{
  let raw_texel : vec4<u32> = vec4<u32>(i32_to_u16(displacement.x), i32_to_u16(displacement.y));
  textureStore(output_{subtype}, coords, raw_texel);
}}",
        subtype = subtype
    )
}

pub fn store_displacement_goal_vector<W: Write>(mut writer: W) -> std::io::Result<()> {
    store_vector_field_vector(&mut writer, "displacement_goal")
}

pub fn store_permutation_vector<W: Write>(mut writer: W) -> std::io::Result<()> {
    store_vector_field_vector(&mut writer, "permutation")
}
