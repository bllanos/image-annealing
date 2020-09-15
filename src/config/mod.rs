use crate::image_utils::rectangle::{self, Rectangle};
use std::env;
use std::error::Error;
use std::path::Path;
use std::path::PathBuf;

pub fn parse_args() -> Result<(String, Option<Rectangle>), Box<dyn Error>> {
    let mut args = env::args();
    args.next()
        .expect("No command-line arguments (not even the program name)");
    let filename = args
        .next()
        .ok_or("Expected at least one command-line argument for an image filename.")?;
    if let Some(arg) = args.next() {
        if arg == "--rect" {
            let v: Vec<_> = args
                .take(rectangle::N_CORNERS)
                .map(|x| x.parse::<u32>())
                .filter(|x| x.is_ok())
                .map(Result::unwrap)
                .collect();
            if v.len() == rectangle::N_CORNERS {
                let rect = Rectangle::from_corners(v[0], v[1], v[2], v[3])?;
                Ok((filename, Some(rect)))
            } else {
                Err("Failed to parse corners for image rectangle")?
            }
        } else {
            Ok((filename, None))
        }
    } else {
        Ok((filename, None))
    }
}

pub fn check_input_path(filename: &String) -> Result<&Path, Box<dyn Error>> {
    let filepath = Path::new(filename);
    if !filepath.is_file() {
        Err(format!(
            "Image file {} does not exist in the filesystem.",
            filename
        ))?
    } else {
        Ok(filepath)
    }
}

pub fn make_output_filepath(input_filepath: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let output_filename = format!(
        "{}_out{}",
        input_filepath
            .file_stem()
            .ok_or("The input filepath is expected to have a stem.")?
            .to_str()
            .ok_or("The file stem of the input filepath is expected to be valid UTF-8.")?,
        match input_filepath.extension() {
            Some(val) => format!(
                ".{}",
                val.to_str().ok_or(
                    "The file extension of the input filepath is expected to be valid UTF-8."
                )?
            ),
            None => String::new(),
        }
    );
    let mut output_path = PathBuf::new();
    if let Some(parent) = input_filepath.parent() {
        output_path.push(parent);
    }    output_path.push(output_filename);
    Ok(output_path)
}
