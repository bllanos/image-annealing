//! References:
//! https://github.com/sotrh/learn-wgpu
//! https://github.com/gfx-rs/naga/blob/master/bin/convert.rs

use anyhow::*;
use glob::glob;
use rayon::prelude::*;
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

fn print_err(error: impl Error) {
    eprintln!("\t{}:", error);
    let mut e = error.source();
    while let Some(source) = e {
        eprintln!("\t\t{}", source);
        e = source.source();
    }
}

struct ShaderData {
    src: String,
    src_path: PathBuf,
}

impl ShaderData {
    pub fn load(src_path: PathBuf) -> Result<Self> {
        let src = read_to_string(&src_path)?;
        Ok(Self { src, src_path })
    }
}

fn main() -> Result<()> {
    // Collect all shaders recursively within /src/
    let mut shader_paths = Vec::new();
    shader_paths.extend(glob("./src/**/*.wgsl")?);

    let shaders = shader_paths
        .into_par_iter()
        .map(|glob_result| ShaderData::load(glob_result?))
        .collect::<Vec<Result<_>>>()
        .into_iter()
        .collect::<Result<Vec<_>>>()?;

    // Shader validation might be parallelized in the future.
    // Ideally, only changed shaders would be validated.
    let mut validator = naga::valid::Validator::new(naga::valid::ValidationFlags::all());
    for shader in shaders {
        println!(
            "cargo:rerun-if-changed={}",
            shader.src_path.as_os_str().to_str().unwrap()
        );

        let module = match naga::front::wgsl::parse_str(&shader.src) {
            Ok(v) => v,
            Err(ref e) => {
                eprintln!("Unable to parse WGSL from {}:", shader.src_path.display());
                print_err(e);
                bail!("Unable to parse WGSL");
            }
        };

        if let Err(ref error) = validator.validate(&module) {
            eprintln!(
                "Validation of WGSL from '{}' failed:",
                shader.src_path.display()
            );
            print_err(error);
            bail!("Validation of WGSL failed");
        }
    }

    Ok(())
}
