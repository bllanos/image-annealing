use image_annealing::image_utils::ImageDimensions;
use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct FileNotFoundError {
    filepath: PathBuf,
}

impl fmt::Display for FileNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "File {} does not exist in the filesystem.",
            self.filepath.display()
        )
    }
}

impl Error for FileNotFoundError {}

#[derive(Deserialize)]
enum UnverifiedConfig {
    CreatePermutationConfig {
        image_width: usize,
        image_height: usize,
        permutation_output_path_no_extension: String,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Config {
    CreatePermutationConfig {
        image_dimensions: ImageDimensions,
        permutation_output_path_no_extension: String,
    },
}

pub fn parse_args<T>(args: T) -> Result<Config, Box<dyn Error>>
where
    T: IntoIterator<Item = String>,
{
    let mut args_iter = args.into_iter();
    args_iter
        .next()
        .expect("No arguments (not even the program name)");
    let filename = args_iter
        .next()
        .ok_or("Expected at least one argument for a configuration file's path.")?;
    parse_config_file(&filename)
}

pub fn parse_config_file<P: AsRef<Path>>(filename: P) -> Result<Config, Box<dyn Error>> {
    check_input_path(&filename)?;
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let unverified_config = serde_json::from_reader(reader)?;

    let config = match unverified_config {
        UnverifiedConfig::CreatePermutationConfig {
            image_width,
            image_height,
            permutation_output_path_no_extension,
        } => Config::CreatePermutationConfig {
            image_dimensions: ImageDimensions::new(image_width, image_height)?,
            permutation_output_path_no_extension,
        },
    };
    Ok(config)
}

fn check_input_path<P: AsRef<Path>>(filepath: P) -> Result<(), FileNotFoundError> {
    let filepath = filepath.as_ref();
    if !filepath.is_file() {
        Err(FileNotFoundError {
            filepath: filepath.to_path_buf(),
        })
    } else {
        Ok(())
    }
}

// The module could also be implemented in this file
#[cfg(test)]
mod tests;
