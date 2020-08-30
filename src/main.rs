use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().peekable();
    args.next()
        .expect("No command-line arguments (not even the program name)");
    let filename = match args.next() {
        Some(val) => val,
        None => Err("Expected at least one command-line argument for an image filename")?,
    };
    println!("Filename: {}", filename);
    if let Some(_) = args.peek() {
        println!("Additional arguments:");
        for argument in args {
            println!("\t{}", argument);
        }
    }

    Ok(())
}
