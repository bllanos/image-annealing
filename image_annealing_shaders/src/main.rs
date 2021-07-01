use image_annealing_shaders::output;
use std::env;
use std::path::PathBuf;
use std::process;

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

fn main() {
    let path = parse_args(env::args());
    if let Err(err) = output::write_default_files(path) {
        eprintln!("Processing error: {}", err);
        process::exit(1);
    }
}
