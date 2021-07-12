use image_annealing_shaders::output;
use std::env;

fn main() {
    let directory = env::var("OUT_DIR").unwrap();
    let mut config = output::write_default_files(Some(directory)).unwrap();
    println!(
        "cargo:rustc-env=CREATE_PERMUTATION_SHADER={}",
        config.create_permutation.take().unwrap().display()
    );

    println!("cargo:rerun-if-changed=build.rs");
}
