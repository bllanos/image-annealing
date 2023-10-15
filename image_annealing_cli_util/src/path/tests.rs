mod check_input_directory_path {
    use super::super::{check_input_directory_path, FromWithPathContext};
    use relative_path::RelativePath;
    use std::error::Error;
    use std::path::PathBuf;

    #[test]
    fn absent_directory() {
        let path = PathBuf::from_with_path_context(
            RelativePath::new("none"),
            test_util::path::base_input().0,
        );
        test_util::assert_error_contains(
            check_input_directory_path(path),
            "does not exist", // Note: do not put a platform-dependent path string here
        );
    }

    #[test]
    fn not_a_directory() {
        let path = PathBuf::from_with_path_context(
            RelativePath::new("image/image/stripes.png"),
            test_util::path::base_input().0,
        );
        test_util::assert_error_contains(check_input_directory_path(path), "is not a directory");
    }

    #[test]
    fn valid_directory() -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from_with_path_context(
            RelativePath::new("."),
            test_util::path::base_input().0,
        );
        Ok(check_input_directory_path(path)?)
    }
}
