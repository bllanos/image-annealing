use image_annealing_shader_cli::{args, cli};

fn main() {
    let options = args::make_option_parser().run();
    if let Err(err) = cli::run(&options) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
