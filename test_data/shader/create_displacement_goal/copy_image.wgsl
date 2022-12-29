fn i32_to_u16(x_signed: i32) -> vec2<u32> {
  let x : u32 = u32(x_signed);
  return vec2<u32>((x >> 8u) & 255u, x & 255u);
}
fn store_displacement_goal_vector(coords : vec2<i32>, displacement : vec2<i32>) {
  let raw_texel : vec4<u32> = vec4<u32>(i32_to_u16(displacement.x), i32_to_u16(displacement.y));
  textureStore(output_displacement_goal, coords, raw_texel);
}
// Copy the first byte of each image channel to the output displacement goal texture
@compute @workgroup_size(16, 16, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
  let coords : vec2<i32> = vec2<i32>(global_id.xy);
  let image_texel : vec4<u32> = textureLoad(input_image, coords, 0);
  store_displacement_goal_vector(
    coords,
    vec2<i32>(
      i32((image_texel.r & 255u) | ((image_texel.g & 255u) << 8u)),
      i32((image_texel.b & 255u) | ((image_texel.a & 255u) << 8u))
    )
  );
}