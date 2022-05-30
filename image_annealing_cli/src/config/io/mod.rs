use super::data::{Config, UnverifiedConfig};
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

pub fn parse_config_file<P: AsRef<Path>>(filename: P) -> Result<Config, Box<dyn Error>> {
    check_input_path(&filename)?;
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let unverified_config: UnverifiedConfig = serde_json::from_reader(reader)
        .map_err(|e| format!("configuration file deserialization error, \"{}\"", e))?;

    let config = Config::try_from(unverified_config)?;
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

pub fn convert_path_separators<T: AsRef<str>>(filepath: T) -> String {
    let filepath = String::from(filepath.as_ref());
    if <String as AsRef<Path>>::as_ref(&filepath).is_absolute() {
        filepath
    } else {
        let new_separator = path::MAIN_SEPARATOR.to_string();
        filepath.replace(&['\\', '/'][..], &new_separator)
    }
}

pub fn convert_and_check_input_path<T: AsRef<str>>(
    filepath: T,
) -> Result<String, FileNotFoundError> {
    let new_path = convert_path_separators(filepath);
    check_input_path(&new_path)?;
    Ok(new_path)
}

#[cfg(test)]
mod tests;
