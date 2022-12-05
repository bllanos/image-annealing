mod image_path {
    use super::super::ImagePath;
    use std::fmt;
    use std::path::Path;

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct TestImagePath(String);

    impl fmt::Display for TestImagePath {
        fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
            unreachable!()
        }
    }

    impl AsRef<Path> for TestImagePath {
        fn as_ref(&self) -> &Path {
            unreachable!()
        }
    }

    impl AsRef<str> for TestImagePath {
        fn as_ref(&self) -> &str {
            unreachable!()
        }
    }

    impl ImagePath for TestImagePath {
        fn from_raw<T: Into<String>>(path: T) -> Self {
            Self(path.into())
        }
    }

    #[test]
    fn from_raw_clone() {
        let path = "1";
        let expected = TestImagePath(String::from(path));
        assert_eq!(TestImagePath::from_raw_clone(path), expected);
    }

    mod from_input_path {
        use super::super::super::ImagePath;
        use super::TestImagePath;
        use image_annealing::ImageDimensions;
        use std::error::Error;

        #[test]
        fn success() -> Result<(), Box<dyn Error>> {
            let path = test_util::make_test_data_path_string(["image", "image", "stripes.png"]);
            let expected = (
                TestImagePath(path.clone()),
                ImageDimensions::from_image_path(&path)?,
            );
            assert_eq!(TestImagePath::from_input_path(path)?, expected);
            Ok(())
        }

        #[test]
        fn not_found() {
            test_util::assert_error_contains(
                TestImagePath::from_input_path(test_util::make_test_data_path_string([
                    "image",
                    "image",
                    "not_found.png",
                ])),
                "does not exist",
            );
        }

        #[test]
        fn non_image() {
            test_util::assert_error_contains(
                TestImagePath::from_input_path(test_util::make_test_data_path_string([
                    "empty.txt",
                ])),
                "The file extension `.\"txt\"` was not recognized as an image format",
            );
        }
    }

    #[test]
    fn from_output_path() {
        let path = test_util::make_test_data_path_string(["image", "image", "stripes.png"]);
        let expected = TestImagePath(path.clone());
        assert_eq!(TestImagePath::from_output_path(path), expected);
    }
}

mod permutation_path {
    use super::super::{ImagePath, PermutationPath};
    use std::path::Path;

    #[test]
    fn display() {
        let path = "1";
        assert_eq!(PermutationPath::from_raw_clone(path).to_string(), path);
    }

    #[test]
    fn as_ref_path() {
        let path = test_util::make_test_data_path(["image", "image", "stripes.png"]);
        assert_eq!(
            <PermutationPath as AsRef<Path>>::as_ref(&PermutationPath::from_raw_clone(
                path.to_str().unwrap()
            )),
            path
        );
    }

    #[test]
    fn as_ref_str() {
        let path = "1";
        assert_eq!(
            <PermutationPath as AsRef<str>>::as_ref(&PermutationPath::from_raw_clone(path)),
            path
        );
    }

    #[test]
    fn from_raw() {
        let path = "1";
        assert_eq!(
            <PermutationPath as AsRef<str>>::as_ref(&PermutationPath::from_raw(String::from(path))),
            path
        );
    }
}

mod displacement_goal_path {
    use super::super::{DisplacementGoalPath, ImagePath};
    use std::path::Path;

    #[test]
    fn display() {
        let path = "1";
        assert_eq!(DisplacementGoalPath::from_raw_clone(path).to_string(), path);
    }

    #[test]
    fn as_ref_path() {
        let path = test_util::make_test_data_path(["image", "image", "stripes.png"]);
        assert_eq!(
            <DisplacementGoalPath as AsRef<Path>>::as_ref(&DisplacementGoalPath::from_raw_clone(
                path.to_str().unwrap()
            )),
            path
        );
    }

    #[test]
    fn as_ref_str() {
        let path = "1";
        assert_eq!(
            <DisplacementGoalPath as AsRef<str>>::as_ref(&DisplacementGoalPath::from_raw_clone(
                path
            )),
            path
        );
    }

    #[test]
    fn from_raw() {
        let path = "1";
        assert_eq!(
            <DisplacementGoalPath as AsRef<str>>::as_ref(&DisplacementGoalPath::from_raw(
                String::from(path)
            )),
            path
        );
    }
}
