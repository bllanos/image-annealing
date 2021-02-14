//! Reference: https://github.com/sotrh/learn-wgpu

use anyhow::*;
use glob::glob;
use rayon::prelude::*;
use std::fs::{read_to_string, write};
use std::path::{Path, PathBuf};

struct ShaderData {
    src: String,
    src_path: PathBuf,
    spv_path: PathBuf,
    kind: shaderc::ShaderKind,
}

impl ShaderData {
    pub fn load(src_path: PathBuf) -> Result<Self> {
        let extension = src_path
            .extension()
            .context("File has no extension")?
            .to_str()
            .context("Extension cannot be converted to a string")?;
        let kind = match extension {
            "vert" => shaderc::ShaderKind::Vertex,
            "frag" => shaderc::ShaderKind::Fragment,
            "comp" => shaderc::ShaderKind::Compute,
            _ => bail!("Unsupported shader extension: {}", src_path.display()),
        };

        let src = read_to_string(&src_path)?;
        let spv_path = src_path.with_extension(format!("{}.spv", extension));

        Ok(Self {
            src,
            src_path,
            spv_path,
            kind,
        })
    }
}

// Reference: https://github.com/Wumpf/blub/blob/master/src/wgpu_utils/shader.rs
fn include_directive_callback(
    name: &str,
    include_type: shaderc::IncludeType,
    source_file: &str,
    _depth: usize,
) -> shaderc::IncludeCallbackResult {
    match include_type {
        shaderc::IncludeType::Relative => {
            let path = Path::new(Path::new(source_file).parent().unwrap()).join(name);
            match std::fs::read_to_string(&path) {
                Ok(glsl_code) => {
                    println!(
                        "cargo:rerun-if-changed={}",
                        path.as_os_str().to_str().unwrap()
                    );
                    Ok(shaderc::ResolvedInclude {
                        resolved_name: String::from(name),
                        content: glsl_code,
                    })
                }
                Err(err) => Err(format!(
                    "Failed to resolve include to {} in {} (was looking for {:?}): {}",
                    name, source_file, path, err
                )),
            }
        }
        _ => Err(format!(
            "Absolute include directives are not yet supported. Include to {} in {}.",
            name, source_file
        )),
    }
}

fn main() -> Result<()> {
    // Collect all shaders recursively within /src/
    let mut shader_paths = Vec::new();
    shader_paths.extend(glob("./src/**/*.vert")?);
    shader_paths.extend(glob("./src/**/*.frag")?);
    shader_paths.extend(glob("./src/**/*.comp")?);

    let shaders = shader_paths
        .into_par_iter()
        .map(|glob_result| ShaderData::load(glob_result?))
        .collect::<Vec<Result<_>>>()
        .into_iter()
        .collect::<Result<Vec<_>>>()?;

    let mut compiler = shaderc::Compiler::new().context("Unable to create shader compiler")?;
    let mut options =
        shaderc::CompileOptions::new().context("Unable to create shader compiler options")?;
    options.set_warnings_as_errors();
    options.set_generate_debug_info();
    options.set_include_callback(include_directive_callback);

    // Shader compilation cannot be parallelized. The [shaderc::Compiler] is not
    // thread safe, and consumes significant resources.
    // The best option would be to compile only what has changed
    for shader in shaders {
        println!(
            "cargo:rerun-if-changed={}",
            shader.src_path.as_os_str().to_str().unwrap()
        );

        let compiled = compiler.compile_into_spirv(
            &shader.src,
            shader.kind,
            &shader.src_path.to_str().unwrap(),
            image_annealing_build_utils::SHADER_ENTRY_POINT,
            Some(&options),
        )?;
        write(shader.spv_path, compiled.as_binary_u8())?;
    }

    Ok(())
}
