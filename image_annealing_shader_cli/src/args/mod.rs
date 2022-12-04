use bpaf::{Bpaf, ShellComp};
use std::path::PathBuf;

#[derive(Debug, Bpaf, Eq, PartialEq)]
#[bpaf(generate(make_option_parser), options, version)]
/// Generate or validate shader files
pub struct Options {
    /// Path of the directory into which to output built-in shaders
    #[bpaf(long, short('d'), argument("DIRECTORY"), complete_shell(ShellComp::Dir { mask: None }))]
    pub output_directory: Option<PathBuf>,
}

#[cfg(test)]
mod tests;
