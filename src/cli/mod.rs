use crate::compute::format::ImageFileWriter;
use crate::compute::{
    self, CreatePermutationInput, CreatePermutationParameters, Dispatcher, OutputStatus,
};
use crate::config::Config;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let dispatcher = create_dispatcher(&config)?;
    run_and_save(dispatcher, &config)?;
    Ok(())
}

fn create_dispatcher(config: &Config) -> Result<Box<dyn Dispatcher>, Box<dyn Error>> {
    match config {
        Config::CreatePermutationConfig {
            image_dimensions, ..
        } => compute::create_dispatcher(image_dimensions),
    }
}

fn run_and_save(dispatcher: Box<dyn Dispatcher>, config: &Config) -> Result<(), Box<dyn Error>> {
    match config {
        Config::CreatePermutationConfig {
            permutation_output_path_no_extension: path,
            ..
        } => {
            let mut algorithm = dispatcher
                .create_permutation(&CreatePermutationInput {}, &CreatePermutationParameters {});
            if let OutputStatus::FinalFullOutput = algorithm.step()? {
                let img = algorithm.full_output().unwrap();
                let output_path = img.save_add_extension(path)?;
                println!("Wrote permutation to: {}", output_path.display());
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
