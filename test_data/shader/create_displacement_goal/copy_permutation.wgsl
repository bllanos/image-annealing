@compute @workgroup_size(16, 16, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
  let coords : vec2<i32> = vec2<i32>(global_id.xy);
  let permutation_texel : vec4<u32> = textureLoad(input_permutation, coords, 0);
  textureStore(output_displacement_goal, coords, permutation_texel);
}