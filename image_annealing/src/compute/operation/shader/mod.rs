pub mod workgroup;

pub use workgroup::WorkgroupDimensions;

pub struct Shader {
    shader: wgpu::ShaderModule,
}

impl Shader {
    pub fn shader(&self) -> &wgpu::ShaderModule {
        &self.shader
    }
}

pub fn create_permutation_shader(device: &wgpu::Device) -> Shader {
    Shader {
        shader: device.create_shader_module(&wgpu::include_spirv!(
            "./glsl/main/create_permutation.comp.spv"
        )),
    }
}
