use image_annealing_shader::output;
use std::env;
use std::error::Error;
use std::path::PathBuf;

fn parse_args<T>(args: T) -> Option<PathBuf>
where
    T: IntoIterator<Item = String>,
{
    let mut args_iter = args.into_iter();
    args_iter
        .next()
        .expect("no arguments (not even the program name)");
    args_iter.next().map(PathBuf::from)
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = parse_args(env::args());
    match output::write_default_files(path) {
        Err(err) => {
            eprintln!("Processing error: {}", err);
            Err(err)
        }
        Ok(_) => Ok(()),
    }
}
