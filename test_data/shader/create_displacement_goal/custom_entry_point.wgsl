@compute @workgroup_size(16, 16, 1)
fn entry_point(@builtin(global_invocation_id) global_id: vec3<u32>) {
  let coords : vec2<i32> = vec2<i32>(global_id.xy);
  textureStore(output_displacement_goal, coords, vec4<u32>(1u, 2u, 3u, 4u));
}