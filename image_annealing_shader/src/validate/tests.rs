mod validate_shader {
    use super::super::validate_shader;
    use std::error::Error;

    #[test]
    fn parse_error() {
        let shader_source = "@compute @workgroup_size(16, 16, 1)
fn main() {
  let coords : vec2<i32> = 0;
}";
        test_util::assert_error_contains(validate_shader(shader_source), "unable to parse WGSL");
    }

    #[test]
    fn validation_error() {
        let shader_source = "@group(0) @binding(0)
var output_permutation : texture_2d<u32>;
@compute @workgroup_size(16, 16, 1)
fn main() {
  textureStore(output_permutation, vec2<i32>(0, 0), vec4<u32>(0u, 0u, 0u, 0u));
}";
        test_util::assert_error_contains(
            validate_shader(shader_source),
            "shader module validation failed",
        );
    }

    #[test]
    fn valid() -> Result<(), Box<dyn Error>> {
        let shader_source = "@group(0) @binding(0)
var output_permutation : texture_storage_2d<rgba8uint, write>;
@compute @workgroup_size(16, 16, 1)
fn main() {
  textureStore(output_permutation, vec2<i32>(0, 0), vec4<u32>(0u, 0u, 0u, 0u));
}";
        validate_shader(shader_source)?;
        Ok(())
    }
}
