use image_annealing::compute;
use image_annealing::compute::format::{
    ImageFileReader, ImageFileWriter, ImageFormat, Rgba16Image, Rgba16ImageBuffer,
};
use image_annealing_cli::cli;
use image_annealing_cli::config::{
    AlgorithmConfig, Config, InputLosslessImagePath, InputPermutationPath, OutputLosslessImagePath,
    UnverifiedInputPermutationPath,
};
use image_annealing_cli_util::path::{InputFilePath, OutputFilePath};
use std::borrow::Cow;
use std::error::Error;
use test_util::permutation::DimensionsAndPermutation;

#[test]
fn permute_valid() -> Result<(), Box<dyn Error>> {
    let path = test_util::unique_absolute_output_file!();
    let full_output_path = Rgba16Image::make_filename(&path.0);
    assert!(!full_output_path.is_file());

    let DimensionsAndPermutation {
        permutation: input_permutation,
        dimensions,
    } = test_util::permutation::bit_interpretation_cases();
    let input_permutation_path_prefix = test_util::unique_absolute_output_file!();
    let input_permutation_path =
        input_permutation.save_add_extension(input_permutation_path_prefix.0)?;

    let input_image = test_util::image::coordinates_to_colors(&dimensions);
    let permuted_image =
        test_util::permutation::bit_interpretation_cases_forward_permute(&input_image);
    let input_image_path_prefix = test_util::unique_absolute_output_file!();
    let input_image_path = input_image.save_add_extension(input_image_path_prefix.0)?;

    let config = Config {
        algorithm: AlgorithmConfig::Permute {
            candidate_permutation: InputPermutationPath(InputFilePath(Cow::Borrowed(
                &input_permutation_path,
            ))),
            original_image: InputLosslessImagePath::Rgba16(InputFilePath(Cow::Borrowed(
                &input_image_path,
            ))),
            permuted_image_output_path_no_extension: OutputLosslessImagePath::Rgba16(path),
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
        InputPermutationPath::try_from_unverified_with_path_context(
            UnverifiedInputPermutationPath(test_util::path::relative_input_file(
                "image/permutation/invalid_permutation.png",
            )),
            test_util::path::base_input().0,
        )?;
    let config = Config {
        algorithm: AlgorithmConfig::Permute {
            candidate_permutation: candidate_permutation_path,
            original_image: InputLosslessImagePath::Rgba8(test_util::path::absolute_input_file(
                "image/image/stripes.png",
            )),
            permuted_image_output_path_no_extension: OutputLosslessImagePath::Rgba8(
                test_util::unique_absolute_output_file!(),
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
        InputPermutationPath::try_from_unverified_with_path_context(
            UnverifiedInputPermutationPath(test_util::path::relative_input_file(
                "image/image/red.png",
            )),
            test_util::path::base_input().0,
        )?;
    let config = Config {
        algorithm: AlgorithmConfig::Permute {
            candidate_permutation: candidate_permutation_path,
            original_image: InputLosslessImagePath::Rgba8(test_util::path::absolute_input_file(
                "image/image/stripes.png",
            )),
            permuted_image_output_path_no_extension: OutputLosslessImagePath::Rgba8(
                test_util::unique_absolute_output_file!(),
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
        InputPermutationPath::try_from_unverified_with_path_context(
            UnverifiedInputPermutationPath(test_util::path::relative_input_file(
                "image/permutation/identity_permutation.png",
            )),
            test_util::path::base_input().0,
        )?;
    let config = Config {
        algorithm: AlgorithmConfig::Permute {
            candidate_permutation: candidate_permutation_path,
            original_image: InputLosslessImagePath::Rgba16(test_util::path::absolute_input_file(
                "image/image/stripes.png",
            )),
            permuted_image_output_path_no_extension: OutputLosslessImagePath::Rgba8(
                test_util::unique_absolute_output_file!(),
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
    let (candidate_permutation_path, image_dimensions) =
        InputPermutationPath::try_from_unverified_with_path_context(
            UnverifiedInputPermutationPath(test_util::path::relative_input_file(
                "image/permutation/identity_permutation.png",
            )),
            test_util::path::base_input().0,
        )?;
    let config = Config {
        algorithm: AlgorithmConfig::Permute {
            candidate_permutation: candidate_permutation_path,
            original_image: InputLosslessImagePath::Rgba8(test_util::path::absolute_input_file(
                "image/image/stripes.png",
            )),
            permuted_image_output_path_no_extension: OutputLosslessImagePath::Rgba8(
                OutputFilePath(Cow::Owned(
                    test_util::path::unverified_absolute_output_path("not_found/cannot_create"),
                )),
            ),
        },
        dispatcher: compute::Config { image_dimensions },
    };
    test_util::assert_error_contains(cli::run(config), "No such file or directory");
    Ok(())
}
