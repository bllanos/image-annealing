// Texel format matches `<PermutationTexture as TextureDatatype>::format()` in src/compute/resource/texture/permutation.rs
[[group(0), binding(0)]]
var input_permutation : [[access(read)]] texture_storage_2d<rgba8uint>;

// Texel format matches `<LosslessImageTexture as TextureDatatype>::format()` in src/compute/resource/texture/lossless_image.rs
[[group(0), binding(1)]]
var input_image : [[access(read)]] texture_storage_2d<rgba32uint>;

// Texel format matches `<LosslessImageTexture as TextureDatatype>::format()` in src/compute/resource/texture/lossless_image.rs
[[group(0), binding(2)]]
var output_image : [[access(write)]] texture_storage_2d<rgba32uint>;

// Reinterpret the two values as the first and second bytes of a big-endian
// 16-bit two's-complement value
fn u16_to_i32(x1 : u32, x2 : u32) -> i32 {
    let x : u32 = (x1 << 8u) | x2;
    if((x & (1u << 15u)) == 0u) {
        return i32(x);
    } else {
        // This is a negative i16 number that must be represented as a negative i32 number
        return i32(x | (~u32(0u) << 16u));
    }
}

fn load_permutation_vector(coords : vec2<i32>) -> vec2<i32> {
    let raw_texel : vec4<u32> = textureLoad(input_permutation, coords);
    return vec2<i32>(u16_to_i32(raw_texel.r, raw_texel.g), u16_to_i32(raw_texel.b, raw_texel.a));
}

// `workgroup_size` matches `WORKGROUP_SIZE` in src/compute/operation/shader/workgroup/mod.rs
[[stage(compute), workgroup_size(32, 32, 1)]]
fn main([[builtin(global_invocation_id)]] global_id: vec3<u32>) {
    let coords : vec2<i32> = vec2<i32>(global_id.xy);
    let permutation_vector : vec2<i32> = load_permutation_vector(coords);
    let image_texel : vec4<u32> = textureLoad(input_image, coords + permutation_vector);
    textureStore(output_image, coords, image_texel);
}