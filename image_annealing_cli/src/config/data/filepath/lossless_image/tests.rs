use super::UnverifiedInputLosslessImagePath;
use image_annealing_cli_util::path::UnverifiedInputFilePath;

fn existing_rgba8_path1() -> UnverifiedInputFilePath<'static> {
    test_util::path::relative_input_file("image/image/stripes.png")
}

fn existing_rgba8_path2() -> UnverifiedInputFilePath<'static> {
    test_util::path::relative_input_file("image/image/green.png")
}

fn existing_rgba8_path3() -> UnverifiedInputFilePath<'static> {
    test_util::path::relative_input_file("image/image/pastel.png")
}

fn existing_rgba8_path4() -> UnverifiedInputFilePath<'static> {
    test_util::path::relative_input_file("image/image/parque.png")
}

fn existing_rgba16_path1() -> UnverifiedInputFilePath<'static> {
    test_util::path::relative_input_file("image/image/red.png")
}

fn existing_rgba16_path2() -> UnverifiedInputFilePath<'static> {
    test_util::path::relative_input_file("image/image/leopard.png")
}

fn valid_rgba8() -> UnverifiedInputLosslessImagePath<'static> {
    UnverifiedInputLosslessImagePath::Rgba8(existing_rgba8_path1())
}

fn valid_rgba8x2() -> UnverifiedInputLosslessImagePath<'static> {
    UnverifiedInputLosslessImagePath::Rgba8x2(existing_rgba8_path1(), existing_rgba8_path2())
}

fn valid_rgba8x3() -> UnverifiedInputLosslessImagePath<'static> {
    UnverifiedInputLosslessImagePath::Rgba8x3(
        existing_rgba8_path1(),
        existing_rgba8_path2(),
        existing_rgba8_path3(),
    )
}

fn valid_rgba8x4() -> UnverifiedInputLosslessImagePath<'static> {
    UnverifiedInputLosslessImagePath::Rgba8x4(
        existing_rgba8_path1(),
        existing_rgba8_path2(),
        existing_rgba8_path3(),
        existing_rgba8_path4(),
    )
}

fn valid_rgba16() -> UnverifiedInputLosslessImagePath<'static> {
    UnverifiedInputLosslessImagePath::Rgba16(existing_rgba16_path1())
}

fn valid_rgba16x2() -> UnverifiedInputLosslessImagePath<'static> {
    UnverifiedInputLosslessImagePath::Rgba16x2(existing_rgba16_path1(), existing_rgba16_path2())
}

fn valid_rgba16_rgba8() -> UnverifiedInputLosslessImagePath<'static> {
    UnverifiedInputLosslessImagePath::Rgba16Rgba8(existing_rgba16_path1(), existing_rgba8_path1())
}

fn valid_rgba16_rgba8x2() -> UnverifiedInputLosslessImagePath<'static> {
    UnverifiedInputLosslessImagePath::Rgba16Rgba8x2(
        existing_rgba16_path1(),
        existing_rgba8_path1(),
        existing_rgba8_path2(),
    )
}

mod unverified_input_lossless_image_path {
    mod format {
        use super::super::super::ImageFormat;

        #[test]
        fn rgba8() {
            assert_eq!(super::super::valid_rgba8().format(), ImageFormat::Rgba8);
        }

        #[test]
        fn rgba8x2() {
            assert_eq!(super::super::valid_rgba8x2().format(), ImageFormat::Rgba8x2);
        }

        #[test]
        fn rgba8x3() {
            assert_eq!(super::super::valid_rgba8x3().format(), ImageFormat::Rgba8x3);
        }

        #[test]
        fn rgba8x4() {
            assert_eq!(super::super::valid_rgba8x4().format(), ImageFormat::Rgba8x4);
        }

        #[test]
        fn rgba16() {
            assert_eq!(super::super::valid_rgba16().format(), ImageFormat::Rgba16);
        }

        #[test]
        fn rgba16x2() {
            assert_eq!(
                super::super::valid_rgba16x2().format(),
                ImageFormat::Rgba16x2
            );
        }

        #[test]
        fn rgba16_rgba8() {
            assert_eq!(
                super::super::valid_rgba16_rgba8().format(),
                ImageFormat::Rgba16Rgba8
            );
        }

        #[test]
        fn rgba16_rgba8x2() {
            assert_eq!(
                super::super::valid_rgba16_rgba8x2().format(),
                ImageFormat::Rgba16Rgba8x2
            );
        }
    }
}

mod unverified_output_lossless_image_path {
    mod format {
        use super::super::super::ImageFormat;
        use super::super::super::UnverifiedOutputLosslessImagePath;

        #[test]
        fn rgba8() {
            assert_eq!(
                UnverifiedOutputLosslessImagePath::Rgba8(test_util::unique_relative_output_file!())
                    .format(),
                ImageFormat::Rgba8
            );
        }

        #[test]
        fn rgba8x2() {
            assert_eq!(
                UnverifiedOutputLosslessImagePath::Rgba8x2(
                    test_util::unique_relative_output_file!(),
                    test_util::unique_relative_output_file!()
                )
                .format(),
                ImageFormat::Rgba8x2
            );
        }

        #[test]
        fn rgba8x3() {
            assert_eq!(
                UnverifiedOutputLosslessImagePath::Rgba8x3(
                    test_util::unique_relative_output_file!(),
                    test_util::unique_relative_output_file!(),
                    test_util::unique_relative_output_file!()
                )
                .format(),
                ImageFormat::Rgba8x3
            );
        }

        #[test]
        fn rgba8x4() {
            assert_eq!(
                UnverifiedOutputLosslessImagePath::Rgba8x4(
                    test_util::unique_relative_output_file!(),
                    test_util::unique_relative_output_file!(),
                    test_util::unique_relative_output_file!(),
                    test_util::unique_relative_output_file!()
                )
                .format(),
                ImageFormat::Rgba8x4
            );
        }

        #[test]
        fn rgba16() {
            assert_eq!(
                UnverifiedOutputLosslessImagePath::Rgba16(
                    test_util::unique_relative_output_file!()
                )
                .format(),
                ImageFormat::Rgba16
            );
        }

        #[test]
        fn rgba16x2() {
            assert_eq!(
                UnverifiedOutputLosslessImagePath::Rgba16x2(
                    test_util::unique_relative_output_file!(),
                    test_util::unique_relative_output_file!()
                )
                .format(),
                ImageFormat::Rgba16x2
            );
        }

        #[test]
        fn rgba16_rgba8() {
            assert_eq!(
                UnverifiedOutputLosslessImagePath::Rgba16Rgba8(
                    test_util::unique_relative_output_file!(),
                    test_util::unique_relative_output_file!()
                )
                .format(),
                ImageFormat::Rgba16Rgba8
            );
        }

        #[test]
        fn rgba16_rgba8x2() {
            assert_eq!(
                UnverifiedOutputLosslessImagePath::Rgba16Rgba8x2(
                    test_util::unique_relative_output_file!(),
                    test_util::unique_relative_output_file!(),
                    test_util::unique_relative_output_file!()
                )
                .format(),
                ImageFormat::Rgba16Rgba8x2
            );
        }
    }
}

mod input_lossless_image_path {
    use super::super::InputLosslessImagePath;

    fn valid_rgba8() -> InputLosslessImagePath<'static> {
        InputLosslessImagePath::try_from_unverified_with_path_context(
            super::valid_rgba8(),
            test_util::path::base_input().0,
        )
        .unwrap()
        .0
    }

    fn valid_rgba8x2() -> InputLosslessImagePath<'static> {
        InputLosslessImagePath::try_from_unverified_with_path_context(
            super::valid_rgba8x2(),
            test_util::path::base_input().0,
        )
        .unwrap()
        .0
    }

    fn valid_rgba8x3() -> InputLosslessImagePath<'static> {
        InputLosslessImagePath::try_from_unverified_with_path_context(
            super::valid_rgba8x3(),
            test_util::path::base_input().0,
        )
        .unwrap()
        .0
    }

    fn valid_rgba8x4() -> InputLosslessImagePath<'static> {
        InputLosslessImagePath::try_from_unverified_with_path_context(
            super::valid_rgba8x4(),
            test_util::path::base_input().0,
        )
        .unwrap()
        .0
    }

    fn valid_rgba16() -> InputLosslessImagePath<'static> {
        InputLosslessImagePath::try_from_unverified_with_path_context(
            super::valid_rgba16(),
            test_util::path::base_input().0,
        )
        .unwrap()
        .0
    }

    fn valid_rgba16x2() -> InputLosslessImagePath<'static> {
        InputLosslessImagePath::try_from_unverified_with_path_context(
            super::valid_rgba16x2(),
            test_util::path::base_input().0,
        )
        .unwrap()
        .0
    }

    fn valid_rgba16_rgba8() -> InputLosslessImagePath<'static> {
        InputLosslessImagePath::try_from_unverified_with_path_context(
            super::valid_rgba16_rgba8(),
            test_util::path::base_input().0,
        )
        .unwrap()
        .0
    }

    fn valid_rgba16_rgba8x2() -> InputLosslessImagePath<'static> {
        InputLosslessImagePath::try_from_unverified_with_path_context(
            super::valid_rgba16_rgba8x2(),
            test_util::path::base_input().0,
        )
        .unwrap()
        .0
    }

    mod format {
        use super::super::super::ImageFormat;

        #[test]
        fn rgba8() {
            assert_eq!(super::valid_rgba8().format(), ImageFormat::Rgba8);
        }

        #[test]
        fn rgba8x2() {
            assert_eq!(super::valid_rgba8x2().format(), ImageFormat::Rgba8x2);
        }

        #[test]
        fn rgba8x3() {
            assert_eq!(super::valid_rgba8x3().format(), ImageFormat::Rgba8x3);
        }

        #[test]
        fn rgba8x4() {
            assert_eq!(super::valid_rgba8x4().format(), ImageFormat::Rgba8x4);
        }

        #[test]
        fn rgba16() {
            assert_eq!(super::valid_rgba16().format(), ImageFormat::Rgba16);
        }

        #[test]
        fn rgba16x2() {
            assert_eq!(super::valid_rgba16x2().format(), ImageFormat::Rgba16x2);
        }

        #[test]
        fn rgba16_rgba8() {
            assert_eq!(
                super::valid_rgba16_rgba8().format(),
                ImageFormat::Rgba16Rgba8
            );
        }

        #[test]
        fn rgba16_rgba8x2() {
            assert_eq!(
                super::valid_rgba16_rgba8x2().format(),
                ImageFormat::Rgba16Rgba8x2
            );
        }
    }

    mod as_vec {
        use image_annealing_cli_util::path::{
            InputFilePath, TryFromWithPathContext, UnverifiedInputFilePath,
        };
        use std::path::{Path, PathBuf};

        fn make_absolute_paths(paths: Vec<UnverifiedInputFilePath<'static>>) -> Vec<PathBuf> {
            paths
                .into_iter()
                .map(|relative_path| {
                    InputFilePath::try_from_with_path_context(
                        relative_path,
                        test_util::path::base_input().0,
                    )
                    .unwrap()
                    .0
                    .into_owned()
                })
                .collect::<Vec<PathBuf>>()
        }

        #[test]
        fn rgba8() {
            let paths = make_absolute_paths(vec![super::super::existing_rgba8_path1()]);
            assert_eq!(
                super::valid_rgba8().as_vec(),
                paths
                    .iter()
                    .map(<PathBuf as AsRef<Path>>::as_ref)
                    .collect::<Vec<&Path>>()
            );
        }

        #[test]
        fn rgba8x2() {
            let paths = make_absolute_paths(vec![
                super::super::existing_rgba8_path1(),
                super::super::existing_rgba8_path2(),
            ]);
            assert_eq!(
                super::valid_rgba8x2().as_vec(),
                paths
                    .iter()
                    .map(<PathBuf as AsRef<Path>>::as_ref)
                    .collect::<Vec<&Path>>()
            );
        }

        #[test]
        fn rgba8x3() {
            let paths = make_absolute_paths(vec![
                super::super::existing_rgba8_path1(),
                super::super::existing_rgba8_path2(),
                super::super::existing_rgba8_path3(),
            ]);
            assert_eq!(
                super::valid_rgba8x3().as_vec(),
                paths
                    .iter()
                    .map(<PathBuf as AsRef<Path>>::as_ref)
                    .collect::<Vec<&Path>>()
            );
        }

        #[test]
        fn rgba8x4() {
            let paths = make_absolute_paths(vec![
                super::super::existing_rgba8_path1(),
                super::super::existing_rgba8_path2(),
                super::super::existing_rgba8_path3(),
                super::super::existing_rgba8_path4(),
            ]);
            assert_eq!(
                super::valid_rgba8x4().as_vec(),
                paths
                    .iter()
                    .map(<PathBuf as AsRef<Path>>::as_ref)
                    .collect::<Vec<&Path>>()
            );
        }

        #[test]
        fn rgba16() {
            let paths = make_absolute_paths(vec![super::super::existing_rgba16_path1()]);
            assert_eq!(
                super::valid_rgba16().as_vec(),
                paths
                    .iter()
                    .map(<PathBuf as AsRef<Path>>::as_ref)
                    .collect::<Vec<&Path>>()
            );
        }

        #[test]
        fn rgba16x2() {
            let paths = make_absolute_paths(vec![
                super::super::existing_rgba16_path1(),
                super::super::existing_rgba16_path2(),
            ]);
            assert_eq!(
                super::valid_rgba16x2().as_vec(),
                paths
                    .iter()
                    .map(<PathBuf as AsRef<Path>>::as_ref)
                    .collect::<Vec<&Path>>()
            );
        }

        #[test]
        fn rgba16_rgba8() {
            let paths = make_absolute_paths(vec![
                super::super::existing_rgba16_path1(),
                super::super::existing_rgba8_path1(),
            ]);
            assert_eq!(
                super::valid_rgba16_rgba8().as_vec(),
                paths
                    .iter()
                    .map(<PathBuf as AsRef<Path>>::as_ref)
                    .collect::<Vec<&Path>>()
            );
        }

        #[test]
        fn rgba16_rgba8x2() {
            let paths = make_absolute_paths(vec![
                super::super::existing_rgba16_path1(),
                super::super::existing_rgba8_path1(),
                super::super::existing_rgba8_path2(),
            ]);
            assert_eq!(
                super::valid_rgba16_rgba8x2().as_vec(),
                paths
                    .iter()
                    .map(<PathBuf as AsRef<Path>>::as_ref)
                    .collect::<Vec<&Path>>()
            );
        }
    }

    mod from_unverified_path {
        use image_annealing::ImageDimensions;
        use image_annealing_cli_util::path::{
            InputFilePath, TryFromWithPathContext, UnverifiedInputFilePath,
        };

        fn valid_image_dimensions() -> ImageDimensions {
            ImageDimensions::from_image_path(
                InputFilePath::try_from_with_path_context(
                    super::super::existing_rgba8_path1(),
                    test_util::path::base_input().0,
                )
                .unwrap()
                .0,
            )
            .unwrap()
        }

        fn large_rgba8_path() -> UnverifiedInputFilePath<'static> {
            test_util::path::relative_input_file("image/image/stripes_large.png")
        }

        fn large_rgba16_path() -> UnverifiedInputFilePath<'static> {
            test_util::path::relative_input_file("image/image/red_large.png")
        }

        fn missing_image_path() -> UnverifiedInputFilePath<'static> {
            test_util::path::relative_input_file("image/image/not_found.png")
        }

        fn missing_error_message() -> &'static str {
            "does not exist"
        }

        fn non_image_path() -> UnverifiedInputFilePath<'static> {
            test_util::path::relative_input_file("empty.txt")
        }

        fn non_image_error_message() -> &'static str {
            "The file extension `.\"txt\"` was not recognized as an image format"
        }

        fn mismatch_error_message() -> &'static str {
            "mismatch in image dimensions, (width, height) = (20, 25) and (width, height) = (21, 25)"
        }

        mod success {
            use super::super::super::super::InputLosslessImagePath;
            use image_annealing_cli_util::path::TryIntoWithPathContext;
            use std::error::Error;

            #[test]
            fn rgba8() -> Result<(), Box<dyn Error>> {
                assert_eq!(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        super::super::super::valid_rgba8(),
                        test_util::path::base_input().0,
                    )?,
                    (
                        InputLosslessImagePath::Rgba8(
                            super::super::super::existing_rgba8_path1()
                                .try_into_with_path_context(test_util::path::base_input().0)?
                        ),
                        super::valid_image_dimensions()
                    )
                );
                Ok(())
            }

            #[test]
            fn rgba8x2() -> Result<(), Box<dyn Error>> {
                assert_eq!(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        super::super::super::valid_rgba8x2(),
                        test_util::path::base_input().0,
                    )?,
                    (
                        InputLosslessImagePath::Rgba8x2(
                            super::super::super::existing_rgba8_path1()
                                .try_into_with_path_context(test_util::path::base_input().0)?,
                            super::super::super::existing_rgba8_path2()
                                .try_into_with_path_context(test_util::path::base_input().0)?
                        ),
                        super::valid_image_dimensions()
                    )
                );
                Ok(())
            }

            #[test]
            fn rgba8x3() -> Result<(), Box<dyn Error>> {
                assert_eq!(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        super::super::super::valid_rgba8x3(),
                        test_util::path::base_input().0,
                    )?,
                    (
                        InputLosslessImagePath::Rgba8x3(
                            super::super::super::existing_rgba8_path1()
                                .try_into_with_path_context(test_util::path::base_input().0)?,
                            super::super::super::existing_rgba8_path2()
                                .try_into_with_path_context(test_util::path::base_input().0)?,
                            super::super::super::existing_rgba8_path3()
                                .try_into_with_path_context(test_util::path::base_input().0)?
                        ),
                        super::valid_image_dimensions()
                    )
                );
                Ok(())
            }

            #[test]
            fn rgba8x4() -> Result<(), Box<dyn Error>> {
                assert_eq!(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        super::super::super::valid_rgba8x4(),
                        test_util::path::base_input().0,
                    )?,
                    (
                        InputLosslessImagePath::Rgba8x4(
                            super::super::super::existing_rgba8_path1()
                                .try_into_with_path_context(test_util::path::base_input().0)?,
                            super::super::super::existing_rgba8_path2()
                                .try_into_with_path_context(test_util::path::base_input().0)?,
                            super::super::super::existing_rgba8_path3()
                                .try_into_with_path_context(test_util::path::base_input().0)?,
                            super::super::super::existing_rgba8_path4()
                                .try_into_with_path_context(test_util::path::base_input().0)?
                        ),
                        super::valid_image_dimensions()
                    )
                );
                Ok(())
            }

            #[test]
            fn rgba16() -> Result<(), Box<dyn Error>> {
                assert_eq!(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        super::super::super::valid_rgba16(),
                        test_util::path::base_input().0,
                    )?,
                    (
                        InputLosslessImagePath::Rgba16(
                            super::super::super::existing_rgba16_path1()
                                .try_into_with_path_context(test_util::path::base_input().0)?
                        ),
                        super::valid_image_dimensions()
                    )
                );
                Ok(())
            }

            #[test]
            fn rgba16x2() -> Result<(), Box<dyn Error>> {
                assert_eq!(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        super::super::super::valid_rgba16x2(),
                        test_util::path::base_input().0,
                    )?,
                    (
                        InputLosslessImagePath::Rgba16x2(
                            super::super::super::existing_rgba16_path1()
                                .try_into_with_path_context(test_util::path::base_input().0)?,
                            super::super::super::existing_rgba16_path1()
                                .try_into_with_path_context(test_util::path::base_input().0)?
                        ),
                        super::valid_image_dimensions()
                    )
                );
                Ok(())
            }

            #[test]
            fn rgba16_rgba8() -> Result<(), Box<dyn Error>> {
                assert_eq!(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        super::super::super::valid_rgba16_rgba8(),
                        test_util::path::base_input().0,
                    )?,
                    (
                        InputLosslessImagePath::Rgba16Rgba8(
                            super::super::super::existing_rgba16_path1()
                                .try_into_with_path_context(test_util::path::base_input().0)?,
                            super::super::super::existing_rgba8_path1()
                                .try_into_with_path_context(test_util::path::base_input().0)?,
                        ),
                        super::valid_image_dimensions()
                    )
                );
                Ok(())
            }

            #[test]
            fn rgba16_rgba8x2() -> Result<(), Box<dyn Error>> {
                assert_eq!(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        super::super::super::valid_rgba16_rgba8x2(),
                        test_util::path::base_input().0,
                    )?,
                    (
                        InputLosslessImagePath::Rgba16Rgba8x2(
                            super::super::super::existing_rgba16_path1()
                                .try_into_with_path_context(test_util::path::base_input().0)?,
                            super::super::super::existing_rgba8_path1()
                                .try_into_with_path_context(test_util::path::base_input().0)?,
                            super::super::super::existing_rgba8_path2()
                                .try_into_with_path_context(test_util::path::base_input().0)?,
                        ),
                        super::valid_image_dimensions()
                    )
                );
                Ok(())
            }
        }

        mod first_image_missing {
            use super::super::super::super::{
                InputLosslessImagePath, UnverifiedInputLosslessImagePath,
            };

            #[test]
            fn rgba8() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba8(super::missing_image_path()),
                        test_util::path::base_input().0,
                    ),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba8x2() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba8x2(
                            super::missing_image_path(),
                            super::super::super::existing_rgba8_path2(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba8x3() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba8x3(
                            super::missing_image_path(),
                            super::super::super::existing_rgba8_path2(),
                            super::super::super::existing_rgba8_path3(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba8x4() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba8x4(
                            super::missing_image_path(),
                            super::super::super::existing_rgba8_path2(),
                            super::super::super::existing_rgba8_path3(),
                            super::super::super::existing_rgba8_path4(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba16() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba16(super::missing_image_path()),
                        test_util::path::base_input().0,
                    ),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba16x2() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba16x2(
                            super::missing_image_path(),
                            super::super::super::existing_rgba16_path2(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba16Rgba8(
                            super::missing_image_path(),
                            super::super::super::existing_rgba8_path1(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8x2() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba16Rgba8x2(
                            super::missing_image_path(),
                            super::super::super::existing_rgba8_path1(),
                            super::super::super::existing_rgba8_path2(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::missing_error_message(),
                );
            }
        }

        mod second_image_missing {
            use super::super::super::super::{
                InputLosslessImagePath, UnverifiedInputLosslessImagePath,
            };

            #[test]
            fn rgba8x2() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba8x2(
                            super::super::super::existing_rgba8_path1(),
                            super::missing_image_path(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba8x3() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba8x3(
                            super::super::super::existing_rgba8_path1(),
                            super::missing_image_path(),
                            super::super::super::existing_rgba8_path3(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba8x4() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba8x4(
                            super::super::super::existing_rgba8_path1(),
                            super::missing_image_path(),
                            super::super::super::existing_rgba8_path3(),
                            super::super::super::existing_rgba8_path4(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba16x2() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba16x2(
                            super::super::super::existing_rgba16_path1(),
                            super::missing_image_path(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba16Rgba8(
                            super::super::super::existing_rgba16_path1(),
                            super::missing_image_path(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8x2() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba16Rgba8x2(
                            super::super::super::existing_rgba16_path1(),
                            super::missing_image_path(),
                            super::super::super::existing_rgba8_path2(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::missing_error_message(),
                );
            }
        }

        mod third_image_missing {
            use super::super::super::super::{
                InputLosslessImagePath, UnverifiedInputLosslessImagePath,
            };

            #[test]
            fn rgba8x3() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba8x3(
                            super::super::super::existing_rgba8_path1(),
                            super::super::super::existing_rgba8_path2(),
                            super::missing_image_path(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba8x4() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba8x4(
                            super::super::super::existing_rgba8_path1(),
                            super::super::super::existing_rgba8_path2(),
                            super::missing_image_path(),
                            super::super::super::existing_rgba8_path4(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8x2() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba16Rgba8x2(
                            super::super::super::existing_rgba16_path1(),
                            super::super::super::existing_rgba8_path1(),
                            super::missing_image_path(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::missing_error_message(),
                );
            }
        }

        mod fourth_image_missing {
            use super::super::super::super::{
                InputLosslessImagePath, UnverifiedInputLosslessImagePath,
            };

            #[test]
            fn rgba8x4() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba8x4(
                            super::super::super::existing_rgba8_path1(),
                            super::super::super::existing_rgba8_path2(),
                            super::super::super::existing_rgba8_path3(),
                            super::missing_image_path(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::missing_error_message(),
                );
            }
        }

        mod first_path_non_image {
            use super::super::super::super::{
                InputLosslessImagePath, UnverifiedInputLosslessImagePath,
            };

            #[test]
            fn rgba8() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba8(super::non_image_path()),
                        test_util::path::base_input().0,
                    ),
                    super::non_image_error_message(),
                );
            }

            #[test]
            fn rgba8x2() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba8x2(
                            super::non_image_path(),
                            super::super::super::existing_rgba8_path2(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::non_image_error_message(),
                );
            }

            #[test]
            fn rgba8x3() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba8x3(
                            super::non_image_path(),
                            super::super::super::existing_rgba8_path2(),
                            super::super::super::existing_rgba8_path3(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::non_image_error_message(),
                );
            }

            #[test]
            fn rgba8x4() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba8x4(
                            super::non_image_path(),
                            super::super::super::existing_rgba8_path2(),
                            super::super::super::existing_rgba8_path3(),
                            super::super::super::existing_rgba8_path4(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::non_image_error_message(),
                );
            }

            #[test]
            fn rgba16() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba16(super::non_image_path()),
                        test_util::path::base_input().0,
                    ),
                    super::non_image_error_message(),
                );
            }

            #[test]
            fn rgba16x2() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba16x2(
                            super::non_image_path(),
                            super::super::super::existing_rgba16_path2(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::non_image_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba16Rgba8(
                            super::non_image_path(),
                            super::super::super::existing_rgba8_path1(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::non_image_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8x2() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba16Rgba8x2(
                            super::non_image_path(),
                            super::super::super::existing_rgba8_path1(),
                            super::super::super::existing_rgba8_path2(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::non_image_error_message(),
                );
            }
        }

        mod second_path_non_image {
            use super::super::super::super::{
                InputLosslessImagePath, UnverifiedInputLosslessImagePath,
            };

            #[test]
            fn rgba8x2() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba8x2(
                            super::super::super::existing_rgba8_path1(),
                            super::non_image_path(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::non_image_error_message(),
                );
            }

            #[test]
            fn rgba8x3() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba8x3(
                            super::super::super::existing_rgba8_path1(),
                            super::non_image_path(),
                            super::super::super::existing_rgba8_path3(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::non_image_error_message(),
                );
            }

            #[test]
            fn rgba8x4() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba8x4(
                            super::super::super::existing_rgba8_path1(),
                            super::non_image_path(),
                            super::super::super::existing_rgba8_path3(),
                            super::super::super::existing_rgba8_path4(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::non_image_error_message(),
                );
            }

            #[test]
            fn rgba16x2() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba16x2(
                            super::super::super::existing_rgba16_path1(),
                            super::non_image_path(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::non_image_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba16Rgba8(
                            super::super::super::existing_rgba16_path1(),
                            super::non_image_path(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::non_image_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8x2() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba16Rgba8x2(
                            super::super::super::existing_rgba16_path1(),
                            super::non_image_path(),
                            super::super::super::existing_rgba8_path2(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::non_image_error_message(),
                );
            }
        }

        mod third_path_non_image {
            use super::super::super::super::{
                InputLosslessImagePath, UnverifiedInputLosslessImagePath,
            };

            #[test]
            fn rgba8x3() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba8x3(
                            super::super::super::existing_rgba8_path1(),
                            super::super::super::existing_rgba8_path2(),
                            super::non_image_path(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::non_image_error_message(),
                );
            }

            #[test]
            fn rgba8x4() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba8x4(
                            super::super::super::existing_rgba8_path1(),
                            super::super::super::existing_rgba8_path2(),
                            super::non_image_path(),
                            super::super::super::existing_rgba8_path4(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::non_image_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8x2() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba16Rgba8x2(
                            super::super::super::existing_rgba16_path1(),
                            super::super::super::existing_rgba8_path1(),
                            super::non_image_path(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::non_image_error_message(),
                );
            }
        }

        mod fourth_path_non_image {
            use super::super::super::super::{
                InputLosslessImagePath, UnverifiedInputLosslessImagePath,
            };

            #[test]
            fn rgba8x4() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba8x4(
                            super::super::super::existing_rgba8_path1(),
                            super::super::super::existing_rgba8_path2(),
                            super::super::super::existing_rgba8_path3(),
                            super::non_image_path(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::non_image_error_message(),
                );
            }
        }

        mod first_second_mismatch {
            use super::super::super::super::{
                InputLosslessImagePath, UnverifiedInputLosslessImagePath,
            };

            #[test]
            fn rgba8x2() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba8x2(
                            super::super::super::existing_rgba8_path1(),
                            super::large_rgba8_path(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::mismatch_error_message(),
                );
            }

            #[test]
            fn rgba8x3() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba8x3(
                            super::super::super::existing_rgba8_path1(),
                            super::large_rgba8_path(),
                            super::super::super::existing_rgba8_path3(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::mismatch_error_message(),
                );
            }

            #[test]
            fn rgba8x4() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba8x4(
                            super::super::super::existing_rgba8_path1(),
                            super::large_rgba8_path(),
                            super::super::super::existing_rgba8_path3(),
                            super::super::super::existing_rgba8_path4(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::mismatch_error_message(),
                );
            }

            #[test]
            fn rgba16x2() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba16x2(
                            super::super::super::existing_rgba16_path1(),
                            super::large_rgba16_path(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::mismatch_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba16Rgba8(
                            super::super::super::existing_rgba16_path1(),
                            super::large_rgba8_path(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::mismatch_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8x2() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba16Rgba8x2(
                            super::super::super::existing_rgba16_path1(),
                            super::large_rgba8_path(),
                            super::super::super::existing_rgba8_path2(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::mismatch_error_message(),
                );
            }
        }

        mod first_third_mismatch {
            use super::super::super::super::{
                InputLosslessImagePath, UnverifiedInputLosslessImagePath,
            };

            #[test]
            fn rgba8x3() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba8x3(
                            super::super::super::existing_rgba8_path1(),
                            super::super::super::existing_rgba8_path2(),
                            super::large_rgba8_path(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::mismatch_error_message(),
                );
            }

            #[test]
            fn rgba8x4() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba8x4(
                            super::super::super::existing_rgba8_path1(),
                            super::super::super::existing_rgba8_path2(),
                            super::large_rgba8_path(),
                            super::super::super::existing_rgba8_path4(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::mismatch_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8x2() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba16Rgba8x2(
                            super::super::super::existing_rgba16_path1(),
                            super::super::super::existing_rgba8_path1(),
                            super::large_rgba8_path(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::mismatch_error_message(),
                );
            }
        }

        mod first_fourth_mismatch {
            use super::super::super::super::{
                InputLosslessImagePath, UnverifiedInputLosslessImagePath,
            };

            #[test]
            fn rgba8x4() {
                test_util::assert_error_contains(
                    InputLosslessImagePath::try_from_unverified_with_path_context(
                        UnverifiedInputLosslessImagePath::Rgba8x4(
                            super::super::super::existing_rgba8_path1(),
                            super::super::super::existing_rgba8_path2(),
                            super::super::super::existing_rgba8_path3(),
                            super::large_rgba8_path(),
                        ),
                        test_util::path::base_input().0,
                    ),
                    super::mismatch_error_message(),
                );
            }
        }
    }
}

mod output_lossless_image_path {

    mod format {
        use super::super::super::ImageFormat;
        use super::super::super::OutputLosslessImagePath;

        #[test]
        fn rgba8() {
            assert_eq!(
                OutputLosslessImagePath::Rgba8(test_util::unique_absolute_output_file!()).format(),
                ImageFormat::Rgba8
            );
        }

        #[test]
        fn rgba8x2() {
            assert_eq!(
                OutputLosslessImagePath::Rgba8x2(
                    test_util::unique_absolute_output_file!(),
                    test_util::unique_absolute_output_file!()
                )
                .format(),
                ImageFormat::Rgba8x2
            );
        }

        #[test]
        fn rgba8x3() {
            assert_eq!(
                OutputLosslessImagePath::Rgba8x3(
                    test_util::unique_absolute_output_file!(),
                    test_util::unique_absolute_output_file!(),
                    test_util::unique_absolute_output_file!()
                )
                .format(),
                ImageFormat::Rgba8x3
            );
        }

        #[test]
        fn rgba8x4() {
            assert_eq!(
                OutputLosslessImagePath::Rgba8x4(
                    test_util::unique_absolute_output_file!(),
                    test_util::unique_absolute_output_file!(),
                    test_util::unique_absolute_output_file!(),
                    test_util::unique_absolute_output_file!()
                )
                .format(),
                ImageFormat::Rgba8x4
            );
        }

        #[test]
        fn rgba16() {
            assert_eq!(
                OutputLosslessImagePath::Rgba16(test_util::unique_absolute_output_file!()).format(),
                ImageFormat::Rgba16
            );
        }

        #[test]
        fn rgba16x2() {
            assert_eq!(
                OutputLosslessImagePath::Rgba16x2(
                    test_util::unique_absolute_output_file!(),
                    test_util::unique_absolute_output_file!()
                )
                .format(),
                ImageFormat::Rgba16x2
            );
        }

        #[test]
        fn rgba16_rgba8() {
            assert_eq!(
                OutputLosslessImagePath::Rgba16Rgba8(
                    test_util::unique_absolute_output_file!(),
                    test_util::unique_absolute_output_file!()
                )
                .format(),
                ImageFormat::Rgba16Rgba8
            );
        }

        #[test]
        fn rgba16_rgba8x2() {
            assert_eq!(
                OutputLosslessImagePath::Rgba16Rgba8x2(
                    test_util::unique_absolute_output_file!(),
                    test_util::unique_absolute_output_file!(),
                    test_util::unique_absolute_output_file!()
                )
                .format(),
                ImageFormat::Rgba16Rgba8x2
            );
        }
    }

    mod as_vec {
        use super::super::super::OutputLosslessImagePath;
        use image_annealing_cli_util::path::OutputFilePath;
        use std::path::{Path, PathBuf};

        fn extract_absolute_paths(paths: Vec<OutputFilePath<'static>>) -> Vec<PathBuf> {
            paths
                .into_iter()
                .map(|absolute_path| absolute_path.0.into_owned())
                .collect::<Vec<PathBuf>>()
        }

        #[test]
        fn rgba8() {
            let relative_paths = vec![test_util::unique_absolute_output_file!()];
            let output_path = OutputLosslessImagePath::Rgba8(relative_paths[0].clone());
            let absolute_paths = extract_absolute_paths(relative_paths);
            assert_eq!(
                output_path.as_vec(),
                absolute_paths
                    .iter()
                    .map(<PathBuf as AsRef<Path>>::as_ref)
                    .collect::<Vec<&Path>>()
            );
        }

        #[test]
        fn rgba8x2() {
            let relative_paths = vec![
                test_util::unique_absolute_output_file!(),
                test_util::unique_absolute_output_file!(),
            ];
            let output_path = OutputLosslessImagePath::Rgba8x2(
                relative_paths[0].clone(),
                relative_paths[1].clone(),
            );
            let absolute_paths = extract_absolute_paths(relative_paths);
            assert_eq!(
                output_path.as_vec(),
                absolute_paths
                    .iter()
                    .map(<PathBuf as AsRef<Path>>::as_ref)
                    .collect::<Vec<&Path>>()
            );
        }

        #[test]
        fn rgba8x3() {
            let relative_paths = vec![
                test_util::unique_absolute_output_file!(),
                test_util::unique_absolute_output_file!(),
                test_util::unique_absolute_output_file!(),
            ];
            let output_path = OutputLosslessImagePath::Rgba8x3(
                relative_paths[0].clone(),
                relative_paths[1].clone(),
                relative_paths[2].clone(),
            );
            let absolute_paths = extract_absolute_paths(relative_paths);
            assert_eq!(
                output_path.as_vec(),
                absolute_paths
                    .iter()
                    .map(<PathBuf as AsRef<Path>>::as_ref)
                    .collect::<Vec<&Path>>()
            );
        }

        #[test]
        fn rgba8x4() {
            let relative_paths = vec![
                test_util::unique_absolute_output_file!(),
                test_util::unique_absolute_output_file!(),
                test_util::unique_absolute_output_file!(),
                test_util::unique_absolute_output_file!(),
            ];
            let output_path = OutputLosslessImagePath::Rgba8x4(
                relative_paths[0].clone(),
                relative_paths[1].clone(),
                relative_paths[2].clone(),
                relative_paths[3].clone(),
            );
            let absolute_paths = extract_absolute_paths(relative_paths);
            assert_eq!(
                output_path.as_vec(),
                absolute_paths
                    .iter()
                    .map(<PathBuf as AsRef<Path>>::as_ref)
                    .collect::<Vec<&Path>>()
            );
        }

        #[test]
        fn rgba16() {
            let relative_paths = vec![test_util::unique_absolute_output_file!()];
            let output_path = OutputLosslessImagePath::Rgba16(relative_paths[0].clone());
            let absolute_paths = extract_absolute_paths(relative_paths);
            assert_eq!(
                output_path.as_vec(),
                absolute_paths
                    .iter()
                    .map(<PathBuf as AsRef<Path>>::as_ref)
                    .collect::<Vec<&Path>>()
            );
        }

        #[test]
        fn rgba16x2() {
            let relative_paths = vec![
                test_util::unique_absolute_output_file!(),
                test_util::unique_absolute_output_file!(),
            ];
            let output_path = OutputLosslessImagePath::Rgba16x2(
                relative_paths[0].clone(),
                relative_paths[1].clone(),
            );
            let absolute_paths = extract_absolute_paths(relative_paths);
            assert_eq!(
                output_path.as_vec(),
                absolute_paths
                    .iter()
                    .map(<PathBuf as AsRef<Path>>::as_ref)
                    .collect::<Vec<&Path>>()
            );
        }

        #[test]
        fn rgba16_rgba8() {
            let relative_paths = vec![
                test_util::unique_absolute_output_file!(),
                test_util::unique_absolute_output_file!(),
            ];
            let output_path = OutputLosslessImagePath::Rgba16Rgba8(
                relative_paths[0].clone(),
                relative_paths[1].clone(),
            );
            let absolute_paths = extract_absolute_paths(relative_paths);
            assert_eq!(
                output_path.as_vec(),
                absolute_paths
                    .iter()
                    .map(<PathBuf as AsRef<Path>>::as_ref)
                    .collect::<Vec<&Path>>()
            );
        }

        #[test]
        fn rgba16_rgba8x2() {
            let relative_paths = vec![
                test_util::unique_absolute_output_file!(),
                test_util::unique_absolute_output_file!(),
                test_util::unique_absolute_output_file!(),
            ];
            let output_path = OutputLosslessImagePath::Rgba16Rgba8x2(
                relative_paths[0].clone(),
                relative_paths[1].clone(),
                relative_paths[2].clone(),
            );
            let absolute_paths = extract_absolute_paths(relative_paths);
            assert_eq!(
                output_path.as_vec(),
                absolute_paths
                    .iter()
                    .map(<PathBuf as AsRef<Path>>::as_ref)
                    .collect::<Vec<&Path>>()
            );
        }
    }

    mod from_unverified_path {
        use image_annealing_cli_util::path::UnverifiedOutputFilePath;

        mod success {
            use super::super::super::super::{
                OutputLosslessImagePath, UnverifiedOutputLosslessImagePath,
            };
            use image_annealing_cli_util::path::{TryFromWithPathContext, TryIntoWithPathContext};
            use std::error::Error;

            #[test]
            fn rgba8() -> Result<(), Box<dyn Error>> {
                let unverified_path0 = test_util::unique_relative_output_file!();
                assert_eq!(
                    OutputLosslessImagePath::try_from_with_path_context(
                        UnverifiedOutputLosslessImagePath::Rgba8(unverified_path0.clone()),
                        test_util::path::base_output().0
                    )?,
                    OutputLosslessImagePath::Rgba8(
                        unverified_path0
                            .try_into_with_path_context(test_util::path::base_output().0)?
                    )
                );
                Ok(())
            }

            #[test]
            fn rgba8x2() -> Result<(), Box<dyn Error>> {
                let unverified_path0 = test_util::unique_relative_output_file!();
                let unverified_path1 = test_util::unique_relative_output_file!();
                assert_eq!(
                    OutputLosslessImagePath::try_from_with_path_context(
                        UnverifiedOutputLosslessImagePath::Rgba8x2(
                            unverified_path0.clone(),
                            unverified_path1.clone()
                        ),
                        test_util::path::base_output().0
                    )?,
                    OutputLosslessImagePath::Rgba8x2(
                        unverified_path0
                            .try_into_with_path_context(test_util::path::base_output().0)?,
                        unverified_path1
                            .try_into_with_path_context(test_util::path::base_output().0)?
                    )
                );
                Ok(())
            }

            #[test]
            fn rgba8x3() -> Result<(), Box<dyn Error>> {
                let unverified_path0 = test_util::unique_relative_output_file!();
                let unverified_path1 = test_util::unique_relative_output_file!();
                let unverified_path2 = test_util::unique_relative_output_file!();
                assert_eq!(
                    OutputLosslessImagePath::try_from_with_path_context(
                        UnverifiedOutputLosslessImagePath::Rgba8x3(
                            unverified_path0.clone(),
                            unverified_path1.clone(),
                            unverified_path2.clone()
                        ),
                        test_util::path::base_output().0
                    )?,
                    OutputLosslessImagePath::Rgba8x3(
                        unverified_path0
                            .try_into_with_path_context(test_util::path::base_output().0)?,
                        unverified_path1
                            .try_into_with_path_context(test_util::path::base_output().0)?,
                        unverified_path2
                            .try_into_with_path_context(test_util::path::base_output().0)?
                    )
                );
                Ok(())
            }

            #[test]
            fn rgba8x4() -> Result<(), Box<dyn Error>> {
                let unverified_path0 = test_util::unique_relative_output_file!();
                let unverified_path1 = test_util::unique_relative_output_file!();
                let unverified_path2 = test_util::unique_relative_output_file!();
                let unverified_path3 = test_util::unique_relative_output_file!();
                assert_eq!(
                    OutputLosslessImagePath::try_from_with_path_context(
                        UnverifiedOutputLosslessImagePath::Rgba8x4(
                            unverified_path0.clone(),
                            unverified_path1.clone(),
                            unverified_path2.clone(),
                            unverified_path3.clone()
                        ),
                        test_util::path::base_output().0
                    )?,
                    OutputLosslessImagePath::Rgba8x4(
                        unverified_path0
                            .try_into_with_path_context(test_util::path::base_output().0)?,
                        unverified_path1
                            .try_into_with_path_context(test_util::path::base_output().0)?,
                        unverified_path2
                            .try_into_with_path_context(test_util::path::base_output().0)?,
                        unverified_path3
                            .try_into_with_path_context(test_util::path::base_output().0)?
                    )
                );
                Ok(())
            }

            #[test]
            fn rgba16() -> Result<(), Box<dyn Error>> {
                let unverified_path0 = test_util::unique_relative_output_file!();
                assert_eq!(
                    OutputLosslessImagePath::try_from_with_path_context(
                        UnverifiedOutputLosslessImagePath::Rgba16(unverified_path0.clone()),
                        test_util::path::base_output().0
                    )?,
                    OutputLosslessImagePath::Rgba16(
                        unverified_path0
                            .try_into_with_path_context(test_util::path::base_output().0)?
                    )
                );
                Ok(())
            }

            #[test]
            fn rgba16x2() -> Result<(), Box<dyn Error>> {
                let unverified_path0 = test_util::unique_relative_output_file!();
                let unverified_path1 = test_util::unique_relative_output_file!();
                assert_eq!(
                    OutputLosslessImagePath::try_from_with_path_context(
                        UnverifiedOutputLosslessImagePath::Rgba16x2(
                            unverified_path0.clone(),
                            unverified_path1.clone()
                        ),
                        test_util::path::base_output().0
                    )?,
                    OutputLosslessImagePath::Rgba16x2(
                        unverified_path0
                            .try_into_with_path_context(test_util::path::base_output().0)?,
                        unverified_path1
                            .try_into_with_path_context(test_util::path::base_output().0)?
                    )
                );
                Ok(())
            }

            #[test]
            fn rgba16_rgba8() -> Result<(), Box<dyn Error>> {
                let unverified_path0 = test_util::unique_relative_output_file!();
                let unverified_path1 = test_util::unique_relative_output_file!();
                assert_eq!(
                    OutputLosslessImagePath::try_from_with_path_context(
                        UnverifiedOutputLosslessImagePath::Rgba16Rgba8(
                            unverified_path0.clone(),
                            unverified_path1.clone()
                        ),
                        test_util::path::base_output().0
                    )?,
                    OutputLosslessImagePath::Rgba16Rgba8(
                        unverified_path0
                            .try_into_with_path_context(test_util::path::base_output().0)?,
                        unverified_path1
                            .try_into_with_path_context(test_util::path::base_output().0)?
                    )
                );
                Ok(())
            }

            #[test]
            fn rgba16_rgba8x2() -> Result<(), Box<dyn Error>> {
                let unverified_path0 = test_util::unique_relative_output_file!();
                let unverified_path1 = test_util::unique_relative_output_file!();
                let unverified_path2 = test_util::unique_relative_output_file!();
                assert_eq!(
                    OutputLosslessImagePath::try_from_with_path_context(
                        UnverifiedOutputLosslessImagePath::Rgba16Rgba8x2(
                            unverified_path0.clone(),
                            unverified_path1.clone(),
                            unverified_path2.clone()
                        ),
                        test_util::path::base_output().0
                    )?,
                    OutputLosslessImagePath::Rgba16Rgba8x2(
                        unverified_path0
                            .try_into_with_path_context(test_util::path::base_output().0)?,
                        unverified_path1
                            .try_into_with_path_context(test_util::path::base_output().0)?,
                        unverified_path2
                            .try_into_with_path_context(test_util::path::base_output().0)?
                    )
                );
                Ok(())
            }
        }

        fn missing_directory_path() -> UnverifiedOutputFilePath<'static> {
            test_util::path::relative_output_file("not_found/cannot_create")
        }

        fn missing_error_message() -> &'static str {
            "No such file or directory"
        }

        mod first_path_directory_missing {
            use super::super::super::super::{
                OutputLosslessImagePath, UnverifiedOutputLosslessImagePath,
            };
            use image_annealing_cli_util::path::TryFromWithPathContext;

            #[test]
            fn rgba8() {
                test_util::assert_error_contains(
                    OutputLosslessImagePath::try_from_with_path_context(
                        UnverifiedOutputLosslessImagePath::Rgba8(super::missing_directory_path()),
                        test_util::path::base_output().0,
                    ),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba8x2() {
                test_util::assert_error_contains(
                    OutputLosslessImagePath::try_from_with_path_context(
                        UnverifiedOutputLosslessImagePath::Rgba8x2(
                            super::missing_directory_path(),
                            test_util::unique_relative_output_file!(),
                        ),
                        test_util::path::base_output().0,
                    ),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba8x3() {
                test_util::assert_error_contains(
                    OutputLosslessImagePath::try_from_with_path_context(
                        UnverifiedOutputLosslessImagePath::Rgba8x3(
                            super::missing_directory_path(),
                            test_util::unique_relative_output_file!(),
                            test_util::unique_relative_output_file!(),
                        ),
                        test_util::path::base_output().0,
                    ),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba8x4() {
                test_util::assert_error_contains(
                    OutputLosslessImagePath::try_from_with_path_context(
                        UnverifiedOutputLosslessImagePath::Rgba8x4(
                            super::missing_directory_path(),
                            test_util::unique_relative_output_file!(),
                            test_util::unique_relative_output_file!(),
                            test_util::unique_relative_output_file!(),
                        ),
                        test_util::path::base_output().0,
                    ),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba16() {
                test_util::assert_error_contains(
                    OutputLosslessImagePath::try_from_with_path_context(
                        UnverifiedOutputLosslessImagePath::Rgba16(super::missing_directory_path()),
                        test_util::path::base_output().0,
                    ),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba16x2() {
                test_util::assert_error_contains(
                    OutputLosslessImagePath::try_from_with_path_context(
                        UnverifiedOutputLosslessImagePath::Rgba16x2(
                            super::missing_directory_path(),
                            test_util::unique_relative_output_file!(),
                        ),
                        test_util::path::base_output().0,
                    ),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8() {
                test_util::assert_error_contains(
                    OutputLosslessImagePath::try_from_with_path_context(
                        UnverifiedOutputLosslessImagePath::Rgba16Rgba8(
                            super::missing_directory_path(),
                            test_util::unique_relative_output_file!(),
                        ),
                        test_util::path::base_output().0,
                    ),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8x2() {
                test_util::assert_error_contains(
                    OutputLosslessImagePath::try_from_with_path_context(
                        UnverifiedOutputLosslessImagePath::Rgba16Rgba8x2(
                            super::missing_directory_path(),
                            test_util::unique_relative_output_file!(),
                            test_util::unique_relative_output_file!(),
                        ),
                        test_util::path::base_output().0,
                    ),
                    super::missing_error_message(),
                );
            }
        }

        mod second_path_directory_missing {
            use super::super::super::super::{
                OutputLosslessImagePath, UnverifiedOutputLosslessImagePath,
            };
            use image_annealing_cli_util::path::TryFromWithPathContext;

            #[test]
            fn rgba8x2() {
                test_util::assert_error_contains(
                    OutputLosslessImagePath::try_from_with_path_context(
                        UnverifiedOutputLosslessImagePath::Rgba8x2(
                            test_util::unique_relative_output_file!(),
                            super::missing_directory_path(),
                        ),
                        test_util::path::base_output().0,
                    ),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba8x3() {
                test_util::assert_error_contains(
                    OutputLosslessImagePath::try_from_with_path_context(
                        UnverifiedOutputLosslessImagePath::Rgba8x3(
                            test_util::unique_relative_output_file!(),
                            super::missing_directory_path(),
                            test_util::unique_relative_output_file!(),
                        ),
                        test_util::path::base_output().0,
                    ),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba8x4() {
                test_util::assert_error_contains(
                    OutputLosslessImagePath::try_from_with_path_context(
                        UnverifiedOutputLosslessImagePath::Rgba8x4(
                            test_util::unique_relative_output_file!(),
                            super::missing_directory_path(),
                            test_util::unique_relative_output_file!(),
                            test_util::unique_relative_output_file!(),
                        ),
                        test_util::path::base_output().0,
                    ),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba16x2() {
                test_util::assert_error_contains(
                    OutputLosslessImagePath::try_from_with_path_context(
                        UnverifiedOutputLosslessImagePath::Rgba16x2(
                            test_util::unique_relative_output_file!(),
                            super::missing_directory_path(),
                        ),
                        test_util::path::base_output().0,
                    ),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8() {
                test_util::assert_error_contains(
                    OutputLosslessImagePath::try_from_with_path_context(
                        UnverifiedOutputLosslessImagePath::Rgba16Rgba8(
                            test_util::unique_relative_output_file!(),
                            super::missing_directory_path(),
                        ),
                        test_util::path::base_output().0,
                    ),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8x2() {
                test_util::assert_error_contains(
                    OutputLosslessImagePath::try_from_with_path_context(
                        UnverifiedOutputLosslessImagePath::Rgba16Rgba8x2(
                            test_util::unique_relative_output_file!(),
                            super::missing_directory_path(),
                            test_util::unique_relative_output_file!(),
                        ),
                        test_util::path::base_output().0,
                    ),
                    super::missing_error_message(),
                );
            }
        }

        mod third_path_directory_missing {
            use super::super::super::super::{
                OutputLosslessImagePath, UnverifiedOutputLosslessImagePath,
            };
            use image_annealing_cli_util::path::TryFromWithPathContext;

            #[test]
            fn rgba8x3() {
                test_util::assert_error_contains(
                    OutputLosslessImagePath::try_from_with_path_context(
                        UnverifiedOutputLosslessImagePath::Rgba8x3(
                            test_util::unique_relative_output_file!(),
                            test_util::unique_relative_output_file!(),
                            super::missing_directory_path(),
                        ),
                        test_util::path::base_output().0,
                    ),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba8x4() {
                test_util::assert_error_contains(
                    OutputLosslessImagePath::try_from_with_path_context(
                        UnverifiedOutputLosslessImagePath::Rgba8x4(
                            test_util::unique_relative_output_file!(),
                            test_util::unique_relative_output_file!(),
                            super::missing_directory_path(),
                            test_util::unique_relative_output_file!(),
                        ),
                        test_util::path::base_output().0,
                    ),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8x2() {
                test_util::assert_error_contains(
                    OutputLosslessImagePath::try_from_with_path_context(
                        UnverifiedOutputLosslessImagePath::Rgba16Rgba8x2(
                            test_util::unique_relative_output_file!(),
                            test_util::unique_relative_output_file!(),
                            super::missing_directory_path(),
                        ),
                        test_util::path::base_output().0,
                    ),
                    super::missing_error_message(),
                );
            }
        }

        mod fourth_path_directory_missing {
            use super::super::super::super::{
                OutputLosslessImagePath, UnverifiedOutputLosslessImagePath,
            };
            use image_annealing_cli_util::path::TryFromWithPathContext;

            #[test]
            fn rgba8x4() {
                test_util::assert_error_contains(
                    OutputLosslessImagePath::try_from_with_path_context(
                        UnverifiedOutputLosslessImagePath::Rgba8x4(
                            test_util::unique_relative_output_file!(),
                            test_util::unique_relative_output_file!(),
                            test_util::unique_relative_output_file!(),
                            super::missing_directory_path(),
                        ),
                        test_util::path::base_output().0,
                    ),
                    super::missing_error_message(),
                );
            }
        }
    }
}
