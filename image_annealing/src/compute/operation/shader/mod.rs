use std::borrow::Cow;

pub mod workgroup;

pub use workgroup::WorkgroupGridDimensions;

pub struct Shader {
    shader: wgpu::ShaderModule,
}

impl Shader {
    pub fn shader(&self) -> &wgpu::ShaderModule {
        &self.shader
    }
}

pub const SHADER_ENTRY_POINT: &str = "main";

pub fn create_permutation_shader(device: &wgpu::Device) -> Shader {
    Shader {
        shader: device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("create_permutation_shader_module"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                "./wgsl/create_permutation.wgsl"
            ))),
            flags: wgpu::ShaderFlags::all(),
        }),
    }
}

pub fn forward_permute_shader(device: &wgpu::Device) -> Shader {
    Shader {
        shader: device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("forward_permute_shader_module"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                "./wgsl/forward_permute.wgsl"
            ))),
            flags: wgpu::ShaderFlags::all(),
        }),
    }
}
