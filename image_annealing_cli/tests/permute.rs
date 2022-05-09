use image_annealing::compute;
use image_annealing::compute::format::{ImageFileWriter, ImageFormat, Rgba8Image};
use image_annealing_cli::cli;
use image_annealing_cli::config::{AlgorithmConfig, Config, LosslessImagePath, PermutationPath};
use std::error::Error;

#[test]
fn permute_valid() -> Result<(), Box<dyn Error>> {
    let path = test_utils::make_test_output_path_string(&["cli_permute"]);
    let full_output_path = Rgba8Image::make_filename(&path);
    if full_output_path.is_file() {
        panic!("Output permuted image already exists in the filesystem.")
    }
    let (candidate_permutation_path, image_dimensions) =
        PermutationPath::from_input_path(test_utils::make_test_data_path_string(&[
            "image",
            "permutation",
            "identity_permutation.png",
        ]))?;
    let config =
        Config {
            algorithm: AlgorithmConfig::Permute {
                candidate_permutation: candidate_permutation_path,
                original_image: LosslessImagePath::Rgba8(test_utils::make_test_data_path_string(
                    &["image", "image", "stripes.png"],
                )),
                permuted_image_output_path_no_extension: LosslessImagePath::Rgba8(path),
            },
            dispatcher: compute::Config { image_dimensions },
        };
    cli::run(config)?;

    if !full_output_path.is_file() {
        panic!("Output permuted image does not exist in the filesystem.",)
    }
    std::fs::remove_file(full_output_path)?;

    Ok(())
}

#[test]
fn permute_invalid() -> Result<(), Box<dyn Error>> {
    let (candidate_permutation_path, image_dimensions) =
        PermutationPath::from_input_path(test_utils::make_test_data_path_string(&[
            "image",
            "permutation",
            "invalid_permutation.png",
        ]))?;
    let config =
        Config {
            algorithm: AlgorithmConfig::Permute {
                candidate_permutation: candidate_permutation_path,
                original_image: LosslessImagePath::Rgba8(test_utils::make_test_data_path_string(
                    &["image", "image", "stripes.png"],
                )),
                permuted_image_output_path_no_extension: LosslessImagePath::Rgba8(
                    test_utils::make_test_output_path_string(&["cli_permute_invalid"]),
                ),
            },
            dispatcher: compute::Config { image_dimensions },
        };
    test_utils::assert_error_contains(cli::run(config), "out of bounds mapping (x, y, delta_x, delta_y) = (3, 10, 257, 511) for an image of dimensions (width, height) = (20, 25)");
    Ok(())
}

#[test]
fn invalid_permutation_format() -> Result<(), Box<dyn Error>> {
    let (candidate_permutation_path, image_dimensions) =
        PermutationPath::from_input_path(test_utils::make_test_data_path_string(&[
            "image", "image", "red.png",
        ]))?;
    let config =
        Config {
            algorithm: AlgorithmConfig::Permute {
                candidate_permutation: candidate_permutation_path,
                original_image: LosslessImagePath::Rgba8(test_utils::make_test_data_path_string(
                    &["image", "image", "stripes.png"],
                )),
                permuted_image_output_path_no_extension: LosslessImagePath::Rgba8(
                    test_utils::make_test_output_path_string(&[
                        "cli_permute_invalid_permutation_format",
                    ]),
                ),
            },
            dispatcher: compute::Config { image_dimensions },
        };
    test_utils::assert_error_contains(
        cli::run(config),
        &format!("not the expected format of {}", ImageFormat::Rgba8),
    );
    Ok(())
}

#[test]
fn invalid_image_format() -> Result<(), Box<dyn Error>> {
    let (candidate_permutation_path, image_dimensions) =
        PermutationPath::from_input_path(test_utils::make_test_data_path_string(&[
            "image",
            "permutation",
            "identity_permutation.png",
        ]))?;
    let config =
        Config {
            algorithm: AlgorithmConfig::Permute {
                candidate_permutation: candidate_permutation_path,
                original_image: LosslessImagePath::Rgba16(test_utils::make_test_data_path_string(
                    &["image", "image", "stripes.png"],
                )),
                permuted_image_output_path_no_extension: LosslessImagePath::Rgba8(
                    test_utils::make_test_output_path_string(&["cli_permute_invalid_image_format"]),
                ),
            },
            dispatcher: compute::Config { image_dimensions },
        };
    test_utils::assert_error_contains(
        cli::run(config),
        &format!("not the expected format of {}", ImageFormat::Rgba16),
    );
    Ok(())
}

#[test]
fn save_missing_directory() -> Result<(), Box<dyn Error>> {
    let path = test_utils::make_test_output_path_string(&["not_found", "cannot_create"]);
    let (candidate_permutation_path, image_dimensions) =
        PermutationPath::from_input_path(test_utils::make_test_data_path_string(&[
            "image",
            "permutation",
            "identity_permutation.png",
        ]))?;
    let config =
        Config {
            algorithm: AlgorithmConfig::Permute {
                candidate_permutation: candidate_permutation_path,
                original_image: LosslessImagePath::Rgba8(test_utils::make_test_data_path_string(
                    &["image", "image", "stripes.png"],
                )),
                permuted_image_output_path_no_extension: LosslessImagePath::Rgba8(path),
            },
            dispatcher: compute::Config { image_dimensions },
        };
    test_utils::assert_error_contains(cli::run(config), "No such file or directory");
    Ok(())
}
