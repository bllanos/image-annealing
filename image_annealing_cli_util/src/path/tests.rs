mod check_input_directory_path {
    use super::super::{check_input_directory_path, FromWithPathContext};
    use relative_path::RelativePath;
    use std::error::Error;
    use std::path::PathBuf;

    #[test]
    fn absent_directory() {
        let path = PathBuf::from_with_path_context(
            RelativePath::new("none"),
            test_util::make_test_data_base_path(),
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
            test_util::make_test_data_base_path(),
        );
        test_util::assert_error_contains(check_input_directory_path(path), "is not a directory");
    }

    #[test]
    fn valid_directory() -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from_with_path_context(
            RelativePath::new("."),
            test_util::make_test_data_base_path(),
        );
        Ok(check_input_directory_path(path)?)
    }
}

mod into_with_path_context {
    use super::super::{FromWithPathContext, IntoWithPathContext};
    use relative_path::RelativePath;
    use std::path::PathBuf;

    #[test]
    fn path_buf_blanket_implementation() {
        let relative_path = RelativePath::new("image/image/stripes.png");
        let base_path = test_util::make_test_data_base_path();
        let path: PathBuf = relative_path.clone().into_with_path_context(base_path);
        assert_eq!(
            path,
            PathBuf::from_with_path_context(relative_path, base_path)
        );
    }
}

mod try_into_with_path_context {
    use super::super::{TryFromWithPathContext, TryIntoWithPathContext};
    use relative_path::{RelativePath, RelativePathBuf};
    use std::path::PathBuf;

    #[test]
    fn relative_path_buf_blanket_implementation() {
        let relative_path = RelativePath::new("image/image/stripes.png");
        let base_path = test_util::make_test_data_base_path();
        let full_path = PathBuf::from_with_path_context(relative_path.clone(), base_path);
        let path: RelativePathBuf = full_path.try_into_with_path_context(base_path);
        assert_eq!(
            path,
            RelativePathBuf::try_from_with_path_context(full_path, base_path)
        );
    }
}

mod path_buf {
    use super::super::FromWithPathContext;
    use relative_path::RelativePath;
    use std::path::PathBuf;

    #[test]
    fn from_with_path_context() {
        let relative_path = RelativePath::new("image/image/stripes.png");
        let base_path = test_util::make_test_data_base_path();
        let path: PathBuf = PathBuf::from_with_path_context(relative_path.clone(), base_path);
        assert_eq!(path, relative_path.to_path(base_path));
    }
}

mod relative_path_buf {

    mod try_from_with_path_context {
        use super::super::TryFromWithPathContext;
        use relative_path::{RelativePath, RelativePathBuf};
        use std::path::PathBuf;

        #[test]
        fn invalid_prefix() {
            let relative_path = RelativePath::new("image/image/stripes.png");
            let base_path = test_util::make_test_data_base_path();
            let full_path = PathBuf::from_with_path_context(relative_path.clone(), base_path);
            test_util::assert_error_contains(
                RelativePathBuf::try_from_with_path_context(full_path, base_path.join("config")),
                "TODO unknown error message",
            );
        }

        #[test]
        fn absolute_path() {
            let base_path = "";
            let absolute_path = test_util::make_test_data_base_path();
            test_util::assert_error_contains(
                RelativePathBuf::try_from_with_path_context(absolute_path, base_path),
                "TODO unknown error message",
            );
        }

        #[test]
        fn valid_relative_path() -> Result<(), Box<dyn Error>> {
            let relative_path = RelativePath::new("image/image/stripes.png");
            let base_path = test_util::make_test_data_base_path();
            let full_path = PathBuf::from_with_path_context(relative_path.clone(), base_path);
            assert_eq!(
                RelativePathBuf::try_from_with_path_context(full_path, base_path)?,
                relative_path,
            );
        }
    }
}

mod make_base_path {
    use relative_path::RelativePath;
    use std::borrow::Cow;

    #[test]
    fn absolute_path() {
        let base_path = "";
        let absolute_path = test_util::make_test_data_base_path();
        assert_eq!(
            super::super::make_base_path(base_path, absolute_path),
            Cow::Owned(absolute_path)
        )
    }

    #[test]
    fn relative_path() {
        let relative_path = RelativePath::new("image/image/stripes.png");
        let base_path = test_util::make_test_data_base_path();
        let full_path = relative_path.to_path(base_path);
        assert_eq!(
            super::super::make_base_path(base_path, relative_path.to_path("")),
            Cow::Owned(full_path)
        )
    }
}

mod make_base_path_using_current_directory {
    #[test]
    fn relative_path() -> Result<(), Box<dyn Error>> {
        let relative_path = RelativePath::new("image/image/stripes.png");
        let base_path = std::env::current_dir()?;
        let full_path = relative_path.to_path(base_path);
        assert_eq!(
            super::super::make_base_path_using_current_directory(relative_path.to_path("")),
            Cow::Owned(full_path)
        )
    }
}
