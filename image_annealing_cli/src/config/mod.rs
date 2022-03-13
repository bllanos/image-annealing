use std::error::Error;

mod data;
mod io;

use data::Config;
use io::parse_config_file;
pub use io::FileNotFoundError;

pub fn parse_args<T>(args: T) -> Result<Config, Box<dyn Error>>
where
    T: IntoIterator<Item = String>,
{
    let mut args_iter = args.into_iter();
    args_iter
        .next()
        .expect("no arguments (not even the program name)");
    let filename = args_iter
        .next()
        .ok_or("expected at least one argument for a configuration file's path")?;
    parse_config_file(&filename)
}

// The module could also be implemented in this file
#[cfg(test)]
mod tests;
