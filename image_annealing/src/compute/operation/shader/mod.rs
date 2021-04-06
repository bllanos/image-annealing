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
    let mut shader_descriptor = wgpu::include_spirv!("./glsl/main/create_permutation.comp.spv");
    shader_descriptor.flags = wgpu::ShaderFlags::empty(); // TODO Re-enable validation
    Shader {
        shader: device.create_shader_module(&shader_descriptor),
    }
}
