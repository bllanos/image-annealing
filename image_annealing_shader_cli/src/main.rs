use image_annealing_shader_cli::args::{self, Options};
use image_annealing_shader_cli::output;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let Options { output_directory } = args::make_option_parser().run();
    match output::write_default_files(output_directory) {
        Err(err) => {
            eprintln!("Processing error: {}", err);
            Err(err)
        }
        Ok(_) => Ok(()),
    }
}
