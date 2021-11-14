use image_annealing::compute::DimensionsMismatchError;
use image_annealing::ImageDimensions;
use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::path::{self, Path, PathBuf};

#[derive(Debug, Clone)]
pub struct FileNotFoundError {
    filepath: PathBuf,
}

impl fmt::Display for FileNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "file {} does not exist in the filesystem",
            self.filepath.display()
        )
    }
}

impl Error for FileNotFoundError {}

#[derive(Deserialize)]
enum UnverifiedConfig {
    CreatePermutation {
        image_width: usize,
        image_height: usize,
        permutation_output_path_no_extension: String,
    },
    Permute {
        candidate_permutation_path: String,
        original_image_path: String,
        permuted_image_output_path_no_extension: String,
    },
    Swap {
        candidate_permutation_path: String,
        permutation_output_path_no_extension: String,
    },
    ValidatePermutation {
        candidate_permutation_path: String,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Config {
    CreatePermutation {
        image_dimensions: ImageDimensions,
        permutation_output_path_no_extension: String,
    },
    Permute {
        candidate_permutation_path: String,
        original_image_path: String,
        permuted_image_output_path_no_extension: String,
    },
    Swap {
        candidate_permutation_path: String,
        permutation_output_path_no_extension: String,
    },
    ValidatePermutation {
        candidate_permutation_path: String,
    },
}

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

pub fn parse_config_file<P: AsRef<Path>>(filename: P) -> Result<Config, Box<dyn Error>> {
    check_input_path(&filename)?;
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let unverified_config = serde_json::from_reader(reader)
        .map_err(|e| format!("configuration file deserialization error, \"{}\"", e))?;

    let config = match unverified_config {
        UnverifiedConfig::CreatePermutation {
            image_width,
            image_height,
            permutation_output_path_no_extension,
        } => Config::CreatePermutation {
            image_dimensions: ImageDimensions::new(image_width, image_height)?,
            permutation_output_path_no_extension: convert_path_separators(
                permutation_output_path_no_extension,
            ),
        },
        UnverifiedConfig::Permute {
            candidate_permutation_path,
            original_image_path,
            permuted_image_output_path_no_extension,
        } => {
            let candidate_permutation_path_checked =
                convert_and_check_input_path(candidate_permutation_path)?;
            let original_image_path_checked = convert_and_check_input_path(original_image_path)?;
            let image_dimensions = ImageDimensions::from_image_path(&original_image_path_checked)?;
            let permutation_dimensions =
                ImageDimensions::from_image_path(&candidate_permutation_path_checked)?;
            if image_dimensions == permutation_dimensions {
                Config::Permute {
                    candidate_permutation_path: candidate_permutation_path_checked,
                    original_image_path: original_image_path_checked,
                    permuted_image_output_path_no_extension: convert_path_separators(
                        permuted_image_output_path_no_extension,
                    ),
                }
            } else {
                return Err(Box::new(DimensionsMismatchError::new(
                    image_dimensions,
                    permutation_dimensions,
                )));
            }
        }
        UnverifiedConfig::Swap {
            candidate_permutation_path,
            permutation_output_path_no_extension,
        } => Config::Swap {
            candidate_permutation_path: convert_and_check_input_path(candidate_permutation_path)?,
            permutation_output_path_no_extension: convert_path_separators(
                permutation_output_path_no_extension,
            ),
        },
        UnverifiedConfig::ValidatePermutation {
            candidate_permutation_path,
        } => Config::ValidatePermutation {
            candidate_permutation_path: convert_and_check_input_path(candidate_permutation_path)?,
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

fn convert_path_separators(filepath: String) -> String {
    if <String as AsRef<Path>>::as_ref(&filepath).is_absolute() {
        filepath
    } else {
        let new_separator = path::MAIN_SEPARATOR.to_string();
        filepath.replace(&['\\', '/'][..], &new_separator)
    }
}

fn convert_and_check_input_path(filepath: String) -> Result<String, FileNotFoundError> {
    let new_path = convert_path_separators(filepath);
    check_input_path(&new_path)?;
    Ok(new_path)
}

// The module could also be implemented in this file
#[cfg(test)]
mod tests;
