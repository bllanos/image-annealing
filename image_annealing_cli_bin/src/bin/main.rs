use image_annealing_cli::args::{self, ParseFailure};
use image_annealing_cli::cli;
use std::env;
use std::io::{self, Write};

fn main() {
    match args::parse_args(env::args()) {
        Err(output) => match output {
            ParseFailure::Stdout(message) => {
                io::stdout().write_all(message.as_bytes()).unwrap();
            }
            ParseFailure::Stderr(message) => {
                eprintln!("{}", message);
            }
        },
        Ok(parsed_config) => {
            if let Err(err) = cli::run(parsed_config) {
                eprintln!("Processing error: {}", err);
                std::process::exit(1);
            }
        }
    }
}
