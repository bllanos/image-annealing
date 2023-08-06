use super::UnverifiedLosslessImagePath;

fn existing_rgba8_path1() -> String {
    test_util::make_test_data_path_string(["image", "image", "stripes.png"])
}

fn existing_rgba8_path2() -> String {
    test_util::make_test_data_path_string(["image", "image", "green.png"])
}

fn existing_rgba8_path3() -> String {
    test_util::make_test_data_path_string(["image", "image", "pastel.png"])
}

fn existing_rgba8_path4() -> String {
    test_util::make_test_data_path_string(["image", "image", "parque.png"])
}

fn existing_rgba16_path1() -> String {
    test_util::make_test_data_path_string(["image", "image", "red.png"])
}

fn existing_rgba16_path2() -> String {
    test_util::make_test_data_path_string(["image", "image", "leopard.png"])
}

fn valid_rgba8() -> UnverifiedLosslessImagePath {
    UnverifiedLosslessImagePath::Rgba8(existing_rgba8_path1())
}

fn valid_rgba8x2() -> UnverifiedLosslessImagePath {
    UnverifiedLosslessImagePath::Rgba8x2(existing_rgba8_path1(), existing_rgba8_path2())
}

fn valid_rgba8x3() -> UnverifiedLosslessImagePath {
    UnverifiedLosslessImagePath::Rgba8x3(
        existing_rgba8_path1(),
        existing_rgba8_path2(),
        existing_rgba8_path3(),
    )
}

fn valid_rgba8x4() -> UnverifiedLosslessImagePath {
    UnverifiedLosslessImagePath::Rgba8x4(
        existing_rgba8_path1(),
        existing_rgba8_path2(),
        existing_rgba8_path3(),
        existing_rgba8_path4(),
    )
}

fn valid_rgba16() -> UnverifiedLosslessImagePath {
    UnverifiedLosslessImagePath::Rgba16(existing_rgba16_path1())
}

fn valid_rgba16x2() -> UnverifiedLosslessImagePath {
    UnverifiedLosslessImagePath::Rgba16x2(existing_rgba16_path1(), existing_rgba16_path2())
}

fn valid_rgba16_rgba8() -> UnverifiedLosslessImagePath {
    UnverifiedLosslessImagePath::Rgba16Rgba8(existing_rgba16_path1(), existing_rgba8_path1())
}

fn valid_rgba16_rgba8x2() -> UnverifiedLosslessImagePath {
    UnverifiedLosslessImagePath::Rgba16Rgba8x2(
        existing_rgba16_path1(),
        existing_rgba8_path1(),
        existing_rgba8_path2(),
    )
}

mod unverified_lossless_image_path {
    use super::super::UnverifiedLosslessImagePath;
    use image_annealing::compute::format::ImageFormat;

    mod from_raw {
        use super::super::super::UnverifiedLosslessImagePath;
        use image_annealing::compute::format::ImageFormat;

        #[test]
        fn rgba8() {
            let paths = &[String::from("1")];
            let expected = UnverifiedLosslessImagePath::Rgba8(paths[0].clone());
            assert_eq!(
                UnverifiedLosslessImagePath::from_raw(ImageFormat::Rgba8, paths),
                expected
            );
        }

        #[test]
        fn rgba8x2() {
            let paths = &[String::from("1"), String::from("2")];
            let expected = UnverifiedLosslessImagePath::Rgba8x2(paths[0].clone(), paths[1].clone());
            assert_eq!(
                UnverifiedLosslessImagePath::from_raw(ImageFormat::Rgba8x2, paths),
                expected
            );
        }

        #[test]
        fn rgba8x3() {
            let paths = &[String::from("1"), String::from("2"), String::from("3")];
            let expected = UnverifiedLosslessImagePath::Rgba8x3(
                paths[0].clone(),
                paths[1].clone(),
                paths[2].clone(),
            );
            assert_eq!(
                UnverifiedLosslessImagePath::from_raw(ImageFormat::Rgba8x3, paths),
                expected
            );
        }

        #[test]
        fn rgba8x4() {
            let paths = &[
                String::from("1"),
                String::from("2"),
                String::from("3"),
                String::from("4"),
            ];
            let expected = UnverifiedLosslessImagePath::Rgba8x4(
                paths[0].clone(),
                paths[1].clone(),
                paths[2].clone(),
                paths[3].clone(),
            );
            assert_eq!(
                UnverifiedLosslessImagePath::from_raw(ImageFormat::Rgba8x4, paths),
                expected
            );
        }

        #[test]
        fn rgba16() {
            let paths = &[String::from("1")];
            let expected = UnverifiedLosslessImagePath::Rgba16(paths[0].clone());
            assert_eq!(
                UnverifiedLosslessImagePath::from_raw(ImageFormat::Rgba16, paths),
                expected
            );
        }

        #[test]
        fn rgba16x2() {
            let paths = &[String::from("1"), String::from("2")];
            let expected =
                UnverifiedLosslessImagePath::Rgba16x2(paths[0].clone(), paths[1].clone());
            assert_eq!(
                UnverifiedLosslessImagePath::from_raw(ImageFormat::Rgba16x2, paths),
                expected
            );
        }

        #[test]
        fn rgba16_rgba8() {
            let paths = &[String::from("1"), String::from("2")];
            let expected =
                UnverifiedLosslessImagePath::Rgba16Rgba8(paths[0].clone(), paths[1].clone());
            assert_eq!(
                UnverifiedLosslessImagePath::from_raw(ImageFormat::Rgba16Rgba8, paths),
                expected
            );
        }

        #[test]
        fn rgba16_rgba8x2() {
            let paths = &[String::from("1"), String::from("2"), String::from("3")];
            let expected = UnverifiedLosslessImagePath::Rgba16Rgba8x2(
                paths[0].clone(),
                paths[1].clone(),
                paths[2].clone(),
            );
            assert_eq!(
                UnverifiedLosslessImagePath::from_raw(ImageFormat::Rgba16Rgba8x2, paths),
                expected
            );
        }
    }

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

mod lossless_image_path {
    use super::super::LosslessImagePath;

    fn valid_rgba8() -> LosslessImagePath {
        LosslessImagePath::Rgba8(super::existing_rgba8_path1())
    }

    fn valid_rgba8x2() -> LosslessImagePath {
        LosslessImagePath::Rgba8x2(super::existing_rgba8_path1(), super::existing_rgba8_path2())
    }

    fn valid_rgba8x3() -> LosslessImagePath {
        LosslessImagePath::Rgba8x3(
            super::existing_rgba8_path1(),
            super::existing_rgba8_path2(),
            super::existing_rgba8_path3(),
        )
    }

    fn valid_rgba8x4() -> LosslessImagePath {
        LosslessImagePath::Rgba8x4(
            super::existing_rgba8_path1(),
            super::existing_rgba8_path2(),
            super::existing_rgba8_path3(),
            super::existing_rgba8_path4(),
        )
    }

    fn valid_rgba16() -> LosslessImagePath {
        LosslessImagePath::Rgba16(super::existing_rgba16_path1())
    }

    fn valid_rgba16x2() -> LosslessImagePath {
        LosslessImagePath::Rgba16x2(
            super::existing_rgba16_path1(),
            super::existing_rgba16_path2(),
        )
    }

    fn valid_rgba16_rgba8() -> LosslessImagePath {
        LosslessImagePath::Rgba16Rgba8(
            super::existing_rgba16_path1(),
            super::existing_rgba8_path1(),
        )
    }

    fn valid_rgba16_rgba8x2() -> LosslessImagePath {
        LosslessImagePath::Rgba16Rgba8x2(
            super::existing_rgba16_path1(),
            super::existing_rgba8_path1(),
            super::existing_rgba8_path2(),
        )
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

    mod to_vec {
        use super::super::super::LosslessImagePath;

        #[test]
        fn rgba8() {
            let paths = vec![String::from("1")];
            assert_eq!(
                LosslessImagePath::Rgba8(paths[0].clone()).to_vec(),
                paths.iter().collect::<Vec<&String>>()
            );
        }

        #[test]
        fn rgba8x2() {
            let paths = vec![String::from("1"), String::from("2")];
            assert_eq!(
                LosslessImagePath::Rgba8x2(paths[0].clone(), paths[1].clone()).to_vec(),
                paths.iter().collect::<Vec<&String>>()
            );
        }

        #[test]
        fn rgba8x3() {
            let paths = vec![String::from("1"), String::from("2"), String::from("3")];
            assert_eq!(
                LosslessImagePath::Rgba8x3(paths[0].clone(), paths[1].clone(), paths[2].clone())
                    .to_vec(),
                paths.iter().collect::<Vec<&String>>()
            );
        }

        #[test]
        fn rgba8x4() {
            let paths = vec![
                String::from("1"),
                String::from("2"),
                String::from("3"),
                String::from("4"),
            ];
            assert_eq!(
                LosslessImagePath::Rgba8x4(
                    paths[0].clone(),
                    paths[1].clone(),
                    paths[2].clone(),
                    paths[3].clone()
                )
                .to_vec(),
                paths.iter().collect::<Vec<&String>>()
            );
        }

        #[test]
        fn rgba16() {
            let paths = vec![String::from("1")];
            assert_eq!(
                LosslessImagePath::Rgba16(paths[0].clone()).to_vec(),
                paths.iter().collect::<Vec<&String>>()
            );
        }

        #[test]
        fn rgba16x2() {
            let paths = vec![String::from("1"), String::from("2")];
            assert_eq!(
                LosslessImagePath::Rgba16x2(paths[0].clone(), paths[1].clone()).to_vec(),
                paths.iter().collect::<Vec<&String>>()
            );
        }

        #[test]
        fn rgba16_rgba8() {
            let paths = vec![String::from("1"), String::from("2")];
            assert_eq!(
                LosslessImagePath::Rgba16Rgba8(paths[0].clone(), paths[1].clone()).to_vec(),
                paths.iter().collect::<Vec<&String>>()
            );
        }

        #[test]
        fn rgba16_rgba8x2() {
            let paths = vec![String::from("1"), String::from("2"), String::from("3")];
            assert_eq!(
                LosslessImagePath::Rgba16Rgba8x2(
                    paths[0].clone(),
                    paths[1].clone(),
                    paths[2].clone()
                )
                .to_vec(),
                paths.iter().collect::<Vec<&String>>()
            );
        }
    }

    mod from_input_path {
        use image_annealing::ImageDimensions;

        fn valid_image_dimensions() -> ImageDimensions {
            ImageDimensions::from_image_path(super::super::existing_rgba8_path1()).unwrap()
        }

        fn large_rgba8_path() -> String {
            test_util::make_test_data_path_string(["image", "image", "stripes_large.png"])
        }

        fn large_rgba16_path() -> String {
            test_util::make_test_data_path_string(["image", "image", "red_large.png"])
        }

        fn missing_image_path() -> String {
            test_util::make_test_data_path_string(["image", "image", "not_found.png"])
        }

        fn missing_error_message() -> &'static str {
            "does not exist"
        }

        fn non_image_path() -> String {
            test_util::make_test_data_path_string(["empty.txt"])
        }

        fn non_image_error_message() -> &'static str {
            "The file extension `.\"txt\"` was not recognized as an image format"
        }

        fn mismatch_error_message() -> &'static str {
            "mismatch in image dimensions, (width, height) = (20, 25) and (width, height) = (21, 25)"
        }

        mod success {
            use super::super::super::super::LosslessImagePath;
            use std::error::Error;

            #[test]
            fn rgba8() -> Result<(), Box<dyn Error>> {
                assert_eq!(
                    LosslessImagePath::from_input_path(super::super::super::valid_rgba8())?,
                    (super::super::valid_rgba8(), super::valid_image_dimensions())
                );
                Ok(())
            }

            #[test]
            fn rgba8x2() -> Result<(), Box<dyn Error>> {
                assert_eq!(
                    LosslessImagePath::from_input_path(super::super::super::valid_rgba8x2())?,
                    (
                        super::super::valid_rgba8x2(),
                        super::valid_image_dimensions()
                    )
                );
                Ok(())
            }

            #[test]
            fn rgba8x3() -> Result<(), Box<dyn Error>> {
                assert_eq!(
                    LosslessImagePath::from_input_path(super::super::super::valid_rgba8x3())?,
                    (
                        super::super::valid_rgba8x3(),
                        super::valid_image_dimensions()
                    )
                );
                Ok(())
            }

            #[test]
            fn rgba8x4() -> Result<(), Box<dyn Error>> {
                assert_eq!(
                    LosslessImagePath::from_input_path(super::super::super::valid_rgba8x4())?,
                    (
                        super::super::valid_rgba8x4(),
                        super::valid_image_dimensions()
                    )
                );
                Ok(())
            }

            #[test]
            fn rgba16() -> Result<(), Box<dyn Error>> {
                assert_eq!(
                    LosslessImagePath::from_input_path(super::super::super::valid_rgba16())?,
                    (
                        super::super::valid_rgba16(),
                        super::valid_image_dimensions()
                    )
                );
                Ok(())
            }

            #[test]
            fn rgba16x2() -> Result<(), Box<dyn Error>> {
                assert_eq!(
                    LosslessImagePath::from_input_path(super::super::super::valid_rgba16x2())?,
                    (
                        super::super::valid_rgba16x2(),
                        super::valid_image_dimensions()
                    )
                );
                Ok(())
            }

            #[test]
            fn rgba16_rgba8() -> Result<(), Box<dyn Error>> {
                assert_eq!(
                    LosslessImagePath::from_input_path(super::super::super::valid_rgba16_rgba8())?,
                    (
                        super::super::valid_rgba16_rgba8(),
                        super::valid_image_dimensions()
                    )
                );
                Ok(())
            }

            #[test]
            fn rgba16_rgba8x2() -> Result<(), Box<dyn Error>> {
                assert_eq!(
                    LosslessImagePath::from_input_path(super::super::super::valid_rgba16_rgba8x2())?,
                    (
                        super::super::valid_rgba16_rgba8x2(),
                        super::valid_image_dimensions()
                    )
                );
                Ok(())
            }
        }

        mod first_image_missing {
            use super::super::super::super::{LosslessImagePath, UnverifiedLosslessImagePath};

            #[test]
            fn rgba8() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba8(
                        super::missing_image_path(),
                    )),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba8x2() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba8x2(
                        super::missing_image_path(),
                        super::super::super::existing_rgba8_path2(),
                    )),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba8x3() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba8x3(
                        super::missing_image_path(),
                        super::super::super::existing_rgba8_path2(),
                        super::super::super::existing_rgba8_path3(),
                    )),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba8x4() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba8x4(
                        super::missing_image_path(),
                        super::super::super::existing_rgba8_path2(),
                        super::super::super::existing_rgba8_path3(),
                        super::super::super::existing_rgba8_path4(),
                    )),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba16() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba16(
                        super::missing_image_path(),
                    )),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba16x2() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba16x2(
                        super::missing_image_path(),
                        super::super::super::existing_rgba16_path2(),
                    )),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba16Rgba8(
                        super::missing_image_path(),
                        super::super::super::existing_rgba8_path1(),
                    )),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8x2() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba16Rgba8x2(
                        super::missing_image_path(),
                        super::super::super::existing_rgba8_path1(),
                        super::super::super::existing_rgba8_path2(),
                    )),
                    super::missing_error_message(),
                );
            }
        }

        mod second_image_missing {
            use super::super::super::super::{LosslessImagePath, UnverifiedLosslessImagePath};

            #[test]
            fn rgba8x2() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba8x2(
                        super::super::super::existing_rgba8_path1(),
                        super::missing_image_path(),
                    )),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba8x3() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba8x3(
                        super::super::super::existing_rgba8_path1(),
                        super::missing_image_path(),
                        super::super::super::existing_rgba8_path3(),
                    )),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba8x4() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba8x4(
                        super::super::super::existing_rgba8_path1(),
                        super::missing_image_path(),
                        super::super::super::existing_rgba8_path3(),
                        super::super::super::existing_rgba8_path4(),
                    )),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba16x2() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba16x2(
                        super::super::super::existing_rgba16_path1(),
                        super::missing_image_path(),
                    )),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba16Rgba8(
                        super::super::super::existing_rgba16_path1(),
                        super::missing_image_path(),
                    )),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8x2() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba16Rgba8x2(
                        super::super::super::existing_rgba16_path1(),
                        super::missing_image_path(),
                        super::super::super::existing_rgba8_path2(),
                    )),
                    super::missing_error_message(),
                );
            }
        }

        mod third_image_missing {
            use super::super::super::super::{LosslessImagePath, UnverifiedLosslessImagePath};

            #[test]
            fn rgba8x3() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba8x3(
                        super::super::super::existing_rgba8_path1(),
                        super::super::super::existing_rgba8_path2(),
                        super::missing_image_path(),
                    )),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba8x4() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba8x4(
                        super::super::super::existing_rgba8_path1(),
                        super::super::super::existing_rgba8_path2(),
                        super::missing_image_path(),
                        super::super::super::existing_rgba8_path4(),
                    )),
                    super::missing_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8x2() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba16Rgba8x2(
                        super::super::super::existing_rgba16_path1(),
                        super::super::super::existing_rgba8_path1(),
                        super::missing_image_path(),
                    )),
                    super::missing_error_message(),
                );
            }
        }

        mod fourth_image_missing {
            use super::super::super::super::{LosslessImagePath, UnverifiedLosslessImagePath};

            #[test]
            fn rgba8x4() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba8x4(
                        super::super::super::existing_rgba8_path1(),
                        super::super::super::existing_rgba8_path2(),
                        super::super::super::existing_rgba8_path3(),
                        super::missing_image_path(),
                    )),
                    super::missing_error_message(),
                );
            }
        }

        mod first_path_non_image {
            use super::super::super::super::{LosslessImagePath, UnverifiedLosslessImagePath};

            #[test]
            fn rgba8() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba8(
                        super::non_image_path(),
                    )),
                    super::non_image_error_message(),
                );
            }

            #[test]
            fn rgba8x2() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba8x2(
                        super::non_image_path(),
                        super::super::super::existing_rgba8_path2(),
                    )),
                    super::non_image_error_message(),
                );
            }

            #[test]
            fn rgba8x3() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba8x3(
                        super::non_image_path(),
                        super::super::super::existing_rgba8_path2(),
                        super::super::super::existing_rgba8_path3(),
                    )),
                    super::non_image_error_message(),
                );
            }

            #[test]
            fn rgba8x4() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba8x4(
                        super::non_image_path(),
                        super::super::super::existing_rgba8_path2(),
                        super::super::super::existing_rgba8_path3(),
                        super::super::super::existing_rgba8_path4(),
                    )),
                    super::non_image_error_message(),
                );
            }

            #[test]
            fn rgba16() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba16(
                        super::non_image_path(),
                    )),
                    super::non_image_error_message(),
                );
            }

            #[test]
            fn rgba16x2() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba16x2(
                        super::non_image_path(),
                        super::super::super::existing_rgba16_path2(),
                    )),
                    super::non_image_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba16Rgba8(
                        super::non_image_path(),
                        super::super::super::existing_rgba8_path1(),
                    )),
                    super::non_image_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8x2() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba16Rgba8x2(
                        super::non_image_path(),
                        super::super::super::existing_rgba8_path1(),
                        super::super::super::existing_rgba8_path2(),
                    )),
                    super::non_image_error_message(),
                );
            }
        }

        mod second_path_non_image {
            use super::super::super::super::{LosslessImagePath, UnverifiedLosslessImagePath};

            #[test]
            fn rgba8x2() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba8x2(
                        super::super::super::existing_rgba8_path1(),
                        super::non_image_path(),
                    )),
                    super::non_image_error_message(),
                );
            }

            #[test]
            fn rgba8x3() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba8x3(
                        super::super::super::existing_rgba8_path1(),
                        super::non_image_path(),
                        super::super::super::existing_rgba8_path3(),
                    )),
                    super::non_image_error_message(),
                );
            }

            #[test]
            fn rgba8x4() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba8x4(
                        super::super::super::existing_rgba8_path1(),
                        super::non_image_path(),
                        super::super::super::existing_rgba8_path3(),
                        super::super::super::existing_rgba8_path4(),
                    )),
                    super::non_image_error_message(),
                );
            }

            #[test]
            fn rgba16x2() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba16x2(
                        super::super::super::existing_rgba16_path1(),
                        super::non_image_path(),
                    )),
                    super::non_image_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba16Rgba8(
                        super::super::super::existing_rgba16_path1(),
                        super::non_image_path(),
                    )),
                    super::non_image_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8x2() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba16Rgba8x2(
                        super::super::super::existing_rgba16_path1(),
                        super::non_image_path(),
                        super::super::super::existing_rgba8_path2(),
                    )),
                    super::non_image_error_message(),
                );
            }
        }

        mod third_path_non_image {
            use super::super::super::super::{LosslessImagePath, UnverifiedLosslessImagePath};

            #[test]
            fn rgba8x3() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba8x3(
                        super::super::super::existing_rgba8_path1(),
                        super::super::super::existing_rgba8_path2(),
                        super::non_image_path(),
                    )),
                    super::non_image_error_message(),
                );
            }

            #[test]
            fn rgba8x4() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba8x4(
                        super::super::super::existing_rgba8_path1(),
                        super::super::super::existing_rgba8_path2(),
                        super::non_image_path(),
                        super::super::super::existing_rgba8_path4(),
                    )),
                    super::non_image_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8x2() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba16Rgba8x2(
                        super::super::super::existing_rgba16_path1(),
                        super::super::super::existing_rgba8_path1(),
                        super::non_image_path(),
                    )),
                    super::non_image_error_message(),
                );
            }
        }

        mod fourth_path_non_image {
            use super::super::super::super::{LosslessImagePath, UnverifiedLosslessImagePath};

            #[test]
            fn rgba8x4() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba8x4(
                        super::super::super::existing_rgba8_path1(),
                        super::super::super::existing_rgba8_path2(),
                        super::super::super::existing_rgba8_path3(),
                        super::non_image_path(),
                    )),
                    super::non_image_error_message(),
                );
            }
        }

        mod first_second_mismatch {
            use super::super::super::super::{LosslessImagePath, UnverifiedLosslessImagePath};

            #[test]
            fn rgba8x2() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba8x2(
                        super::super::super::existing_rgba8_path1(),
                        super::large_rgba8_path(),
                    )),
                    super::mismatch_error_message(),
                );
            }

            #[test]
            fn rgba8x3() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba8x3(
                        super::super::super::existing_rgba8_path1(),
                        super::large_rgba8_path(),
                        super::super::super::existing_rgba8_path3(),
                    )),
                    super::mismatch_error_message(),
                );
            }

            #[test]
            fn rgba8x4() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba8x4(
                        super::super::super::existing_rgba8_path1(),
                        super::large_rgba8_path(),
                        super::super::super::existing_rgba8_path3(),
                        super::super::super::existing_rgba8_path4(),
                    )),
                    super::mismatch_error_message(),
                );
            }

            #[test]
            fn rgba16x2() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba16x2(
                        super::super::super::existing_rgba16_path1(),
                        super::large_rgba16_path(),
                    )),
                    super::mismatch_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba16Rgba8(
                        super::super::super::existing_rgba16_path1(),
                        super::large_rgba8_path(),
                    )),
                    super::mismatch_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8x2() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba16Rgba8x2(
                        super::super::super::existing_rgba16_path1(),
                        super::large_rgba8_path(),
                        super::super::super::existing_rgba8_path2(),
                    )),
                    super::mismatch_error_message(),
                );
            }
        }

        mod first_third_mismatch {
            use super::super::super::super::{LosslessImagePath, UnverifiedLosslessImagePath};

            #[test]
            fn rgba8x3() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba8x3(
                        super::super::super::existing_rgba8_path1(),
                        super::super::super::existing_rgba8_path2(),
                        super::large_rgba8_path(),
                    )),
                    super::mismatch_error_message(),
                );
            }

            #[test]
            fn rgba8x4() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba8x4(
                        super::super::super::existing_rgba8_path1(),
                        super::super::super::existing_rgba8_path2(),
                        super::large_rgba8_path(),
                        super::super::super::existing_rgba8_path4(),
                    )),
                    super::mismatch_error_message(),
                );
            }

            #[test]
            fn rgba16_rgba8x2() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba16Rgba8x2(
                        super::super::super::existing_rgba16_path1(),
                        super::super::super::existing_rgba8_path1(),
                        super::large_rgba8_path(),
                    )),
                    super::mismatch_error_message(),
                );
            }
        }

        mod first_fourth_mismatch {
            use super::super::super::super::{LosslessImagePath, UnverifiedLosslessImagePath};

            #[test]
            fn rgba8x4() {
                test_util::assert_error_contains(
                    LosslessImagePath::from_input_path(UnverifiedLosslessImagePath::Rgba8x4(
                        super::super::super::existing_rgba8_path1(),
                        super::super::super::existing_rgba8_path2(),
                        super::super::super::existing_rgba8_path3(),
                        super::large_rgba8_path(),
                    )),
                    super::mismatch_error_message(),
                );
            }
        }
    }

    mod from_output_path {
        use super::super::super::super::LosslessImagePath;

        #[test]
        fn rgba8() {
            assert_eq!(
                LosslessImagePath::from_output_path(super::super::valid_rgba8()),
                super::valid_rgba8()
            );
        }

        #[test]
        fn rgba8x2() {
            assert_eq!(
                LosslessImagePath::from_output_path(super::super::valid_rgba8x2()),
                super::valid_rgba8x2()
            );
        }

        #[test]
        fn rgba8x3() {
            assert_eq!(
                LosslessImagePath::from_output_path(super::super::valid_rgba8x3()),
                super::valid_rgba8x3()
            );
        }

        #[test]
        fn rgba8x4() {
            assert_eq!(
                LosslessImagePath::from_output_path(super::super::valid_rgba8x4()),
                super::valid_rgba8x4()
            );
        }

        #[test]
        fn rgba16() {
            assert_eq!(
                LosslessImagePath::from_output_path(super::super::valid_rgba16()),
                super::valid_rgba16()
            );
        }

        #[test]
        fn rgba16x2() {
            assert_eq!(
                LosslessImagePath::from_output_path(super::super::valid_rgba16x2()),
                super::valid_rgba16x2()
            );
        }

        #[test]
        fn rgba16_rgba8() {
            assert_eq!(
                LosslessImagePath::from_output_path(super::super::valid_rgba16_rgba8()),
                super::valid_rgba16_rgba8()
            );
        }

        #[test]
        fn rgba16_rgba8x2() {
            assert_eq!(
                LosslessImagePath::from_output_path(super::super::valid_rgba16_rgba8x2()),
                super::valid_rgba16_rgba8x2()
            );
        }
    }
}
