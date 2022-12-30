use image_annealing_shader::shader;
use std::borrow::Cow;
use std::default::Default;
use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct OutputConfig<'a> {
    pub count_swap: Option<Cow<'a, Path>>,
    pub create_displacement_goal_default: Option<Cow<'a, Path>>,
    pub create_permutation: Option<Cow<'a, Path>>,
    pub permute: Option<Cow<'a, Path>>,
    pub swap: Option<Cow<'a, Path>>,
}

impl<'a> OutputConfig<'a> {
    pub fn with_base_directory<P: AsRef<Path>>(
        directory: Option<P>,
    ) -> Result<Self, Box<dyn Error>> {
        let path = directory
            .as_ref()
            .map_or_else(|| Path::new("."), <P as AsRef<Path>>::as_ref);
        image_annealing_cli_util::io::check_directory_path(path)?;
        Ok(Self {
            count_swap: Some(Cow::from(path.join("count_swap.wgsl"))),
            create_displacement_goal_default: Some(Cow::from(
                path.join("create_displacement_goal_default.wgsl"),
            )),
            create_permutation: Some(Cow::from(path.join("create_permutation.wgsl"))),
            permute: Some(Cow::from(path.join("permute.wgsl"))),
            swap: Some(Cow::from(path.join("swap.wgsl"))),
        })
    }
}

pub fn write_files(config: &OutputConfig) -> std::io::Result<()> {
    if let Some(path) = config.count_swap.as_ref() {
        let mut f = File::create(path)?;
        shader::count_swap(&mut f)?;
    }
    if let Some(path) = config.create_displacement_goal_default.as_ref() {
        let mut f = File::create(path)?;
        shader::create_displacement_goal_default(&mut f)?;
    }
    if let Some(path) = config.create_permutation.as_ref() {
        let mut f = File::create(path)?;
        shader::create_permutation(&mut f)?;
    }
    if let Some(path) = config.permute.as_ref() {
        let mut f = File::create(path)?;
        shader::permute(&mut f)?;
    }
    if let Some(path) = config.swap.as_ref() {
        let mut f = File::create(path)?;
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
