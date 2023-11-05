mod input_text_file_path {
    use super::super::InputTextFilePath;
    use crate::path::InputFilePath;
    use std::borrow::Cow;
    use std::path::Path;

    mod from_unverified_path {
        use super::super::super::{InputTextFilePath, UnverifiedInputTextFilePath};
        use crate::path::{InputFilePath, TryFromWithPathContext, UnverifiedInputFilePath};
        use relative_path::RelativePath;
        use std::borrow::Cow;
        use std::error::Error;

        #[test]
        fn absent_file() {
            let path = UnverifiedInputTextFilePath(UnverifiedInputFilePath(Cow::Borrowed(
                RelativePath::new("none.txt"),
            )));
            test_util::assert_error_contains(
                InputTextFilePath::try_from_with_path_context(
                    path,
                    test_util::path::base_input().0,
                ),
                "does not exist", // Note: do not put a platform-dependent path string here
            );
        }

        #[test]
        fn valid_file() -> Result<(), Box<dyn Error>> {
            let relative_path =
                RelativePath::new("shader/create_displacement_goal/copy_image.wgsl");
            let unverified_path =
                UnverifiedInputTextFilePath(UnverifiedInputFilePath(Cow::Borrowed(relative_path)));
            let base_path = test_util::path::base_input().0;
            assert_eq!(
                InputTextFilePath::try_from_with_path_context(unverified_path.clone(), &base_path)?,
                InputTextFilePath(InputFilePath::try_from_with_path_context(
                    unverified_path.0,
                    base_path,
                )?),
            );
            Ok(())
        }
    }

    #[test]
    fn display() {
        let path_string = "text.txt";
        assert_eq!(
            InputTextFilePath(InputFilePath(Cow::Borrowed(Path::new(path_string)))).to_string(),
            path_string
        );
    }
}

mod string_from_unverified_path {
    use super::super::{InputTextFilePath, UnverifiedInputTextFilePath};
    use crate::path::{TryFromWithPathContext, UnverifiedInputFilePath};
    use relative_path::RelativePath;
    use std::borrow::Cow;
    use std::error::Error;
    use std::fs;

    #[test]
    fn absent_file() {
        let path = UnverifiedInputTextFilePath(UnverifiedInputFilePath(Cow::Borrowed(
            RelativePath::new("none.txt"),
        )));
        test_util::assert_error_contains(
            <String as TryFromWithPathContext<
            UnverifiedInputTextFilePath,
            >>::try_from_with_path_context(
                path,
                test_util::path::base_input().0,
            ),
            "does not exist", // Note: do not put a platform-dependent path string here
        );
    }

    #[test]
    fn valid_file() -> Result<(), Box<dyn Error>> {
        let relative_path = RelativePath::new("shader/create_displacement_goal/copy_image.wgsl");
        let unverified_path =
            UnverifiedInputTextFilePath(UnverifiedInputFilePath(Cow::Borrowed(relative_path)));
        let base_path = test_util::path::base_input().0;
        assert_eq!(
            <String as TryFromWithPathContext<
            UnverifiedInputTextFilePath,
            >>::try_from_with_path_context(unverified_path.clone(), &base_path)?,
            fs::read_to_string(
                InputTextFilePath::try_from_with_path_context(
                    unverified_path,
                    test_util::path::base_input().0
                )?
                .0.0
            )?,
        );
        Ok(())
    }
}
