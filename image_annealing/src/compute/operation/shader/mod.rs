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

pub fn count_swap_shader(device: &wgpu::Device) -> Shader {
    Shader {
        shader: device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("count_swap_shader_module"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(env!(
                "COUNT_SWAP_SHADER"
            )))),
        }),
    }
}

pub fn create_displacement_goal_default_shader(device: &wgpu::Device) -> Shader {
    Shader {
        shader: device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("create_displacement_goal_default_shader_module"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(env!(
                "CREATE_DISPLACEMENT_GOAL_DEFAULT_SHADER"
            )))),
        }),
    }
}

pub fn create_permutation_shader(device: &wgpu::Device) -> Shader {
    Shader {
        shader: device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("create_permutation_shader_module"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(env!(
                "CREATE_PERMUTATION_SHADER"
            )))),
        }),
    }
}

pub fn permute_shader(device: &wgpu::Device) -> Shader {
    Shader {
        shader: device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("permute_shader_module"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(env!("PERMUTE_SHADER")))),
        }),
    }
}

pub fn swap_shader(device: &wgpu::Device) -> Shader {
    Shader {
        shader: device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("swap_shader_module"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(env!("SWAP_SHADER")))),
        }),
    }
}
