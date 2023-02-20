// Copy a byte of each image channel to the output displacement goal texture
@compute @workgroup_size(16, 16, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
  let coords : vec2<i32> = vec2<i32>(global_id.xy);
  let image_texel : vec4<u32> = textureLoad(input_image, coords, 0);
  let byte_index = 0u;
  let shift = byte_index * 8u;
  let mask = 255u << shift;
  let output_texel : vec4<u32> = vec4<u32>(
    (image_texel.r & mask) >> shift,
    (image_texel.g & mask) >> shift,
    (image_texel.b & mask) >> shift,
    (image_texel.a & mask) >> shift
  );
  textureStore(output_displacement_goal, coords, output_texel);
}