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
