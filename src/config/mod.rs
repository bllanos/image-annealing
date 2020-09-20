use crate::image_utils::rectangle::{self, Rectangle};
use std::error::Error;
use std::path::Path;
use std::path::PathBuf;

pub fn parse_args<T>(args: T) -> Result<(String, Option<Rectangle>), Box<dyn Error>>
where
    T: IntoIterator<Item = String>,
{
    let mut args_iter = args.into_iter();
    args_iter
        .next()
        .expect("No arguments (not even the program name)");
    let filename = args_iter
        .next()
        .ok_or("Expected at least one argument for an image filename.")?;
    if let Some(arg) = args_iter.next() {
        if arg == "--rect" {
            let v: Vec<_> = args_iter
                .take(rectangle::N_CORNERS)
                .map(|x| x.parse::<u32>())
                .filter(|x| x.is_ok())
                .map(Result::unwrap)
                .collect();
            if v.len() == rectangle::N_CORNERS {
                let rect = Rectangle::from_corners(v[0], v[1], v[2], v[3])?;
                if rect.is_empty() {
                    Err("Image rectangle corners specify an empty rectangle")?
                } else {
                    Ok((filename, Some(rect)))
                }
            } else {
                Err("Failed to parse corners for image rectangle from arguments")?
            }
        } else {
            Ok((filename, None))
        }
    } else {
        Ok((filename, None))
    }
}

pub fn check_input_path(filename: &String) -> Result<&Path, Box<dyn Error>> {
    let filepath = Path::new(filename);
    if !filepath.is_file() {
        Err(format!(
            "Image file {} does not exist in the filesystem.",
            filename
        ))?
    } else {
        Ok(filepath)
    }
}

pub fn make_output_filepath(input_filepath: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let output_filename = format!(
        "{}_out{}",
        input_filepath
            .file_stem()
            .ok_or("The input filepath is expected to have a stem.")?
            .to_str()
            .ok_or("The file stem of the input filepath is expected to be valid UTF-8.")?,
        match input_filepath.extension() {
            Some(val) => format!(
                ".{}",
                val.to_str().ok_or(
                    "The file extension of the input filepath is expected to be valid UTF-8."
                )?
            ),
            None => String::new(),
        }
    );
    let mut output_path = PathBuf::new();
    if let Some(parent) = input_filepath.parent() {
        output_path.push(parent);
    }
    output_path.push(output_filename);
    Ok(output_path)
}

#[cfg(test)]
mod tests {

    mod parse_args {
        use super::super::parse_args;
        use crate::image_utils::rectangle::Rectangle;
        use std::error::Error;

        #[test]
        #[should_panic(expected = "No arguments (not even the program name)")]
        fn empty_input() {
            let v: Vec<String> = Vec::new();
            println!("{:?}", parse_args(v));
        }

        #[test]
        fn no_image() {
            let v = vec!["one"];
            let v_strings: Vec<String> = v.into_iter().map(String::from).collect();
            let r = parse_args(v_strings);
            r.expect_err("At least one argument should be required");
        }

        #[test]
        fn single_arg() -> Result<(), Box<dyn Error>> {
            let v = vec!["one", "two"];
            let v_strings: Vec<String> = v.into_iter().map(String::from).collect();
            let r = parse_args(v_strings.clone())?;
            assert_eq!(r.0, v_strings[1]);
            // Option::expect_none is an experimental feature
            assert!(r.1.is_none());
            Ok(())
        }

        #[test]
        fn irrelevant_args() -> Result<(), Box<dyn Error>> {
            let v = vec!["one", "two", "three", "1", "2", "3", "4"];
            let v_strings: Vec<String> = v.into_iter().map(String::from).collect();
            let r = parse_args(v_strings.clone())?;
            assert_eq!(r.0, v_strings[1]);
            assert!(
                r.1.is_none(),
                "A Rectangle should not be returned if `--rect` is not the second argument."
            );
            Ok(())
        }

        #[test]
        fn incomplete_rect() {
            let v = vec!["one", "two", "--rect"];
            let v_strings: Vec<String> = v.into_iter().map(String::from).collect();
            let r = parse_args(v_strings);
            r.expect_err("`--rect` with no following coordinates should result in an error");
        }

        #[test]
        fn bad_rect_parse() {
            let v = vec!["one", "two", "--rect", "a", "b", "1", "2"];
            let v_strings: Vec<String> = v.into_iter().map(String::from).collect();
            let r = parse_args(v_strings);
            r.expect_err("`--rect` with non-numbers afterwards should be an error");
        }

        #[test]
        fn bad_rect_negative() {
            let v = vec!["one", "two", "--rect", "-1", "-1", "1", "2"];
            let v_strings: Vec<String> = v.into_iter().map(String::from).collect();
            let r = parse_args(v_strings);
            r.expect_err("`--rect` with negative numbers afterwards should be an error");
        }

        #[test]
        fn bad_rect_malformed() {
            let v = vec!["one", "two", "--rect", "1", "2", "1", "0"];
            let v_strings: Vec<String> = v.into_iter().map(String::from).collect();
            let r = parse_args(v_strings);
            r.expect_err("`--rect` impossible coordinates afterwards should be an error");
        }

        #[test]
        fn bad_rect_empty() {
            let v = vec!["one", "two", "--rect", "1", "0", "2", "0"];
            let v_strings: Vec<String> = v.into_iter().map(String::from).collect();
            let r = parse_args(v_strings);
            r.expect_err("`--rect` empty rectangle coordinates afterwards should be an error");
        }

        #[test]
        fn good_rect() -> Result<(), Box<dyn Error>> {
            let v = vec!["one", "two", "--rect", "1", "0", "2", "5"];
            let v_strings: Vec<String> = v.into_iter().map(String::from).collect();
            let r = parse_args(v_strings.clone())?;
            assert_eq!(r.0, v_strings[1]);
            assert_eq!(
                r.1.ok_or("Expected a valid rectangle")?,
                Rectangle::from_corners(1, 0, 2, 5)?
            );
            Ok(())
        }
    }
}
