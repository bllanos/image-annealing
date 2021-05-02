// Texel format matches `<PermutationTexture as TextureDatatype>::format()` in src/compute/resource/texture/permutation.rs
[[group(0), binding(0)]]
var output_permutation : [[access(write)]] texture_storage_2d<rgba8uint>;

// `workgroup_size` matches `WORKGROUP_SIZE` in src/compute/operation/shader/workgroup/mod.rs
[[stage(compute), workgroup_size(32, 32, 1)]]
fn main([[builtin(global_invocation_id)]] global_id: vec3<u32>) {
    textureStore(output_permutation, vec2<i32>(global_id.xy), vec4<u32>(0u,0u,0u,0u));
}