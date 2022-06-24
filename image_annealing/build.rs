use image_annealing_shaders::output;
use std::env;

fn main() {
    let directory = env::var("OUT_DIR").unwrap();
    let config = output::write_default_files(Some(directory)).unwrap();
    println!(
        "cargo:rustc-env=COUNT_SWAP_SHADER={}",
        config.count_swap.as_ref().unwrap().display()
    );
    println!(
        "cargo:rustc-env=CREATE_PERMUTATION_SHADER={}",
        config.create_permutation.as_ref().unwrap().display()
    );
    println!(
        "cargo:rustc-env=PERMUTE_SHADER={}",
        config.permute.as_ref().unwrap().display()
    );
    println!(
        "cargo:rustc-env=SWAP_SHADER={}",
        config.swap.as_ref().unwrap().display()
    );

    println!("cargo:rerun-if-changed=build.rs");
}
