use image_annealing::compute::format::ImageFileWriter;
use image_annealing::ImageDimensions;
use std::error::Error;
use std::path::PathBuf;

mod synthesis;

fn parse_args<T>(args: T) -> Result<ImageDimensions, Box<dyn Error>>
where
    T: IntoIterator<Item = String>,
{
    let mut args_iter = args.into_iter();
    args_iter
        .next()
        .expect("no arguments (not even the program name)");
    let image_width: usize = args_iter
        .next()
        .ok_or("expected a square image side length as a command-line argument")?
        .parse()?;
    Ok(ImageDimensions::new(image_width, image_width)?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let dimensions = parse_args(std::env::args())?;
    println!("Generating data using image dimensions {}", dimensions);

    let mut image_path_no_extension = PathBuf::new();
    image_path_no_extension
        .extend(["examples_output", "image_annealing_cli_bin_dot", "image"].into_iter());
    let image_path =
        synthesis::white_dot(&dimensions).save_add_extension(image_path_no_extension)?;
    println!("Saved input image to {}", image_path.display());

    let mut displacement_goal_path_no_extension = PathBuf::new();
    displacement_goal_path_no_extension.extend(
        [
            "examples_output",
            "image_annealing_cli_bin_dot",
            "displacement_goal",
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

#[cfg(test)]
mod tests {
    mod parse_args {
        use super::super::parse_args;
        use image_annealing::ImageDimensions;
        use std::error::Error;

        #[test]
        #[should_panic(expected = "no arguments (not even the program name)")]
        fn empty_input() {
            let v: Vec<String> = Vec::new();
            let _ = parse_args(v);
        }

        #[test]
        fn no_argument() {
            let v = vec![String::from("12")];
            test_utils::assert_error_contains(
                parse_args(v),
                "expected a square image side length as a command-line argument",
            );
        }

        #[test]
        fn negative() {
            let v = vec![String::from("12"), String::from("-1")];
            test_utils::assert_error_contains(parse_args(v), "invalid digit found in string");
        }

        #[test]
        fn fraction() {
            let v = vec![String::from("12"), String::from("1.5")];
            test_utils::assert_error_contains(parse_args(v), "invalid digit found in string");
        }

        #[test]
        fn zero() {
            let v = vec![String::from("12"), String::from("0")];
            test_utils::assert_error_contains(parse_args(v), "width is zero");
        }

        #[test]
        fn success() -> Result<(), Box<dyn Error>> {
            let v = vec![String::from("12"), String::from("325")];
            assert_eq!(parse_args(v)?, ImageDimensions::new(325, 325)?,);
            Ok(())
        }
    }
}
