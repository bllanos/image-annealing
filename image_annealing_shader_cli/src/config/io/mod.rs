use super::data::{Config, UnverifiedConfig};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn parse_config_file<P: AsRef<Path>>(filename: P) -> Result<Config<'static>, Box<dyn Error>> {
    image_annealing_cli_util::io::check_input_file_path(&filename)?;
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let unverified_config: UnverifiedConfig = serde_json::from_reader(reader)
        .map_err(|e| format!("configuration file deserialization error, \"{}\"", e))?;

    let config = Config::try_from(unverified_config)?;
    Ok(config)
}

#[cfg(test)]
mod tests;
