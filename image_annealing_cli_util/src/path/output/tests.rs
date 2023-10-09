mod check_parent_path {
    use super::super::check_parent_path;
    use crate::path::FromWithPathContext;
    use relative_path::RelativePath;
    use std::error::Error;
    use std::path::{Path, PathBuf};

    #[test]
    fn absent_parent_directory() {
        let path = PathBuf::from_with_path_context(
            RelativePath::new("not_found/child"),
            test_util::path::base_input().0,
        );
        test_util::assert_error_contains(
            check_parent_path(path),
            "does not exist", // Note: do not put a platform-dependent path string here
        );
    }

    #[test]
    fn not_a_directory() {
        let path = PathBuf::from_with_path_context(
            RelativePath::new("image/image/stripes.png/child"),
            test_util::path::base_input().0,
        );
        test_util::assert_error_contains(check_parent_path(path), "is not a directory");
    }

    #[test]
    fn valid_directory() -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from_with_path_context(
            RelativePath::new("image/image/stripes.png"),
            test_util::path::base_input().0,
        );
        Ok(check_parent_path(path)?)
    }

    #[test]
    fn parent_of_root() -> Result<(), Box<dyn Error>> {
        let mut absolute_path = std::env::current_dir()?;
        while let Some(parent) = absolute_path.parent() {
            check_parent_path(&absolute_path)?;
            absolute_path = parent.to_path_buf();
        }
        Ok(check_parent_path(absolute_path)?)
    }

    #[test]
    fn no_parent() -> Result<(), Box<dyn Error>> {
        let path = Path::new("not_found.png");
        Ok(check_parent_path(path)?)
    }

    #[test]
    fn empty_path() -> Result<(), Box<dyn Error>> {
        let path = Path::new("");
        Ok(check_parent_path(path)?)
    }
}

mod check_output_file_path {
    use super::super::check_output_file_path;
    use crate::path::FromWithPathContext;
    use relative_path::RelativePath;
    use std::error::Error;
    use std::path::PathBuf;

    #[test]
    fn absent_parent() {
        let path = PathBuf::from_with_path_context(
            RelativePath::new("not_found/none.png"),
            test_util::path::base_input().0,
        );
        test_util::assert_error_contains(
            check_output_file_path(path),
            "does not exist", // Note: do not put a platform-dependent path string here
        );
    }

    #[test]
    fn existing_file() -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from_with_path_context(
            RelativePath::new("image/image/stripes.png"),
            test_util::path::base_input().0,
        );
        Ok(check_output_file_path(path)?)
    }

    #[test]
    fn absent_file() -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from_with_path_context(
            RelativePath::new("image/image/not_found.png"),
            test_util::path::base_input().0,
        );
        Ok(check_output_file_path(path)?)
    }
}

mod check_output_directory_path {
    use super::super::check_output_directory_path;
    use crate::path::FromWithPathContext;
    use relative_path::RelativePath;
    use std::error::Error;
    use std::path::PathBuf;

    #[test]
    fn absent_parent() {
        let path = PathBuf::from_with_path_context(
            RelativePath::new("not_found/none"),
            test_util::path::base_input().0,
        );
        test_util::assert_error_contains(
            check_output_directory_path(path),
            "does not exist", // Note: do not put a platform-dependent path string here
        );
    }

    #[test]
    fn not_a_directory() {
        let path = PathBuf::from_with_path_context(
            RelativePath::new("image/image/stripes.png"),
            test_util::path::base_input().0,
        );
        test_util::assert_error_contains(check_output_directory_path(path), "is not a directory");
    }

    #[test]
    fn existing_directory() -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from_with_path_context(
            RelativePath::new("image/image"),
            test_util::path::base_input().0,
        );
        Ok(check_output_directory_path(path)?)
    }

    #[test]
    fn absent_directory() -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from_with_path_context(
            RelativePath::new("image/not_found"),
            test_util::path::base_input().0,
        );
        Ok(check_output_directory_path(path)?)
    }
}

mod output_file_path {
    mod try_from_unverified_output_file_path {
        use super::super::super::{OutputFilePath, UnverifiedOutputFilePath};
        use crate::path::{FromWithPathContext, TryFromWithPathContext};
        use relative_path::RelativePath;
        use std::borrow::Cow;
        use std::error::Error;
        use std::path::PathBuf;

        #[test]
        fn absent_parent() {
            let path =
                UnverifiedOutputFilePath(Cow::Borrowed(RelativePath::new("not_found/none.png")));
            test_util::assert_error_contains(
                OutputFilePath::try_from_with_path_context(path, test_util::path::base_input().0),
                "does not exist", // Note: do not put a platform-dependent path string here
            );
        }

        #[test]
        fn existing_file() -> Result<(), Box<dyn Error>> {
            let relative_path = RelativePath::new("image/image/stripes.png");
            let unverified_path = UnverifiedOutputFilePath(Cow::Borrowed(relative_path));
            let base_path = test_util::path::base_input().0;
            assert_eq!(
                OutputFilePath::try_from_with_path_context(unverified_path.clone(), &base_path)?,
                OutputFilePath(Cow::Owned(PathBuf::from_with_path_context(
                    &unverified_path.0,
                    base_path,
                ))),
            );
            Ok(())
        }

        #[test]
        fn absent_file() -> Result<(), Box<dyn Error>> {
            let relative_path = RelativePath::new("image/image/not_found.png");
            let unverified_path = UnverifiedOutputFilePath(Cow::Borrowed(relative_path));
            let base_path = test_util::path::base_input().0;
            assert_eq!(
                OutputFilePath::try_from_with_path_context(unverified_path.clone(), &base_path)?,
                OutputFilePath(Cow::Owned(PathBuf::from_with_path_context(
                    &unverified_path.0,
                    base_path,
                ))),
            );
            Ok(())
        }
    }
}

mod output_directory_path {
    mod try_from_unverified_output_directory_path {
        use super::super::super::{OutputDirectoryPath, UnverifiedOutputDirectoryPath};
        use crate::path::{FromWithPathContext, TryFromWithPathContext};
        use relative_path::RelativePath;
        use std::borrow::Cow;
        use std::error::Error;
        use std::path::PathBuf;

        #[test]
        fn absent_parent() {
            let path =
                UnverifiedOutputDirectoryPath(Cow::Borrowed(RelativePath::new("not_found/none")));
            test_util::assert_error_contains(
                OutputDirectoryPath::try_from_with_path_context(
                    path,
                    test_util::path::base_input().0,
                ),
                "does not exist", // Note: do not put a platform-dependent path string here
            );
        }

        #[test]
        fn not_a_directory() {
            let path = UnverifiedOutputDirectoryPath(Cow::Borrowed(RelativePath::new(
                "image/image/stripes.png",
            )));
            test_util::assert_error_contains(
                OutputDirectoryPath::try_from_with_path_context(
                    path,
                    test_util::path::base_input().0,
                ),
                "is not a directory",
            );
        }

        #[test]
        fn existing_directory() -> Result<(), Box<dyn Error>> {
            let relative_path = RelativePath::new("image/image");
            let unverified_path = UnverifiedOutputDirectoryPath(Cow::Borrowed(relative_path));
            let base_path = test_util::path::base_input().0;
            assert_eq!(
                OutputDirectoryPath::try_from_with_path_context(
                    unverified_path.clone(),
                    &base_path
                )?,
                OutputDirectoryPath(Cow::Owned(PathBuf::from_with_path_context(
                    &unverified_path.0,
                    base_path,
                ))),
            );
            Ok(())
        }

        #[test]
        fn absent_directory() -> Result<(), Box<dyn Error>> {
            let relative_path = RelativePath::new("image/not_found");
            let unverified_path = UnverifiedOutputDirectoryPath(Cow::Borrowed(relative_path));
            let base_path = test_util::path::base_input().0;
            assert_eq!(
                OutputDirectoryPath::try_from_with_path_context(
                    unverified_path.clone(),
                    &base_path
                )?,
                OutputDirectoryPath(Cow::Owned(PathBuf::from_with_path_context(
                    &unverified_path.0,
                    base_path,
                ))),
            );
            Ok(())
        }
    }
}
