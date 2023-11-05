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

    #[test]
    fn error_in_make_context_path() {
        test_util::assert_error_contains(
            super::super::make_base_path(Path::new("test.txt"), || {
                Err::<Cow<Path>, &'static str>("error message")
            }),
            "error message",
        );
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

mod make_base_path_using_environment_variable {
    use crate::env::EnvironmentVariableAccessError;
    use relative_path::RelativePath;
    use std::borrow::Cow;
    use std::env;
    use std::error::Error;
    use std::path::Path;

    #[test]
    fn exists() -> Result<(), Box<dyn Error>> {
        let key = format!("{}.{}.{}", module_path!(), line!(), column!());
        let value = "environment_variable_value";
        env::set_var(&key, value);
        let relative_path = RelativePath::new("image/image/stripes.png");
        let full_path = relative_path.to_path(value);
        assert_eq!(
            super::super::make_base_path_using_environment_variable(
                &relative_path.to_path(""),
                &key
            )?,
            Cow::<Path>::Owned(full_path)
        );
        env::remove_var(&key);
        Ok(())
    }

    #[test]
    fn not_found() {
        let key = format!("{}.{}.{}", module_path!(), line!(), column!());
        assert_eq!(
            super::super::make_base_path_using_environment_variable(Path::new("test.txt"), &key),
            Err(EnvironmentVariableAccessError::new(&key)),
        );
    }
}
