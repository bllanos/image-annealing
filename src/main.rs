use std::env;
use std::error::Error;
use std::fmt;
use std::process;

use std::path::Path;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let parse_result = parse_args();
    if let Err(err) = parse_result {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    }

    let (filename, rect) = parse_result.unwrap();

    rect.iter()
        .for_each(|x| println!("Image rectangle is {:?}", x));

    let filepath = check_input_path(&filename)?;
    let output_path = make_output_filepath(&filepath)?;
    let img = image::open(filepath)?;

    println!("Saving image to: {}", output_path.to_str().unwrap());
    img.save(output_path)?;

    Ok(())
}

#[derive(Debug, Clone)]
struct InvalidCornersError;

impl fmt::Display for InvalidCornersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "invalid specification of top left and bottom right image rectangle corners"
        )
    }
}

impl Error for InvalidCornersError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
struct Rectangle {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

const N_CORNERS: usize = 4;

impl Rectangle {
    fn from_corners(x1: u32, y1: u32, x2: u32, y2: u32) -> Result<Rectangle, InvalidCornersError> {
        if x1 <= x2 && y1 <= y2 {
            Ok(Rectangle {
                x: x1,
                y: y1,
                width: x2 - x1,
                height: y2 - y1,
            })
        } else {
            Err(InvalidCornersError)
        }
    }
}

fn parse_args() -> Result<(String, Option<Rectangle>), Box<dyn Error>> {
    let mut args = env::args();
    args.next()
        .expect("No command-line arguments (not even the program name)");
    let filename = args
        .next()
        .ok_or("Expected at least one command-line argument for an image filename.")?;
    if let Some(arg) = args.next() {
        if arg == "--rect" {
            let v: Vec<_> = args
                .take(N_CORNERS)
                .map(|x| x.parse::<u32>())
                .filter(|x| x.is_ok())
                .map(Result::unwrap)
                .collect();
            if v.len() == N_CORNERS {
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

fn check_input_path(filename: &String) -> Result<&Path, Box<dyn Error>> {
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

fn make_output_filepath(input_filepath: &Path) -> Result<PathBuf, Box<dyn Error>> {
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
    }
    output_path.push(output_filename);
    Ok(output_path)
}
