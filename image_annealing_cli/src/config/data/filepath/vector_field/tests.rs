use image_annealing_cli_util::path::{UnverifiedInputFilePath, UnverifiedOutputFilePath};

fn non_image_path() -> UnverifiedInputFilePath<'static> {
    test_util::path::relative_input_file("empty.txt")
}

fn non_image_error_message() -> &'static str {
    "The file extension `.\"txt\"` was not recognized as an image format"
}

fn missing_directory_path() -> UnverifiedOutputFilePath<'static> {
    test_util::path::relative_output_file("not_found/cannot_create")
}

fn missing_error_message() -> &'static str {
    "does not exist"
}

mod input_permutation_path {
    use super::super::InputPermutationPath;
    use image_annealing_cli_util::path::InputFilePath;
    use std::borrow::Cow;
    use std::path::Path;

    mod from_unverified_path {
        use super::super::super::{InputPermutationPath, UnverifiedInputPermutationPath};
        use image_annealing::ImageDimensions;
        use image_annealing_cli_util::path::{
            InputFilePath, TryFromWithPathContext, TryIntoWithPathContext,
        };
        use std::error::Error;

        #[test]
        fn success() -> Result<(), Box<dyn Error>> {
            let unverified_path =
                test_util::path::relative_input_file("image/permutation/identity_permutation.png");
            assert_eq!(
                InputPermutationPath::try_from_unverified_with_path_context(
                    UnverifiedInputPermutationPath(unverified_path.clone()),
                    test_util::path::base_input().0,
                )?,
                (
                    InputPermutationPath(
                        unverified_path
                            .clone()
                            .try_into_with_path_context(test_util::path::base_input().0)?
                    ),
                    ImageDimensions::from_image_path(
                        InputFilePath::try_from_with_path_context(
                            unverified_path,
                            test_util::path::base_input().0,
                        )
                        .unwrap()
                        .0,
                    )
                    .unwrap()
                )
            );
            Ok(())
        }

        #[test]
        fn not_found() {
            test_util::assert_error_contains(
                InputPermutationPath::try_from_unverified_with_path_context(
                    UnverifiedInputPermutationPath(test_util::path::relative_input_file(
                        "image/permutation/not_found.png",
                    )),
                    test_util::path::base_input().0,
                ),
                super::super::missing_error_message(),
            );
        }

        #[test]
        fn non_image() {
            test_util::assert_error_contains(
                InputPermutationPath::try_from_unverified_with_path_context(
                    UnverifiedInputPermutationPath(super::super::non_image_path()),
                    test_util::path::base_input().0,
                ),
                super::super::non_image_error_message(),
            );
        }
    }

    #[test]
    fn display() {
        let path_string = "permutation.png";
        assert_eq!(
            InputPermutationPath(InputFilePath(Cow::Borrowed(Path::new(path_string)))).to_string(),
            path_string
        );
    }
}

mod output_permutation_path {
    use super::super::OutputPermutationPath;
    use image_annealing_cli_util::path::OutputFilePath;
    use std::borrow::Cow;
    use std::path::Path;

    mod from_unverified_path {
        use super::super::super::{OutputPermutationPath, UnverifiedOutputPermutationPath};
        use image_annealing_cli_util::path::{TryFromWithPathContext, TryIntoWithPathContext};
        use std::error::Error;

        #[test]
        fn success() -> Result<(), Box<dyn Error>> {
            let unverified_path = test_util::unique_relative_output_file!();
            assert_eq!(
                OutputPermutationPath::try_from_with_path_context(
                    UnverifiedOutputPermutationPath(unverified_path.clone()),
                    test_util::path::base_output().0,
                )?,
                OutputPermutationPath(
                    unverified_path.try_into_with_path_context(test_util::path::base_output().0)?
                )
            );
            Ok(())
        }

        #[test]
        fn missing_directory() {
            test_util::assert_error_contains(
                OutputPermutationPath::try_from_with_path_context(
                    UnverifiedOutputPermutationPath(super::super::missing_directory_path()),
                    test_util::path::base_output().0,
                ),
                super::super::missing_error_message(),
            );
        }
    }

    #[test]
    fn display() {
        let path_string = "permutation";
        assert_eq!(
            OutputPermutationPath(OutputFilePath(Cow::Borrowed(Path::new(path_string))))
                .to_string(),
            path_string
        );
    }
}

mod input_displacement_goal_path {
    use super::super::InputDisplacementGoalPath;
    use image_annealing_cli_util::path::InputFilePath;
    use std::borrow::Cow;
    use std::path::Path;

    mod from_unverified_path {
        use super::super::super::{InputDisplacementGoalPath, UnverifiedInputDisplacementGoalPath};
        use image_annealing::ImageDimensions;
        use image_annealing_cli_util::path::{
            InputFilePath, TryFromWithPathContext, TryIntoWithPathContext,
        };
        use std::error::Error;

        #[test]
        fn success() -> Result<(), Box<dyn Error>> {
            let unverified_path = test_util::path::relative_input_file(
                "image/displacement_goal/identity_displacement_goal.png",
            );
            assert_eq!(
                InputDisplacementGoalPath::try_from_unverified_with_path_context(
                    UnverifiedInputDisplacementGoalPath(unverified_path.clone()),
                    test_util::path::base_input().0,
                )?,
                (
                    InputDisplacementGoalPath(
                        unverified_path
                            .clone()
                            .try_into_with_path_context(test_util::path::base_input().0)?
                    ),
                    ImageDimensions::from_image_path(
                        InputFilePath::try_from_with_path_context(
                            unverified_path,
                            test_util::path::base_input().0,
                        )
                        .unwrap()
                        .0,
                    )
                    .unwrap()
                )
            );
            Ok(())
        }

        #[test]
        fn not_found() {
            test_util::assert_error_contains(
                InputDisplacementGoalPath::try_from_unverified_with_path_context(
                    UnverifiedInputDisplacementGoalPath(test_util::path::relative_input_file(
                        "image/displacement_goal/not_found.png",
                    )),
                    test_util::path::base_input().0,
                ),
                super::super::missing_error_message(),
            );
        }

        #[test]
        fn non_image() {
            test_util::assert_error_contains(
                InputDisplacementGoalPath::try_from_unverified_with_path_context(
                    UnverifiedInputDisplacementGoalPath(super::super::non_image_path()),
                    test_util::path::base_input().0,
                ),
                super::super::non_image_error_message(),
            );
        }
    }

    #[test]
    fn display() {
        let path_string = "displacement_goal.png";
        assert_eq!(
            InputDisplacementGoalPath(InputFilePath(Cow::Borrowed(Path::new(path_string))))
                .to_string(),
            path_string
        );
    }
}

mod output_displacement_goal_path {
    use super::super::OutputDisplacementGoalPath;
    use image_annealing_cli_util::path::OutputFilePath;
    use std::borrow::Cow;
    use std::path::Path;

    mod from_unverified_path {
        use super::super::super::{
            OutputDisplacementGoalPath, UnverifiedOutputDisplacementGoalPath,
        };
        use image_annealing_cli_util::path::{TryFromWithPathContext, TryIntoWithPathContext};
        use std::error::Error;

        #[test]
        fn success() -> Result<(), Box<dyn Error>> {
            let unverified_path = test_util::unique_relative_output_file!();
            assert_eq!(
                OutputDisplacementGoalPath::try_from_with_path_context(
                    UnverifiedOutputDisplacementGoalPath(unverified_path.clone()),
                    test_util::path::base_output().0,
                )?,
                OutputDisplacementGoalPath(
                    unverified_path.try_into_with_path_context(test_util::path::base_output().0)?
                )
            );
            Ok(())
        }

        #[test]
        fn missing_directory() {
            test_util::assert_error_contains(
                OutputDisplacementGoalPath::try_from_with_path_context(
                    UnverifiedOutputDisplacementGoalPath(super::super::missing_directory_path()),
                    test_util::path::base_output().0,
                ),
                super::super::missing_error_message(),
            );
        }
    }

    #[test]
    fn display() {
        let path_string = "displacement_goal";
        assert_eq!(
            OutputDisplacementGoalPath(OutputFilePath(Cow::Borrowed(Path::new(path_string))))
                .to_string(),
            path_string
        );
    }
}
