mod check_input_file_path {
    use super::super::check_input_file_path;
    use std::error::Error;
    use std::path::Path;

    #[test]
    fn absent_file() {
        let path = test_util::make_test_data_path(["none.png"]);
        test_util::assert_error_contains(
            check_input_file_path(path),
            "does not exist", // Note: do not put a platform-dependent path string here
        );
    }

    #[test]
    fn not_a_file() {
        let path = test_util::make_test_data_path::<Vec<&Path>, &Path>(Vec::new());
        test_util::assert_error_contains(check_input_file_path(path), "is not a file");
    }

    #[test]
    fn valid_file() -> Result<(), Box<dyn Error>> {
        let path = test_util::make_test_data_path(["image", "image", "stripes.png"]);
        Ok(check_input_file_path(path)?)
    }
}

mod check_directory_path {
    use super::super::check_directory_path;
    use std::error::Error;
    use std::path::Path;

    #[test]
    fn absent_directory() {
        let path = test_util::make_test_data_path(["none"]);
        test_util::assert_error_contains(
            check_directory_path(path),
            "does not exist", // Note: do not put a platform-dependent path string here
        );
    }

    #[test]
    fn not_a_directory() {
        let path = test_util::make_test_data_path(["image", "image", "stripes.png"]);
        test_util::assert_error_contains(check_directory_path(path), "is not a directory");
    }

    #[test]
    fn valid_directory() -> Result<(), Box<dyn Error>> {
        let path = test_util::make_test_data_path::<Vec<&Path>, &Path>(Vec::new());
        Ok(check_directory_path(path)?)
    }
}

mod convert_path_separators {
    use super::super::convert_path_separators;
    use std::path::MAIN_SEPARATOR;

    #[test]
    fn windows_path() {
        let filepath = String::from("one\\two\\three\\..\\.\\end.txt");
        let converted = convert_path_separators(&filepath);
        if MAIN_SEPARATOR == '\\' {
            assert_eq!(converted, filepath);
        } else {
            assert!(converted.find('\\').is_none());
            assert!(converted.find(MAIN_SEPARATOR).is_some());
        }
    }

    #[test]
    fn unix_path() {
        let filepath = String::from("one/two/three/.././end.txt");
        let converted = convert_path_separators(&filepath);
        if MAIN_SEPARATOR == '/' {
            assert_eq!(converted, filepath);
        } else {
            assert!(converted.find('/').is_none());
            assert!(converted.find(MAIN_SEPARATOR).is_some());
        }
    }

    #[test]
    fn no_separators() {
        let filepath = String::from("end.txt");
        let converted = convert_path_separators(&filepath);
        assert_eq!(converted, filepath);
    }
}

mod convert_and_check_input_file_path {
    use super::super::convert_and_check_input_file_path;
    use std::error::Error;
    use std::path::Path;
    use std::path::MAIN_SEPARATOR;

    #[test]
    fn absent_file() {
        let path = test_util::make_test_data_path_string(["none.png"]);
        test_util::assert_error_contains(
            convert_and_check_input_file_path(path),
            "does not exist", // Note: do not put a platform-dependent path string here
        );
    }

    #[test]
    fn not_a_file() {
        let path = test_util::make_test_data_path_string::<Vec<&Path>, &Path>(Vec::new());
        test_util::assert_error_contains(convert_and_check_input_file_path(path), "is not a file");
    }

    #[test]
    fn valid_file() -> Result<(), Box<dyn Error>> {
        let path = test_util::make_test_data_path_string(["image", "image", "stripes.png"]);
        assert_eq!(convert_and_check_input_file_path(&path)?, path);
        Ok(())
    }

    #[test]
    fn valid_file_windows_path() -> Result<(), Box<dyn Error>> {
        let path = "..\\test_data\\image\\image\\stripes.png";
        let converted = convert_and_check_input_file_path(path)?;
        if MAIN_SEPARATOR == '\\' {
            assert_eq!(converted, path);
        } else {
            assert!(converted.find('\\').is_none());
            assert!(converted.find(MAIN_SEPARATOR).is_some());
        }
        Ok(())
    }

    #[test]
    fn valid_file_unix_path() -> Result<(), Box<dyn Error>> {
        let path = "../test_data/image/image/stripes.png";
        let converted = convert_and_check_input_file_path(path)?;
        if MAIN_SEPARATOR == '/' {
            assert_eq!(converted, path);
        } else {
            assert!(converted.find('/').is_none());
            assert!(converted.find(MAIN_SEPARATOR).is_some());
        }
        Ok(())
    }
}
