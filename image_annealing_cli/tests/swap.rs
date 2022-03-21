use image_annealing::compute;
use image_annealing::compute::format::{ImageFileWriter, VectorFieldImageBuffer};
use image_annealing_cli::cli;
use image_annealing_cli::config::{AlgorithmConfig, Config, DisplacementGoalPath, PermutationPath};
use std::error::Error;

#[test]
fn swap_valid() -> Result<(), Box<dyn Error>> {
    let path = test_utils::make_test_output_path_string(&["cli_swap"]);
    let full_output_path = VectorFieldImageBuffer::make_filename(&path);
    if full_output_path.is_file() {
        panic!("Output permutation already exists in the filesystem.")
    }

    let (candidate_permutation_path, image_dimensions) =
        PermutationPath::from_input_path(test_utils::make_test_data_path_string(&[
            "image",
            "permutation",
            "identity_permutation.png",
        ]))?;
    let config = Config {
        algorithm: AlgorithmConfig::Swap {
            candidate_permutation: candidate_permutation_path,
            displacement_goal: DisplacementGoalPath(test_utils::make_test_data_path_string(&[
                "image",
                "displacement_goal",
                "identity_displacement_goal.png",
            ])),
            permutation_output_path_no_extension: PermutationPath(path),
        },
        dispatcher: compute::Config { image_dimensions },
    };
    cli::run(config)?;

    if !full_output_path.is_file() {
        panic!("Output permutation does not exist in the filesystem.",)
    }
    std::fs::remove_file(full_output_path)?;

    Ok(())
}

#[test]
fn swap_invalid() -> Result<(), Box<dyn Error>> {
    let (candidate_permutation_path, image_dimensions) =
        PermutationPath::from_input_path(test_utils::make_test_data_path_string(&[
            "image",
            "permutation",
            "invalid_permutation.png",
        ]))?;
    let config = Config {
        algorithm: AlgorithmConfig::Swap {
            candidate_permutation: candidate_permutation_path,
            displacement_goal: DisplacementGoalPath(test_utils::make_test_data_path_string(&[
                "image",
                "displacement_goal",
                "identity_displacement_goal.png",
            ])),
            permutation_output_path_no_extension: PermutationPath(
                test_utils::make_test_output_path_string(&["cli_swap_invalid"]),
            ),
        },
        dispatcher: compute::Config { image_dimensions },
    };
    test_utils::assert_error_contains(cli::run(config), "out of bounds mapping (x, y, delta_x, delta_y) = (3, 10, 257, 511) for an image of dimensions (width, height) = (20, 25)");
    Ok(())
}
