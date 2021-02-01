mod parse_args {
    use super::super::{parse_args, Config};
    use crate::image_utils::ImageDimensions;
    use crate::test_utils;
    use std::error::Error;

    #[test]
    #[should_panic(expected = "No arguments (not even the program name)")]
    fn empty_input() {
        let v: Vec<String> = Vec::new();
        println!("{:?}", parse_args(v));
    }

    #[test]
    fn no_config_file() {
        let v = vec![String::from("one")];
        let r = parse_args(v);
        r.expect_err("At least one argument should be required");
    }

    #[test]
    fn valid_config_file() -> Result<(), Box<dyn Error>> {
        let path =
            test_utils::make_test_data_path_string(&["config", "create_permutation", "valid.json"]);
        let v = vec![String::from("one"), path];
        let r = parse_args(v)?;
        assert_eq!(
            r,
            Config::CreatePermutationConfig {
                image_dimensions: ImageDimensions::new(20, 25)?,
                permutation_output_path_no_extension: String::from("permutation_out"),
            }
        );
        Ok(())
    }

    #[test]
    fn additional_args() -> Result<(), Box<dyn Error>> {
        let path =
            test_utils::make_test_data_path_string(&["config", "create_permutation", "valid.json"]);
        let v = vec![String::from("one"), path, String::from("other_arg")];
        let r = parse_args(v)?;
        assert_eq!(
            r,
            Config::CreatePermutationConfig {
                image_dimensions: ImageDimensions::new(20, 25)?,
                permutation_output_path_no_extension: String::from("permutation_out"),
            }
        );
        Ok(())
    }
}

mod parse_config_file {
    use super::super::{parse_config_file, Config};
    use crate::image_utils::ImageDimensions;
    use crate::test_utils;
    use std::error::Error;

    #[test]
    fn valid_create_permutation_config_file() -> Result<(), Box<dyn Error>> {
        let path =
            test_utils::make_test_data_path_string(&["config", "create_permutation", "valid.json"]);
        let r = parse_config_file(&path)?;
        assert_eq!(
            r,
            Config::CreatePermutationConfig {
                image_dimensions: ImageDimensions::new(20, 25)?,
                permutation_output_path_no_extension: String::from("permutation_out"),
            }
        );
        Ok(())
    }

    #[test]
    fn invalid_create_permutation_config_file() -> Result<(), Box<dyn Error>> {
        let path = test_utils::make_test_data_path_string(&[
            "config",
            "create_permutation",
            "invalid_dimensions.json",
        ]);
        parse_config_file(&path).expect_err(
            "A configuration file with invalid image dimensions should trigger an error",
        );
        Ok(())
    }
}

mod check_input_path {
    use super::super::check_input_path;
    use crate::test_utils;
    use std::error::Error;

    #[test]
    fn absent_file() {
        let path = test_utils::make_test_data_path_string(&["none.png"]);
        let r = check_input_path(&path);
        r.expect_err("A non-existing file should trigger an error");
    }

    #[test]
    fn not_a_file() {
        let path = test_utils::make_test_data_path_string::<Vec<&str>, &str>(Vec::new());
        let r = check_input_path(&path);
        r.expect_err("A directory instead of a file should trigger an error");
    }

    #[test]
    fn valid_file() -> Result<(), Box<dyn Error>> {
        let path = test_utils::make_test_data_path_string(&["image", "radial_gradient_rg.png"]);
        check_input_path(&path)
    }
}
