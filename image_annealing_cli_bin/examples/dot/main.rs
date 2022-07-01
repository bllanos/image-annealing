use image_annealing::compute::format::ImageFileWriter;
use image_annealing::ImageDimensions;
use std::error::Error;
use std::path::PathBuf;

mod synthesis;

const IMAGE_WIDTH: usize = 32;

fn main() -> Result<(), Box<dyn Error>> {
    let dimensions = ImageDimensions::new(IMAGE_WIDTH, IMAGE_WIDTH)?;
    println!("Generating data using image dimensions {}", dimensions);

    let mut image_path_no_extension = PathBuf::new();
    image_path_no_extension
        .extend(["examples_output", "image_annealing_cli_bin_dot_image"].into_iter());
    let image_path =
        synthesis::white_dot(&dimensions).save_add_extension(image_path_no_extension)?;
    println!("Saved input image to {}", image_path.display());

    let mut displacement_goal_path_no_extension = PathBuf::new();
    displacement_goal_path_no_extension.extend(
        [
            "examples_output",
            "image_annealing_cli_bin_dot_displacement_goal",
        ]
        .into_iter(),
    );
    let displacement_goal_path =
        synthesis::dot_goal(&dimensions).save_add_extension(displacement_goal_path_no_extension)?;
    println!(
        "Saved input displacement goal to {}",
        displacement_goal_path.display()
    );
    Ok(())
}
