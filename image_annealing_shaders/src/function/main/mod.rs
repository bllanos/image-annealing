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
  let image_texel : vec4<u32> = textureLoad(input_image, coords + permutation_vector);
  textureStore(output_image, coords, image_texel);
}}"
    )
}
