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
        "  let coords_left : vec2<i32> = vec2<i32>(i32(global_id.x * 2u), i32(global_id.y));
  let coords_right : vec2<i32> = vec2<i32>(coords_left.x + 1, coords_left.y);
  let dimensions : vec2<i32> = textureDimensions(input_permutation);
  if(coords_left.x < dimensions.x && coords_left.y < dimensions.y) {{
    let permutation_vector_left : vec2<i32> = load_permutation_vector(coords_left);
    var permutation_vector_right : vec2<i32> = permutation_vector_left;

    if(coords_right.x < dimensions.x && coords_right.y < dimensions.y) {{
        permutation_vector_right = load_permutation_vector(coords_right) + vec2<i32>(1, 0);
        store_permutation_vector(coords_right, permutation_vector_left + vec2<i32>(-1, 0));
    }}

    store_permutation_vector(coords_left, permutation_vector_right);
  }}
}}"
    )
}
