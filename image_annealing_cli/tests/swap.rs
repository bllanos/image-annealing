use image_annealing::compute::format::{ImageFileWriter, VectorFieldImageBuffer};
use image_annealing_cli::cli;
use image_annealing_cli::config::Config;
use std::error::Error;

#[test]
fn swap_valid() -> Result<(), Box<dyn Error>> {
    let path = test_utils::make_test_output_path_string(&["cli_swap"]);
    let full_output_path = VectorFieldImageBuffer::make_filename(&path);
    if full_output_path.is_file() {
        panic!("Output permutation already exists in the filesystem.")
    }

    let config = Config::Swap {
        candidate_permutation_path: test_utils::make_test_data_path_string(&[
            "image",
            "permutation",
            "identity_permutation.png",
        ]),
        permutation_output_path_no_extension: path,
    };
    cli::run(config)?;

    if !full_output_path.is_file() {
        panic!("Output permutation does not exist in the filesystem.",)
    }
    std::fs::remove_file(full_output_path)?;

    Ok(())
}

#[test]
fn swap_invalid() {
    let config = Config::Swap {
        candidate_permutation_path: test_utils::make_test_data_path_string(&[
            "image",
            "permutation",
            "invalid_permutation.png",
        ]),
        permutation_output_path_no_extension: test_utils::make_test_output_path_string(&[
            "cli_swap_invalid",
        ]),
    };
    test_utils::assert_error_contains(cli::run(config), "out of bounds mapping (x, y, delta_x, delta_y) = (3, 10, 257, 511) for an image of dimensions (width, height) = (20, 25)");
}
