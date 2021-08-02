use std::io::Write;

pub fn load_permutation_vector<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "fn load_permutation_vector(coords : vec2<i32>) -> vec2<i32> {{
  let raw_texel : vec4<u32> = textureLoad(input_permutation, coords, 0);
  return vec2<i32>(u16_to_i32(raw_texel.r, raw_texel.g), u16_to_i32(raw_texel.b, raw_texel.a));
}}"
    )
}
