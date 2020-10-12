mod parse_args {
    use crate::config::parse_args;
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
}

mod check_input_path {
    use crate::config::check_input_path;
    use std::error::Error;
    use std::path::PathBuf;

    #[test]
    fn absent_file() {
        let mut path = PathBuf::new();
        path.push("data");
        path.push("none.png");
        let filename = String::from(path.to_str().unwrap());
        let r = check_input_path(&filename);
        r.expect_err("A non-existing file should trigger an error");
    }

    #[test]
    fn not_a_file() {
        let mut path = PathBuf::new();
        path.push("data");
        let filename = String::from(path.to_str().unwrap());
        let r = check_input_path(&filename);
        r.expect_err("A directory instead of a file should trigger an error");
    }

    #[test]
    fn valid_file() -> Result<(), Box<dyn Error>> {
        let mut path = PathBuf::new();
        path.push("data");
        path.push("radial_gradient_rg.png");
        let filename = String::from(path.to_str().unwrap());
        let path_out = check_input_path(&filename)?;
        assert_eq!(path, path_out);
        Ok(())
    }
}

mod make_output_filepath {
    use crate::config::make_output_filepath;
    use std::error::Error;
    use std::path::{Path, PathBuf};

    #[test]
    fn empty_input() {
        let path = PathBuf::new();
        let r = make_output_filepath(&path);
        r.expect_err("An empty input path should trigger an error");
    }

    #[test]
    fn empty_extension() -> Result<(), Box<dyn Error>> {
        let path = Path::new("test");
        let r = make_output_filepath(&path)?;
        let path_out = Path::new("test_out");
        assert_eq!(r, path_out);
        Ok(())
    }

    #[test]
    fn with_extension() -> Result<(), Box<dyn Error>> {
        let path = Path::new("test.jpg");
        let r = make_output_filepath(&path)?;
        let path_out = Path::new("test_out.jpg");
        assert_eq!(r, path_out);
        Ok(())
    }

    #[test]
    fn with_path() -> Result<(), Box<dyn Error>> {
        let mut path = PathBuf::new();
        path.push("parent");
        path.push("test.jpg");
        let r = make_output_filepath(&path)?;
        let path_out = Path::new("parent/test_out.jpg");
        assert_eq!(r, path_out);
        Ok(())
    }
}
