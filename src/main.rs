use std::env;
use std::error::Error;

use std::path::Path;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().peekable();
    args.next()
        .expect("No command-line arguments (not even the program name)");
    let filename = args
        .next()
        .ok_or("Expected at least one command-line argument for an image filename.")?;
    println!("Filename: {}", filename);
    if let Some(_) = args.peek() {
        println!("Additional arguments:");
        for argument in args {
            println!("\t{}", argument);
        }
    }

    let filepath = Path::new(&filename);
    if !filepath.is_file() {
        Err(format!(
            "Image file '{}' does not exist in the filesystem.",
            filename
        ))?
    }
    let output_filename = format!(
        "{}_out{}",
        filepath
            .file_stem()
            .unwrap()
            .to_str()
            .expect("The path should be valid UTF-8 because it is from std::env::args()."),
        match filepath.extension() {
            Some(val) => format!(
                ".{}",
                val.to_str().expect(
                    "The extension should be valid UTF-8 because it is from std::env::args()."
                )
            ),
            None => String::new(),
        }
    );
    let mut output_path = PathBuf::new();
    if let Some(parent) = filepath.parent() {
        output_path.push(parent);
    }
    output_path.push(output_filename);

    let img = image::open(filepath)?;

    println!("Saving image to: {}", output_path.to_str().unwrap());
    img.save(output_path)?;

    Ok(())
}
