use image_annealing::compute;
use image_annealing::compute::format::{
    ImageFileReader, ImageFileWriter, ImageFormat, Rgba16ImageBuffer, Rgba8Image,
};
use image_annealing_cli::cli;
use image_annealing_cli::config::{
    AlgorithmConfig, Config, ImagePath, LosslessImagePath, PermutationPath,
};
use std::error::Error;
use test_util::permutation::DimensionsAndPermutation;

#[test]
fn permute_valid() -> Result<(), Box<dyn Error>> {
    let path = test_util::make_test_output_path_string(["cli_permute"]);
    let full_output_path = Rgba8Image::make_filename(&path);
    assert!(!full_output_path.is_file());

    let DimensionsAndPermutation {
        permutation: input_permutation,
        dimensions,
    } = test_util::permutation::bit_interpretation_cases();
    let input_permutation_path_prefix =
        test_util::make_test_output_path(["cli_permute_input_permutation"]);
    let input_permutation_path =
        input_permutation.save_add_extension(input_permutation_path_prefix)?;

    let input_image = test_util::image::coordinates_to_colors(&dimensions);
    let permuted_image =
        test_util::permutation::bit_interpretation_cases_forward_permute(&input_image);
    let input_image_path_prefix = test_util::make_test_output_path(["cli_permute_input_image"]);
    let input_image_path = input_image.save_add_extension(input_image_path_prefix)?;

    let config = Config {
        algorithm: AlgorithmConfig::Permute {
            candidate_permutation: PermutationPath::from_raw_clone(
                input_permutation_path.to_str().unwrap(),
            ),
            original_image: LosslessImagePath::Rgba16(String::from(
                input_image_path.to_str().unwrap(),
            )),
            permuted_image_output_path_no_extension: LosslessImagePath::Rgba16(path),
        },
        dispatcher: compute::Config {
            image_dimensions: dimensions,
        },
    };
    cli::run(config)?;

    let output_image = Rgba16ImageBuffer::load(&full_output_path)?;
    assert_eq!(output_image, permuted_image);
    std::fs::remove_file(input_permutation_path)?;
    std::fs::remove_file(input_image_path)?;
    std::fs::remove_file(full_output_path)?;

    Ok(())
}

#[test]
fn permute_invalid() -> Result<(), Box<dyn Error>> {
    let (candidate_permutation_path, image_dimensions) =
        PermutationPath::from_input_path(test_util::make_test_data_path_string([
            "image",
            "permutation",
            "invalid_permutation.png",
        ]))?;
    let config = Config {
        algorithm: AlgorithmConfig::Permute {
            candidate_permutation: candidate_permutation_path,
            original_image: LosslessImagePath::Rgba8(test_util::make_test_data_path_string([
                "image",
                "image",
                "stripes.png",
            ])),
            permuted_image_output_path_no_extension: LosslessImagePath::Rgba8(
                test_util::make_test_output_path_string(["cli_permute_invalid"]),
            ),
        },
        dispatcher: compute::Config { image_dimensions },
    };
    test_util::assert_error_contains(cli::run(config), "out of bounds mapping (x, y, delta_x, delta_y) = (3, 10, 257, 511) for an image of dimensions (width, height) = (20, 25)");
    Ok(())
}

#[test]
fn invalid_permutation_format() -> Result<(), Box<dyn Error>> {
    let (candidate_permutation_path, image_dimensions) =
        PermutationPath::from_input_path(test_util::make_test_data_path_string([
            "image", "image", "red.png",
        ]))?;
    let config = Config {
        algorithm: AlgorithmConfig::Permute {
            candidate_permutation: candidate_permutation_path,
            original_image: LosslessImagePath::Rgba8(test_util::make_test_data_path_string([
                "image",
                "image",
                "stripes.png",
            ])),
            permuted_image_output_path_no_extension: LosslessImagePath::Rgba8(
                test_util::make_test_output_path_string(["cli_permute_invalid_permutation_format"]),
            ),
        },
        dispatcher: compute::Config { image_dimensions },
    };
    test_util::assert_error_contains(
        cli::run(config),
        &format!("not the expected format of {}", ImageFormat::Rgba8),
    );
    Ok(())
}

#[test]
fn invalid_image_format() -> Result<(), Box<dyn Error>> {
    let (candidate_permutation_path, image_dimensions) =
        PermutationPath::from_input_path(test_util::make_test_data_path_string([
            "image",
            "permutation",
            "identity_permutation.png",
        ]))?;
    let config = Config {
        algorithm: AlgorithmConfig::Permute {
            candidate_permutation: candidate_permutation_path,
            original_image: LosslessImagePath::Rgba16(test_util::make_test_data_path_string([
                "image",
                "image",
                "stripes.png",
            ])),
            permuted_image_output_path_no_extension: LosslessImagePath::Rgba8(
                test_util::make_test_output_path_string(["cli_permute_invalid_image_format"]),
            ),
        },
        dispatcher: compute::Config { image_dimensions },
    };
    test_util::assert_error_contains(
        cli::run(config),
        &format!("not the expected format of {}", ImageFormat::Rgba16),
    );
    Ok(())
}

#[test]
fn save_missing_directory() -> Result<(), Box<dyn Error>> {
    let path = test_util::make_test_output_path_string(["not_found", "cannot_create"]);
    let (candidate_permutation_path, image_dimensions) =
        PermutationPath::from_input_path(test_util::make_test_data_path_string([
            "image",
            "permutation",
            "identity_permutation.png",
        ]))?;
    let config = Config {
        algorithm: AlgorithmConfig::Permute {
            candidate_permutation: candidate_permutation_path,
            original_image: LosslessImagePath::Rgba8(test_util::make_test_data_path_string([
                "image",
                "image",
                "stripes.png",
            ])),
            permuted_image_output_path_no_extension: LosslessImagePath::Rgba8(path),
        },
        dispatcher: compute::Config { image_dimensions },
    };
    test_util::assert_error_contains(cli::run(config), "No such file or directory");
    Ok(())
}
