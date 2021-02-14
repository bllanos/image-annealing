use image_annealing_cli::{cli, config};
use std::env;
use std::process;

fn main() {
    let parse_result = config::parse_args(env::args());
    match parse_result {
        Err(err) => {
            eprintln!("Configuration error: {}", err);
            process::exit(1);
        }
        Ok(parsed_config) => {
            if let Err(err) = cli::run(parsed_config) {
                eprintln!("Processing error: {}", err);
                process::exit(1);
            }
        }
    }
}
