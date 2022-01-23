use crate::constant;
use std::io::Write;

pub const SHADER_ENTRY_POINT: &str = "main";

fn global_invocation_id_header<W: Write>(mut writer: W) -> std::io::Result<()> {
  writeln!(
    writer,
    "fn {}([[builtin(global_invocation_id)]] global_id: vec3<u32>) {{",
    SHADER_ENTRY_POINT
  )
}

pub fn create_permutation<W: Write>(mut writer: W) -> std::io::Result<()> {
  global_invocation_id_header(&mut writer)?;
  writeln!(
    writer,
    "  textureStore(output_permutation, vec2<i32>(global_id.xy), vec4<u32>(0u,0u,0u,0u));
}}"
  )
}

pub fn forward_permute<W: Write>(mut writer: W) -> std::io::Result<()> {
  global_invocation_id_header(&mut writer)?;
  writeln!(
    writer,
    "  let coords : vec2<i32> = vec2<i32>(global_id.xy);
  let permutation_vector : vec2<i32> = load_permutation_vector(coords);
  let image_texel : vec4<u32> = textureLoad(input_image, coords + permutation_vector, 0);
  textureStore(output_image, coords, image_texel);
}}"
  )
}

pub fn swap<W: Write>(mut writer: W) -> std::io::Result<()> {
  global_invocation_id_header(&mut writer)?;
  writeln!(
    writer,
    "  let coords1 : vec2<i32> = vec2<i32>(i32(global_id.x * {}u), i32(global_id.y));
  let displacement : vec2<i32> = vec2<i32>(1, 0);
  let coords2 : vec2<i32> = coords1 + displacement;
  let dimensions : vec2<i32> = textureDimensions(input_permutation);
  if(coords1.x < dimensions.x && coords1.y < dimensions.y) {{
    let input_permutation_vector1 : vec2<i32> = load_permutation_vector(coords1);
    var output_permutation_vector1 : vec2<i32> = input_permutation_vector1;

    if(coords2.x < dimensions.x && coords2.y < dimensions.y) {{
      let input_permutation_vector2 : vec2<i32> = load_permutation_vector(coords2);
      var output_permutation_vector2 : vec2<i32> = input_permutation_vector2;

      if(swap_cost(coords1, displacement, input_permutation_vector1, input_permutation_vector2) < 0.0) {{
        output_permutation_vector1 = input_permutation_vector2 + displacement;
        output_permutation_vector2 = input_permutation_vector1 - displacement;
      }}

      store_permutation_vector(coords2, output_permutation_vector2);
    }}

    store_permutation_vector(coords1, output_permutation_vector1);
  }}
}}", constant::swap::STRIDE
  )
}
