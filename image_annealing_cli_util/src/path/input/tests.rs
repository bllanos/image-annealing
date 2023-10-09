mod check_input_file_path {
    use super::super::check_input_file_path;
    use crate::path::FromWithPathContext;
    use relative_path::RelativePath;
    use std::error::Error;
    use std::path::PathBuf;

    #[test]
    fn absent_file() {
        let path = PathBuf::from_with_path_context(
            RelativePath::new("none.png"),
            test_util::path::base_input().0,
        );
        test_util::assert_error_contains(
            check_input_file_path(path),
            "does not exist", // Note: do not put a platform-dependent path string here
        );
    }

    #[test]
    fn not_a_file() {
        let path = PathBuf::from_with_path_context(
            RelativePath::new("."),
            test_util::path::base_input().0,
        );
        test_util::assert_error_contains(check_input_file_path(path), "is not a file");
    }

    #[test]
    fn valid_file() -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from_with_path_context(
            RelativePath::new("image/image/stripes.png"),
            test_util::path::base_input().0,
        );
        Ok(check_input_file_path(path)?)
    }
}

mod input_file_path {
    mod try_from_unverified_input_file_path {
        use super::super::super::{InputFilePath, UnverifiedInputFilePath};
        use crate::path::{FromWithPathContext, TryFromWithPathContext};
        use relative_path::RelativePath;
        use std::borrow::Cow;
        use std::error::Error;
        use std::path::PathBuf;

        #[test]
        fn absent_file() {
            let path = UnverifiedInputFilePath(Cow::Borrowed(RelativePath::new("none.png")));
            test_util::assert_error_contains(
                InputFilePath::try_from_with_path_context(path, test_util::path::base_input().0),
                "does not exist", // Note: do not put a platform-dependent path string here
            );
        }

        #[test]
        fn not_a_file() {
            let path = UnverifiedInputFilePath(Cow::Borrowed(RelativePath::new(".")));
            test_util::assert_error_contains(
                InputFilePath::try_from_with_path_context(path, test_util::path::base_input().0),
                "is not a file",
            );
        }

        #[test]
        fn valid_file() -> Result<(), Box<dyn Error>> {
            let relative_path = RelativePath::new("image/image/stripes.png");
            let unverified_path = UnverifiedInputFilePath(Cow::Borrowed(relative_path));
            let base_path = test_util::path::base_input().0;
            assert_eq!(
                InputFilePath::try_from_with_path_context(unverified_path.clone(), &base_path)?,
                InputFilePath(Cow::Owned(PathBuf::from_with_path_context(
                    &unverified_path.0,
                    base_path,
                ))),
            );
            Ok(())
        }
    }
}

mod input_directory_path {
    mod try_from_unverified_input_directory_path {
        use super::super::super::{InputDirectoryPath, UnverifiedInputDirectoryPath};
        use crate::path::{FromWithPathContext, TryFromWithPathContext};
        use relative_path::RelativePath;
        use std::borrow::Cow;
        use std::error::Error;
        use std::path::PathBuf;

        #[test]
        fn absent_directory() {
            let path = UnverifiedInputDirectoryPath(Cow::Borrowed(RelativePath::new("none")));
            test_util::assert_error_contains(
                InputDirectoryPath::try_from_with_path_context(
                    path,
                    test_util::path::base_input().0,
                ),
                "does not exist", // Note: do not put a platform-dependent path string here
            );
        }

        #[test]
        fn not_a_directory() {
            let path = UnverifiedInputDirectoryPath(Cow::Borrowed(RelativePath::new(
                "image/image/stripes.png",
            )));
            test_util::assert_error_contains(
                InputDirectoryPath::try_from_with_path_context(
                    path,
                    test_util::path::base_input().0,
                ),
                "is not a directory",
            );
        }

        #[test]
        fn valid_directory() -> Result<(), Box<dyn Error>> {
            let relative_path = RelativePath::new(".");
            let unverified_path = UnverifiedInputDirectoryPath(Cow::Borrowed(relative_path));
            let base_path = test_util::path::base_input().0;
            assert_eq!(
                InputDirectoryPath::try_from_with_path_context(
                    unverified_path.clone(),
                    &base_path
                )?,
                InputDirectoryPath(Cow::Owned(PathBuf::from_with_path_context(
                    &unverified_path.0,
                    base_path,
                ))),
            );
            Ok(())
        }
    }
}
