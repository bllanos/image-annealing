use image_annealing_cli::{cli, config};
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    match config::parse_args(env::args()) {
        Err(err) => {
            eprintln!("Configuration error: {}", err);
            Err(err)
        }
        Ok(parsed_config) => match cli::run(parsed_config) {
            Err(err) => {
                eprintln!("Processing error: {}", err);
                Err(err)
            }
            Ok(_) => Ok(()),
        },
    }
}
