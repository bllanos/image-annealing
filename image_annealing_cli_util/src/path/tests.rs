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

mod into_with_path_context {
    use super::super::{FromWithPathContext, IntoWithPathContext};
    use relative_path::RelativePath;
    use std::path::PathBuf;

    #[test]
    fn path_buf_blanket_implementation() {
        let relative_path = RelativePath::new("image/image/stripes.png");
        let base_path = test_util::path::base_input().0;
        let path: PathBuf = relative_path.into_with_path_context(&base_path);
        assert_eq!(
            path,
            PathBuf::from_with_path_context(relative_path, base_path)
        );
    }
}

mod try_into_with_path_context {
    use super::super::{FromWithPathContext, TryFromWithPathContext, TryIntoWithPathContext};
    use relative_path::{RelativePath, RelativePathBuf};
    use std::error::Error;
    use std::path::PathBuf;

    #[test]
    fn relative_path_buf_blanket_implementation() -> Result<(), Box<dyn Error>> {
        let relative_path = RelativePath::new("image/image/stripes.png");
        let base_path = test_util::path::base_input().0;
        let full_path = PathBuf::from_with_path_context(relative_path, &base_path);
        let path: RelativePathBuf = full_path.clone().try_into_with_path_context(&base_path)?;
        assert_eq!(
            path,
            RelativePathBuf::try_from_with_path_context(full_path, base_path)?
        );
        Ok(())
    }
}

mod path_buf {
    use super::super::FromWithPathContext;
    use relative_path::RelativePath;
    use std::path::PathBuf;

    #[test]
    fn from_with_path_context() {
        let relative_path = RelativePath::new("image/image/stripes.png");
        let base_path = test_util::path::base_input().0;
        let path: PathBuf = PathBuf::from_with_path_context(relative_path, &base_path);
        assert_eq!(path, relative_path.to_path(base_path));
    }
}

mod relative_path_buf {

    mod try_from_with_path_context {
        use super::super::super::{FromWithPathContext, TryFromWithPathContext};
        use relative_path::{RelativePath, RelativePathBuf};
        use std::error::Error;
        use std::path::PathBuf;

        #[test]
        fn invalid_prefix() {
            let relative_path = RelativePath::new("image/image/stripes.png");
            let base_path = test_util::path::base_input().0;
            let full_path = PathBuf::from_with_path_context(relative_path, &base_path);
            test_util::assert_error_contains(
                RelativePathBuf::try_from_with_path_context(
                    full_path.clone(),
                    base_path.join("config"),
                ),
                "prefix not found",
            );
        }

        #[test]
        fn absolute_path() -> Result<(), Box<dyn Error>> {
            let base_path = "";
            let absolute_path = std::env::current_dir()?;
            test_util::assert_error_contains(
                RelativePathBuf::try_from_with_path_context(absolute_path, base_path),
                "path contains non-relative component",
            );
            Ok(())
        }

        #[test]
        fn valid_relative_path() -> Result<(), Box<dyn Error>> {
            let relative_path = RelativePath::new("image/image/stripes.png");
            let base_path = test_util::path::base_input().0;
            let full_path = PathBuf::from_with_path_context(relative_path, &base_path);
            assert_eq!(
                RelativePathBuf::try_from_with_path_context(full_path, base_path)?,
                relative_path,
            );
            Ok(())
        }
    }
}

mod make_base_path {
    use relative_path::RelativePath;
    use std::borrow::Cow;
    use std::error::Error;
    use std::path::{Path, PathBuf};

    #[test]
    fn absolute_path() -> Result<(), Box<dyn Error>> {
        let absolute_path = std::env::current_dir()?;
        assert_eq!(
            super::super::make_base_path(&absolute_path, || -> Result<PathBuf, Box<dyn Error>> {
                unreachable!(
                "make_context_path argument should not be called if the candidate path is absolute"
            )
            })?,
            Cow::<Path>::Borrowed(&absolute_path)
        );
        Ok(())
    }

    #[test]
    fn relative_path() -> Result<(), Box<dyn Error>> {
        let relative_path = RelativePath::new("image/image/stripes.png");
        let base_path = test_util::path::base_input().0;
        let full_path = relative_path.to_path(&base_path);
        assert_eq!(
            super::super::make_base_path(&relative_path.to_path(""), || Ok::<
                Cow<Path>,
                Box<dyn Error>,
            >(base_path))?,
            Cow::<Path>::Owned(full_path)
        );
        Ok(())
    }
}

mod make_base_path_using_current_directory {
    use relative_path::RelativePath;
    use std::borrow::Cow;
    use std::error::Error;
    use std::path::Path;

    #[test]
    fn relative_path() -> Result<(), Box<dyn Error>> {
        let relative_path = RelativePath::new("image/image/stripes.png");
        let base_path = std::env::current_dir()?;
        let full_path = relative_path.to_path(base_path);
        assert_eq!(
            super::super::make_base_path_using_current_directory(&relative_path.to_path(""))?,
            Cow::<Path>::Owned(full_path)
        );
        Ok(())
    }
}
