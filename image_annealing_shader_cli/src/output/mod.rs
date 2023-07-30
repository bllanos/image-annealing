use image_annealing_cli_util::path::{
    OutputFilePath, TryIntoWithPathContext, UnverifiedOutputFilePath,
};
use image_annealing_shader::shader;
use relative_path::RelativePath;
use std::borrow::Cow;
use std::default::Default;
use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct OutputConfig<'a> {
    pub count_swap: Option<OutputFilePath<'a>>,
    pub create_displacement_goal_default: Option<OutputFilePath<'a>>,
    pub create_permutation: Option<OutputFilePath<'a>>,
    pub permute: Option<OutputFilePath<'a>>,
    pub swap: Option<OutputFilePath<'a>>,
}

impl<'a> OutputConfig<'a> {
    pub fn with_base_directory<P: AsRef<Path>>(
        directory: Option<P>,
    ) -> Result<Self, Box<dyn Error>> {
        let path = directory
            .as_ref()
            .map_or_else(|| Path::new("."), <P as AsRef<Path>>::as_ref);
        Ok(Self {
            count_swap: Some(
                UnverifiedOutputFilePath(Cow::Borrowed(RelativePath::new("count_swap.wgsl")))
                    .try_into_with_path_context(path)?,
            ),
            create_displacement_goal_default: Some(
                UnverifiedOutputFilePath(Cow::Borrowed(RelativePath::new(
                    "create_displacement_goal_default.wgsl",
                )))
                .try_into_with_path_context(path)?,
            ),
            create_permutation: Some(
                UnverifiedOutputFilePath(Cow::Borrowed(RelativePath::new(
                    "create_permutation.wgsl",
                )))
                .try_into_with_path_context(path)?,
            ),
            permute: Some(
                UnverifiedOutputFilePath(Cow::Borrowed(RelativePath::new("permute.wgsl")))
                    .try_into_with_path_context(path)?,
            ),
            swap: Some(
                UnverifiedOutputFilePath(Cow::Borrowed(RelativePath::new("swap.wgsl")))
                    .try_into_with_path_context(path)?,
            ),
        })
    }
}

pub fn write_files(config: &OutputConfig) -> std::io::Result<()> {
    if let Some(path) = config.count_swap.as_ref() {
        let mut f = File::create(&path.0)?;
        shader::count_swap(&mut f)?;
    }
    if let Some(path) = config.create_displacement_goal_default.as_ref() {
        let mut f = File::create(&path.0)?;
        shader::create_displacement_goal_default(&mut f)?;
    }
    if let Some(path) = config.create_permutation.as_ref() {
        let mut f = File::create(&path.0)?;
        shader::create_permutation(&mut f)?;
    }
    if let Some(path) = config.permute.as_ref() {
        let mut f = File::create(&path.0)?;
        shader::permute(&mut f)?;
    }
    if let Some(path) = config.swap.as_ref() {
        let mut f = File::create(&path.0)?;
        shader::swap(&mut f)?;
    }
    Ok(())
}

pub fn write_default_files<'a, P: AsRef<Path>>(
    directory: Option<P>,
) -> Result<OutputConfig<'a>, Box<dyn Error>> {
    let config = OutputConfig::with_base_directory(directory)?;
    write_files(&config)?;
    Ok(config)
}

#[cfg(test)]
mod tests;
