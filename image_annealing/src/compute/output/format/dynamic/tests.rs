mod io {
    use super::super::super::{
        Rgba16Image, Rgba16ImageBuffer, Rgba16Rgba8Image, Rgba16Rgba8x2Image, Rgba16x2Image,
        Rgba8Image, Rgba8x2Image, Rgba8x3Image, Rgba8x4Image,
    };
    use super::super::LosslessImage;
    use std::path::PathBuf;

    fn assert_not_files(paths: &[PathBuf]) {
        assert!(paths.iter().all(|path| !path.is_file()));
    }

    fn existing_rgba8_path() -> PathBuf {
        test_utils::make_test_data_path(&["image", "image", "stripes.png"])
    }

    fn existing_rgba16_path() -> PathBuf {
        test_utils::make_test_data_path(&["image", "image", "red.png"])
    }

    fn large_rgba8_path() -> PathBuf {
        test_utils::make_test_data_path(&["image", "image", "stripes_large.png"])
    }

    fn large_rgba16_path() -> PathBuf {
        test_utils::make_test_data_path(&["image", "image", "red_large.png"])
    }

    fn missing_image_path() -> PathBuf {
        test_utils::make_test_data_path(&["image", "image", "not_found.png"])
    }

    fn missing_directory_path() -> PathBuf {
        test_utils::make_test_output_path(&["not_found", "cannot_create"])
    }

    fn mismatch_error_message() -> &'static str {
        "mismatch in image dimensions, (width, height) = (20, 25) and (width, height) = (21, 25)"
    }

    fn missing_error_message() -> &'static str {
        "No such file or directory"
    }

    fn valid_rgba8() -> image::RgbaImage {
        image::RgbaImage::from_pixel(2, 1, image::Rgba([0, 0, 0, 0]))
    }

    fn valid_rgba16() -> Rgba16ImageBuffer {
        Rgba16ImageBuffer::from_pixel(2, 1, image::Rgba([0, 0, 0, 0]))
    }

    fn valid_rgba8x2() -> LosslessImage {
        LosslessImage::Rgba8x2(Rgba8x2Image::new(valid_rgba8(), valid_rgba8()).unwrap())
    }

    fn valid_rgba8x3() -> LosslessImage {
        LosslessImage::Rgba8x3(
            Rgba8x3Image::new(valid_rgba8(), valid_rgba8(), valid_rgba8()).unwrap(),
        )
    }

    fn valid_rgba8x4() -> LosslessImage {
        LosslessImage::Rgba8x4(
            Rgba8x4Image::new(valid_rgba8(), valid_rgba8(), valid_rgba8(), valid_rgba8()).unwrap(),
        )
    }

    fn valid_rgba16x2() -> LosslessImage {
        LosslessImage::Rgba16x2(Rgba16x2Image::new(valid_rgba16(), valid_rgba16()).unwrap())
    }

    mod success {
        use super::super::super::super::{
            ImageFileReader, ImageFileWriter, Rgba16ImageBuffer, VectorFieldImageBuffer,
        };
        use super::super::super::{
            ImageFormat, LosslessImage, Rgba16Image, Rgba16Rgba8Image, Rgba16Rgba8x2Image,
            Rgba16x2Image, Rgba8Image, Rgba8x2Image, Rgba8x3Image, Rgba8x4Image,
        };
        use std::error::Error;
        use std::path::PathBuf;
        use test_utils::image::{DimensionsAndRgba16Buffer, DimensionsAndRgba8Buffer};

        fn assert_same_paths(a: &[PathBuf], b: &[&PathBuf]) {
            assert_eq!(a.len(), b.len());
            assert!(a
                .iter()
                .zip(b.iter())
                .all(|(path_a, path_b)| &path_a == path_b));
        }

        #[test]
        fn rgba8() -> Result<(), Box<dyn Error>> {
            let DimensionsAndRgba8Buffer { image, .. } =
                test_utils::image::linear_indices_with_bias_to_colors(0);
            let output_image = LosslessImage::Rgba8(Rgba8Image::new(image.clone())?);

            match output_image {
                LosslessImage::Rgba8(ref wrapped_image) => assert_eq!(wrapped_image, &image),
                _ => panic!("Unexpected image format"),
            }

            let path = test_utils::make_test_output_path_string(&[
                "compute_output_format_dynamic_io_rgba8",
            ]);
            let expected_output_path = VectorFieldImageBuffer::make_filename(&path);
            if expected_output_path.is_file() {
                panic!("Rgba8 image already exists in the filesystem")
            }

            let full_output_paths = output_image.save_add_extension(&[path])?;
            assert_same_paths(&full_output_paths, &[&expected_output_path]);

            let read_image = VectorFieldImageBuffer::load(&full_output_paths[0])?;
            assert_eq!(read_image, image);

            let input_image = LosslessImage::load(ImageFormat::Rgba8, &full_output_paths)?;

            match input_image {
                LosslessImage::Rgba8(ref wrapped_image) => assert_eq!(wrapped_image, &image),
                _ => panic!("Unexpected image format"),
            }
            assert_eq!(input_image, output_image);

            Ok(std::fs::remove_file(expected_output_path)?)
        }

        #[test]
        fn rgba8x2() -> Result<(), Box<dyn Error>> {
            let DimensionsAndRgba8Buffer { image: image1, .. } =
                test_utils::image::linear_indices_with_bias_to_colors(0);
            let DimensionsAndRgba8Buffer { image: image2, .. } =
                test_utils::image::linear_indices_with_bias_to_colors(*image1.last().unwrap() + 1);
            let output_image =
                LosslessImage::Rgba8x2(Rgba8x2Image::new(image1.clone(), image2.clone())?);

            match output_image {
                LosslessImage::Rgba8x2(ref wrapped_image) => {
                    assert_eq!(wrapped_image.first_inner(), &image1);
                    assert_eq!(wrapped_image.second_inner(), &image2)
                }
                _ => panic!("Unexpected image format"),
            }

            let paths = [
                test_utils::make_test_output_path_string(&[
                    "compute_output_format_dynamic_io_rgba8x2_1",
                ]),
                test_utils::make_test_output_path_string(&[
                    "compute_output_format_dynamic_io_rgba8x2_2",
                ]),
            ];
            let expected_output_paths = [
                VectorFieldImageBuffer::make_filename(&paths[0]),
                VectorFieldImageBuffer::make_filename(&paths[1]),
            ];
            super::assert_not_files(&expected_output_paths);

            let full_output_paths = output_image.save_add_extension(&paths)?;
            assert_eq!(full_output_paths, expected_output_paths);

            let mut read_image = VectorFieldImageBuffer::load(&full_output_paths[0])?;
            assert_eq!(read_image, image1);
            read_image = VectorFieldImageBuffer::load(&full_output_paths[1])?;
            assert_eq!(read_image, image2);

            let input_image = LosslessImage::load(ImageFormat::Rgba8x2, &full_output_paths)?;

            match input_image {
                LosslessImage::Rgba8x2(ref wrapped_image) => {
                    assert_eq!(wrapped_image.first_inner(), &image1);
                    assert_eq!(wrapped_image.second_inner(), &image2)
                }
                _ => panic!("Unexpected image format"),
            }
            assert_eq!(input_image, output_image);

            Ok(expected_output_paths
                .iter()
                .try_for_each(std::fs::remove_file)?)
        }

        #[test]
        fn rgba8x3() -> Result<(), Box<dyn Error>> {
            let DimensionsAndRgba8Buffer { image: image1, .. } =
                test_utils::image::linear_indices_with_bias_to_colors(0);
            let DimensionsAndRgba8Buffer { image: image2, .. } =
                test_utils::image::linear_indices_with_bias_to_colors(*image1.last().unwrap() + 1);
            let DimensionsAndRgba8Buffer { image: image3, .. } =
                test_utils::image::linear_indices_with_bias_to_colors(*image2.last().unwrap() + 1);
            let output_image = LosslessImage::Rgba8x3(Rgba8x3Image::new(
                image1.clone(),
                image2.clone(),
                image3.clone(),
            )?);

            match output_image {
                LosslessImage::Rgba8x3(ref wrapped_image) => {
                    assert_eq!(wrapped_image.first_inner(), &image1);
                    assert_eq!(wrapped_image.second_inner(), &image2);
                    assert_eq!(wrapped_image.third_inner(), &image3)
                }
                _ => panic!("Unexpected image format"),
            }

            let paths = [
                test_utils::make_test_output_path_string(&[
                    "compute_output_format_dynamic_io_rgba8x3_1",
                ]),
                test_utils::make_test_output_path_string(&[
                    "compute_output_format_dynamic_io_rgba8x3_2",
                ]),
                test_utils::make_test_output_path_string(&[
                    "compute_output_format_dynamic_io_rgba8x3_3",
                ]),
            ];
            let expected_output_paths = [
                VectorFieldImageBuffer::make_filename(&paths[0]),
                VectorFieldImageBuffer::make_filename(&paths[1]),
                VectorFieldImageBuffer::make_filename(&paths[2]),
            ];
            super::assert_not_files(&expected_output_paths);

            let full_output_paths = output_image.save_add_extension(&paths)?;
            assert_eq!(full_output_paths, expected_output_paths);

            let mut read_image = VectorFieldImageBuffer::load(&full_output_paths[0])?;
            assert_eq!(read_image, image1);
            read_image = VectorFieldImageBuffer::load(&full_output_paths[1])?;
            assert_eq!(read_image, image2);
            read_image = VectorFieldImageBuffer::load(&full_output_paths[2])?;
            assert_eq!(read_image, image3);

            let input_image = LosslessImage::load(ImageFormat::Rgba8x3, &full_output_paths)?;

            match input_image {
                LosslessImage::Rgba8x3(ref wrapped_image) => {
                    assert_eq!(wrapped_image.first_inner(), &image1);
                    assert_eq!(wrapped_image.second_inner(), &image2);
                    assert_eq!(wrapped_image.third_inner(), &image3)
                }
                _ => panic!("Unexpected image format"),
            }
            assert_eq!(input_image, output_image);

            Ok(expected_output_paths
                .iter()
                .try_for_each(std::fs::remove_file)?)
        }

        #[test]
        fn rgba8x4() -> Result<(), Box<dyn Error>> {
            let DimensionsAndRgba8Buffer { image: image1, .. } =
                test_utils::image::linear_indices_with_bias_to_colors(0);
            let DimensionsAndRgba8Buffer { image: image2, .. } =
                test_utils::image::linear_indices_with_bias_to_colors(*image1.last().unwrap() + 1);
            let DimensionsAndRgba8Buffer { image: image3, .. } =
                test_utils::image::linear_indices_with_bias_to_colors(*image2.last().unwrap() + 1);
            let DimensionsAndRgba8Buffer { image: image4, .. } =
                test_utils::image::linear_indices_with_bias_to_colors(*image3.last().unwrap() + 1);
            let output_image = LosslessImage::Rgba8x4(Rgba8x4Image::new(
                image1.clone(),
                image2.clone(),
                image3.clone(),
                image4.clone(),
            )?);

            match output_image {
                LosslessImage::Rgba8x4(ref wrapped_image) => {
                    assert_eq!(wrapped_image.first_inner(), &image1);
                    assert_eq!(wrapped_image.second_inner(), &image2);
                    assert_eq!(wrapped_image.third_inner(), &image3);
                    assert_eq!(wrapped_image.fourth_inner(), &image4)
                }
                _ => panic!("Unexpected image format"),
            }

            let paths = [
                test_utils::make_test_output_path_string(&[
                    "compute_output_format_dynamic_io_rgba8x4_1",
                ]),
                test_utils::make_test_output_path_string(&[
                    "compute_output_format_dynamic_io_rgba8x4_2",
                ]),
                test_utils::make_test_output_path_string(&[
                    "compute_output_format_dynamic_io_rgba8x4_3",
                ]),
                test_utils::make_test_output_path_string(&[
                    "compute_output_format_dynamic_io_rgba8x4_4",
                ]),
            ];
            let expected_output_paths = [
                VectorFieldImageBuffer::make_filename(&paths[0]),
                VectorFieldImageBuffer::make_filename(&paths[1]),
                VectorFieldImageBuffer::make_filename(&paths[2]),
                VectorFieldImageBuffer::make_filename(&paths[3]),
            ];
            super::assert_not_files(&expected_output_paths);

            let full_output_paths = output_image.save_add_extension(&paths)?;
            assert_eq!(full_output_paths, expected_output_paths);

            let mut read_image = VectorFieldImageBuffer::load(&full_output_paths[0])?;
            assert_eq!(read_image, image1);
            read_image = VectorFieldImageBuffer::load(&full_output_paths[1])?;
            assert_eq!(read_image, image2);
            read_image = VectorFieldImageBuffer::load(&full_output_paths[2])?;
            assert_eq!(read_image, image3);
            read_image = VectorFieldImageBuffer::load(&full_output_paths[3])?;
            assert_eq!(read_image, image4);

            let input_image = LosslessImage::load(ImageFormat::Rgba8x4, &full_output_paths)?;

            match input_image {
                LosslessImage::Rgba8x4(ref wrapped_image) => {
                    assert_eq!(wrapped_image.first_inner(), &image1);
                    assert_eq!(wrapped_image.second_inner(), &image2);
                    assert_eq!(wrapped_image.third_inner(), &image3);
                    assert_eq!(wrapped_image.fourth_inner(), &image4)
                }
                _ => panic!("Unexpected image format"),
            }
            assert_eq!(input_image, output_image);

            Ok(expected_output_paths
                .iter()
                .try_for_each(std::fs::remove_file)?)
        }

        #[test]
        fn rgba16() -> Result<(), Box<dyn Error>> {
            let DimensionsAndRgba16Buffer { image, .. } =
                test_utils::image::linear_indices_with_bias_to_colors(0);
            let output_image = LosslessImage::Rgba16(Rgba16Image::new(image.clone())?);

            match output_image {
                LosslessImage::Rgba16(ref wrapped_image) => assert_eq!(wrapped_image, &image),
                _ => panic!("Unexpected image format"),
            }

            let path = test_utils::make_test_output_path_string(&[
                "compute_output_format_dynamic_io_rgba16",
            ]);
            let expected_output_path = Rgba16ImageBuffer::make_filename(&path);
            if expected_output_path.is_file() {
                panic!("Rgba16 image already exists in the filesystem")
            }

            let full_output_paths = output_image.save_add_extension(&[path])?;
            assert_same_paths(&full_output_paths, &[&expected_output_path]);

            let read_image = Rgba16ImageBuffer::load(&full_output_paths[0])?;
            assert_eq!(read_image, image);

            let input_image = LosslessImage::load(ImageFormat::Rgba16, &full_output_paths)?;

            match input_image {
                LosslessImage::Rgba16(ref wrapped_image) => assert_eq!(wrapped_image, &image),
                _ => panic!("Unexpected image format"),
            }
            assert_eq!(input_image, output_image);

            Ok(std::fs::remove_file(expected_output_path)?)
        }

        #[test]
        fn rgba16x2() -> Result<(), Box<dyn Error>> {
            let DimensionsAndRgba16Buffer { image: image1, .. } =
                test_utils::image::linear_indices_with_bias_to_colors(0);
            let DimensionsAndRgba16Buffer { image: image2, .. } =
                test_utils::image::linear_indices_with_bias_to_colors(*image1.last().unwrap() + 1);
            let output_image =
                LosslessImage::Rgba16x2(Rgba16x2Image::new(image1.clone(), image2.clone())?);

            match output_image {
                LosslessImage::Rgba16x2(ref wrapped_image) => {
                    assert_eq!(wrapped_image.first_inner(), &image1);
                    assert_eq!(wrapped_image.second_inner(), &image2)
                }
                _ => panic!("Unexpected image format"),
            }

            let paths = [
                test_utils::make_test_output_path_string(&[
                    "compute_output_format_dynamic_io_rgba16x2_1",
                ]),
                test_utils::make_test_output_path_string(&[
                    "compute_output_format_dynamic_io_rgba16x2_2",
                ]),
            ];
            let expected_output_paths = [
                Rgba16ImageBuffer::make_filename(&paths[0]),
                Rgba16ImageBuffer::make_filename(&paths[1]),
            ];
            super::assert_not_files(&expected_output_paths);

            let full_output_paths = output_image.save_add_extension(&paths)?;
            assert_eq!(full_output_paths, expected_output_paths);

            let mut read_image = Rgba16ImageBuffer::load(&full_output_paths[0])?;
            assert_eq!(read_image, image1);
            read_image = Rgba16ImageBuffer::load(&full_output_paths[1])?;
            assert_eq!(read_image, image2);

            let input_image = LosslessImage::load(ImageFormat::Rgba16x2, &full_output_paths)?;

            match input_image {
                LosslessImage::Rgba16x2(ref wrapped_image) => {
                    assert_eq!(wrapped_image.first_inner(), &image1);
                    assert_eq!(wrapped_image.second_inner(), &image2)
                }
                _ => panic!("Unexpected image format"),
            }
            assert_eq!(input_image, output_image);

            Ok(expected_output_paths
                .iter()
                .try_for_each(std::fs::remove_file)?)
        }

        #[test]
        fn rgba16_rgba8() -> Result<(), Box<dyn Error>> {
            let DimensionsAndRgba16Buffer { image: image1, .. } =
                test_utils::image::linear_indices_with_bias_to_colors(0);
            let DimensionsAndRgba8Buffer { image: image2, .. } =
                test_utils::image::linear_indices_with_bias_to_colors(*image1.last().unwrap() + 1);
            let output_image =
                LosslessImage::Rgba16Rgba8(Rgba16Rgba8Image::new(image1.clone(), image2.clone())?);

            match output_image {
                LosslessImage::Rgba16Rgba8(ref wrapped_image) => {
                    assert_eq!(wrapped_image.first_inner(), &image1);
                    assert_eq!(wrapped_image.second_inner(), &image2)
                }
                _ => panic!("Unexpected image format"),
            }

            let paths = [
                test_utils::make_test_output_path_string(&[
                    "compute_output_format_dynamic_io_rgba16_rgba8_1",
                ]),
                test_utils::make_test_output_path_string(&[
                    "compute_output_format_dynamic_io_rgba16_rgba8_2",
                ]),
            ];
            let expected_output_paths = [
                Rgba16ImageBuffer::make_filename(&paths[0]),
                VectorFieldImageBuffer::make_filename(&paths[1]),
            ];
            super::assert_not_files(&expected_output_paths);

            let full_output_paths = output_image.save_add_extension(&paths)?;
            assert_eq!(full_output_paths, expected_output_paths);

            let read_image = Rgba16ImageBuffer::load(&full_output_paths[0])?;
            assert_eq!(read_image, image1);
            let read_image = VectorFieldImageBuffer::load(&full_output_paths[1])?;
            assert_eq!(read_image, image2);

            let input_image = LosslessImage::load(ImageFormat::Rgba16Rgba8, &full_output_paths)?;

            match input_image {
                LosslessImage::Rgba16Rgba8(ref wrapped_image) => {
                    assert_eq!(wrapped_image.first_inner(), &image1);
                    assert_eq!(wrapped_image.second_inner(), &image2)
                }
                _ => panic!("Unexpected image format"),
            }
            assert_eq!(input_image, output_image);

            Ok(expected_output_paths
                .iter()
                .try_for_each(std::fs::remove_file)?)
        }

        #[test]
        fn rgba16_rgba8x2() -> Result<(), Box<dyn Error>> {
            let DimensionsAndRgba16Buffer { image: image1, .. } =
                test_utils::image::linear_indices_with_bias_to_colors(0);
            let DimensionsAndRgba8Buffer { image: image2, .. } =
                test_utils::image::linear_indices_with_bias_to_colors(*image1.last().unwrap() + 1);
            let DimensionsAndRgba8Buffer { image: image3, .. } =
                test_utils::image::linear_indices_with_bias_to_colors(*image2.last().unwrap() + 1);
            let output_image = LosslessImage::Rgba16Rgba8x2(Rgba16Rgba8x2Image::new(
                image1.clone(),
                image2.clone(),
                image3.clone(),
            )?);

            match output_image {
                LosslessImage::Rgba16Rgba8x2(ref wrapped_image) => {
                    assert_eq!(wrapped_image.first_inner(), &image1);
                    assert_eq!(wrapped_image.second_inner(), &image2);
                    assert_eq!(wrapped_image.third_inner(), &image3)
                }
                _ => panic!("Unexpected image format"),
            }

            let paths = [
                test_utils::make_test_output_path_string(&[
                    "compute_output_format_dynamic_io_rgba16_rgba8x2_1",
                ]),
                test_utils::make_test_output_path_string(&[
                    "compute_output_format_dynamic_io_rgba16_rgba8x2_2",
                ]),
                test_utils::make_test_output_path_string(&[
                    "compute_output_format_dynamic_io_rgba16_rgba8x2_3",
                ]),
            ];
            let expected_output_paths = [
                Rgba16ImageBuffer::make_filename(&paths[0]),
                VectorFieldImageBuffer::make_filename(&paths[1]),
                VectorFieldImageBuffer::make_filename(&paths[2]),
            ];
            super::assert_not_files(&expected_output_paths);

            let full_output_paths = output_image.save_add_extension(&paths)?;
            assert_eq!(full_output_paths, expected_output_paths);

            let read_image = Rgba16ImageBuffer::load(&full_output_paths[0])?;
            assert_eq!(read_image, image1);
            let mut read_image = VectorFieldImageBuffer::load(&full_output_paths[1])?;
            assert_eq!(read_image, image2);
            read_image = VectorFieldImageBuffer::load(&full_output_paths[2])?;
            assert_eq!(read_image, image3);

            let input_image = LosslessImage::load(ImageFormat::Rgba16Rgba8x2, &full_output_paths)?;

            match input_image {
                LosslessImage::Rgba16Rgba8x2(ref wrapped_image) => {
                    assert_eq!(wrapped_image.first_inner(), &image1);
                    assert_eq!(wrapped_image.second_inner(), &image2);
                    assert_eq!(wrapped_image.third_inner(), &image3)
                }
                _ => panic!("Unexpected image format"),
            }
            assert_eq!(input_image, output_image);

            Ok(expected_output_paths
                .iter()
                .try_for_each(std::fs::remove_file)?)
        }
    }

    mod first_image_missing {
        use super::super::super::{ImageFormat, LosslessImage};

        #[test]
        fn rgba8x2() {
            test_utils::assert_error_contains(
                LosslessImage::load(
                    ImageFormat::Rgba8x2,
                    &[super::missing_image_path(), super::existing_rgba8_path()],
                ),
                super::missing_error_message(),
            );
        }

        #[test]
        fn rgba8x3() {
            test_utils::assert_error_contains(
                LosslessImage::load(
                    ImageFormat::Rgba8x3,
                    &[
                        super::missing_image_path(),
                        super::existing_rgba8_path(),
                        super::existing_rgba8_path(),
                    ],
                ),
                super::missing_error_message(),
            );
        }

        #[test]
        fn rgba8x4() {
            test_utils::assert_error_contains(
                LosslessImage::load(
                    ImageFormat::Rgba8x4,
                    &[
                        super::missing_image_path(),
                        super::existing_rgba8_path(),
                        super::existing_rgba8_path(),
                        super::existing_rgba8_path(),
                    ],
                ),
                super::missing_error_message(),
            );
        }

        #[test]
        fn rgba16x2() {
            test_utils::assert_error_contains(
                LosslessImage::load(
                    ImageFormat::Rgba16x2,
                    &[super::missing_image_path(), super::existing_rgba16_path()],
                ),
                super::missing_error_message(),
            );
        }
    }

    mod second_image_missing {
        use super::super::super::{ImageFormat, LosslessImage};

        #[test]
        fn rgba8x2() {
            test_utils::assert_error_contains(
                LosslessImage::load(
                    ImageFormat::Rgba8x2,
                    &[super::existing_rgba8_path(), super::missing_image_path()],
                ),
                super::missing_error_message(),
            );
        }

        #[test]
        fn rgba8x3() {
            test_utils::assert_error_contains(
                LosslessImage::load(
                    ImageFormat::Rgba8x3,
                    &[
                        super::existing_rgba8_path(),
                        super::missing_image_path(),
                        super::existing_rgba8_path(),
                    ],
                ),
                super::missing_error_message(),
            );
        }

        #[test]
        fn rgba8x4() {
            test_utils::assert_error_contains(
                LosslessImage::load(
                    ImageFormat::Rgba8x4,
                    &[
                        super::existing_rgba8_path(),
                        super::missing_image_path(),
                        super::existing_rgba8_path(),
                        super::existing_rgba8_path(),
                    ],
                ),
                super::missing_error_message(),
            );
        }

        #[test]
        fn rgba16x2() {
            test_utils::assert_error_contains(
                LosslessImage::load(
                    ImageFormat::Rgba16x2,
                    &[super::existing_rgba16_path(), super::missing_image_path()],
                ),
                super::missing_error_message(),
            );
        }
    }

    mod third_image_missing {
        use super::super::super::{ImageFormat, LosslessImage};

        #[test]
        fn rgba8x3() {
            test_utils::assert_error_contains(
                LosslessImage::load(
                    ImageFormat::Rgba8x3,
                    &[
                        super::existing_rgba8_path(),
                        super::existing_rgba8_path(),
                        super::missing_image_path(),
                    ],
                ),
                super::missing_error_message(),
            );
        }

        #[test]
        fn rgba8x4() {
            test_utils::assert_error_contains(
                LosslessImage::load(
                    ImageFormat::Rgba8x4,
                    &[
                        super::existing_rgba8_path(),
                        super::existing_rgba8_path(),
                        super::missing_image_path(),
                        super::existing_rgba8_path(),
                    ],
                ),
                super::missing_error_message(),
            );
        }
    }

    mod fourth_image_missing {
        use super::super::super::{ImageFormat, LosslessImage};

        #[test]
        fn rgba8x4() {
            test_utils::assert_error_contains(
                LosslessImage::load(
                    ImageFormat::Rgba8x4,
                    &[
                        super::existing_rgba8_path(),
                        super::existing_rgba8_path(),
                        super::existing_rgba8_path(),
                        super::missing_image_path(),
                    ],
                ),
                super::missing_error_message(),
            );
        }
    }

    mod first_second_mismatch {
        use super::super::super::{ImageFormat, LosslessImage};

        #[test]
        fn rgba8x2() {
            test_utils::assert_error_contains(
                LosslessImage::load(
                    ImageFormat::Rgba8x2,
                    &[super::existing_rgba8_path(), super::large_rgba8_path()],
                ),
                super::mismatch_error_message(),
            );
        }

        #[test]
        fn rgba8x3() {
            test_utils::assert_error_contains(
                LosslessImage::load(
                    ImageFormat::Rgba8x3,
                    &[
                        super::existing_rgba8_path(),
                        super::large_rgba8_path(),
                        super::existing_rgba8_path(),
                    ],
                ),
                super::mismatch_error_message(),
            );
        }

        #[test]
        fn rgba8x4() {
            test_utils::assert_error_contains(
                LosslessImage::load(
                    ImageFormat::Rgba8x4,
                    &[
                        super::existing_rgba8_path(),
                        super::large_rgba8_path(),
                        super::existing_rgba8_path(),
                        super::existing_rgba8_path(),
                    ],
                ),
                super::mismatch_error_message(),
            );
        }

        #[test]
        fn rgba16x2() {
            test_utils::assert_error_contains(
                LosslessImage::load(
                    ImageFormat::Rgba16x2,
                    &[super::existing_rgba16_path(), super::large_rgba16_path()],
                ),
                super::mismatch_error_message(),
            );
        }
    }

    mod first_third_mismatch {
        use super::super::super::{ImageFormat, LosslessImage};

        #[test]
        fn rgba8x3() {
            test_utils::assert_error_contains(
                LosslessImage::load(
                    ImageFormat::Rgba8x3,
                    &[
                        super::existing_rgba8_path(),
                        super::existing_rgba8_path(),
                        super::large_rgba8_path(),
                    ],
                ),
                super::mismatch_error_message(),
            );
        }

        #[test]
        fn rgba8x4() {
            test_utils::assert_error_contains(
                LosslessImage::load(
                    ImageFormat::Rgba8x4,
                    &[
                        super::existing_rgba8_path(),
                        super::existing_rgba8_path(),
                        super::large_rgba8_path(),
                        super::existing_rgba8_path(),
                    ],
                ),
                super::mismatch_error_message(),
            );
        }
    }

    mod first_fourth_mismatch {
        use super::super::super::{ImageFormat, LosslessImage};

        #[test]
        fn rgba8x4() {
            test_utils::assert_error_contains(
                LosslessImage::load(
                    ImageFormat::Rgba8x4,
                    &[
                        super::existing_rgba8_path(),
                        super::existing_rgba8_path(),
                        super::existing_rgba8_path(),
                        super::large_rgba8_path(),
                    ],
                ),
                super::mismatch_error_message(),
            );
        }
    }

    mod first_image_save_error {
        #[test]
        fn rgba8x2() {
            test_utils::assert_error_contains(
                super::valid_rgba8x2().save_add_extension(&[
                    super::missing_directory_path(),
                    test_utils::make_test_output_path(&[
                        "compute_output_format_dynamic_io_first_image_save_error_rgba8x2_2",
                    ]),
                ]),
                super::missing_error_message(),
            );
            super::assert_not_files(&[test_utils::make_test_output_path(&[
                "compute_output_format_dynamic_io_first_image_save_error_rgba8x2_2.png",
            ])]);
        }

        #[test]
        fn rgba8x3() {
            test_utils::assert_error_contains(
                super::valid_rgba8x3().save_add_extension(&[
                    super::missing_directory_path(),
                    test_utils::make_test_output_path(&[
                        "compute_output_format_dynamic_io_first_image_save_error_rgba8x3_2",
                    ]),
                    test_utils::make_test_output_path(&[
                        "compute_output_format_dynamic_io_first_image_save_error_rgba8x3_3",
                    ]),
                ]),
                super::missing_error_message(),
            );
            super::assert_not_files(&[
                test_utils::make_test_output_path(&[
                    "compute_output_format_dynamic_io_first_image_save_error_rgba8x3_2.png",
                ]),
                test_utils::make_test_output_path(&[
                    "compute_output_format_dynamic_io_first_image_save_error_rgba8x3_3.png",
                ]),
            ]);
        }

        #[test]
        fn rgba8x4() {
            test_utils::assert_error_contains(
                super::valid_rgba8x4().save_add_extension(&[
                    super::missing_directory_path(),
                    test_utils::make_test_output_path(&[
                        "compute_output_format_dynamic_io_first_image_save_error_rgba8x4_2",
                    ]),
                    test_utils::make_test_output_path(&[
                        "compute_output_format_dynamic_io_first_image_save_error_rgba8x4_3",
                    ]),
                    test_utils::make_test_output_path(&[
                        "compute_output_format_dynamic_io_first_image_save_error_rgba8x4_4",
                    ]),
                ]),
                super::missing_error_message(),
            );
            super::assert_not_files(&[
                test_utils::make_test_output_path(&[
                    "compute_output_format_dynamic_io_first_image_save_error_rgba8x4_2.png",
                ]),
                test_utils::make_test_output_path(&[
                    "compute_output_format_dynamic_io_first_image_save_error_rgba8x4_3.png",
                ]),
                test_utils::make_test_output_path(&[
                    "compute_output_format_dynamic_io_first_image_save_error_rgba8x4_4.png",
                ]),
            ]);
        }

        #[test]
        fn rgba16x2() {
            test_utils::assert_error_contains(
                super::valid_rgba16x2().save_add_extension(&[
                    super::missing_directory_path(),
                    test_utils::make_test_output_path(&[
                        "compute_output_format_dynamic_io_first_image_save_error_rgba16x2_2",
                    ]),
                ]),
                super::missing_error_message(),
            );
            super::assert_not_files(&[test_utils::make_test_output_path(&[
                "compute_output_format_dynamic_io_first_image_save_error_rgba16x2_2.png",
            ])]);
        }
    }

    mod second_image_save_error {
        #[test]
        fn rgba8x2() {
            test_utils::assert_error_contains(
                super::valid_rgba8x2().save_add_extension(&[
                    test_utils::make_test_output_path(&[
                        "compute_output_format_dynamic_io_second_image_save_error_rgba8x2_1",
                    ]),
                    super::missing_directory_path(),
                ]),
                super::missing_error_message(),
            );
            super::assert_not_files(&[test_utils::make_test_output_path(&[
                "compute_output_format_dynamic_io_second_image_save_error_rgba8x2_1.png",
            ])]);
        }

        #[test]
        fn rgba8x3() {
            test_utils::assert_error_contains(
                super::valid_rgba8x3().save_add_extension(&[
                    test_utils::make_test_output_path(&[
                        "compute_output_format_dynamic_io_second_image_save_error_rgba8x3_1",
                    ]),
                    super::missing_directory_path(),
                    test_utils::make_test_output_path(&[
                        "compute_output_format_dynamic_io_second_image_save_error_rgba8x3_3",
                    ]),
                ]),
                super::missing_error_message(),
            );
            super::assert_not_files(&[
                test_utils::make_test_output_path(&[
                    "compute_output_format_dynamic_io_second_image_save_error_rgba8x3_1.png",
                ]),
                test_utils::make_test_output_path(&[
                    "compute_output_format_dynamic_io_second_image_save_error_rgba8x3_3.png",
                ]),
            ]);
        }

        #[test]
        fn rgba8x4() {
            test_utils::assert_error_contains(
                super::valid_rgba8x4().save_add_extension(&[
                    test_utils::make_test_output_path(&[
                        "compute_output_format_dynamic_io_second_image_save_error_rgba8x4_1",
                    ]),
                    super::missing_directory_path(),
                    test_utils::make_test_output_path(&[
                        "compute_output_format_dynamic_io_second_image_save_error_rgba8x4_3",
                    ]),
                    test_utils::make_test_output_path(&[
                        "compute_output_format_dynamic_io_second_image_save_error_rgba8x4_4",
                    ]),
                ]),
                super::missing_error_message(),
            );
            super::assert_not_files(&[
                test_utils::make_test_output_path(&[
                    "compute_output_format_dynamic_io_second_image_save_error_rgba8x4_1.png",
                ]),
                test_utils::make_test_output_path(&[
                    "compute_output_format_dynamic_io_second_image_save_error_rgba8x4_3.png",
                ]),
                test_utils::make_test_output_path(&[
                    "compute_output_format_dynamic_io_second_image_save_error_rgba8x4_4.png",
                ]),
            ]);
        }

        #[test]
        fn rgba16x2() {
            test_utils::assert_error_contains(
                super::valid_rgba16x2().save_add_extension(&[
                    test_utils::make_test_output_path(&[
                        "compute_output_format_dynamic_io_second_image_save_error_rgba16x2_1",
                    ]),
                    super::missing_directory_path(),
                ]),
                super::missing_error_message(),
            );
            super::assert_not_files(&[test_utils::make_test_output_path(&[
                "compute_output_format_dynamic_io_second_image_save_error_rgba16x2_1.png",
            ])]);
        }
    }

    mod third_image_save_error {
        #[test]
        fn rgba8x3() {
            test_utils::assert_error_contains(
                super::valid_rgba8x3().save_add_extension(&[
                    test_utils::make_test_output_path(&[
                        "compute_output_format_dynamic_io_third_image_save_error_rgba8x3_1",
                    ]),
                    test_utils::make_test_output_path(&[
                        "compute_output_format_dynamic_io_third_image_save_error_rgba8x3_2",
                    ]),
                    super::missing_directory_path(),
                ]),
                super::missing_error_message(),
            );
            super::assert_not_files(&[
                test_utils::make_test_output_path(&[
                    "compute_output_format_dynamic_io_third_image_save_error_rgba8x3_1.png",
                ]),
                test_utils::make_test_output_path(&[
                    "compute_output_format_dynamic_io_third_image_save_error_rgba8x3_2.png",
                ]),
            ]);
        }

        #[test]
        fn rgba8x4() {
            test_utils::assert_error_contains(
                super::valid_rgba8x4().save_add_extension(&[
                    test_utils::make_test_output_path(&[
                        "compute_output_format_dynamic_io_third_image_save_error_rgba8x4_1",
                    ]),
                    test_utils::make_test_output_path(&[
                        "compute_output_format_dynamic_io_third_image_save_error_rgba8x4_2",
                    ]),
                    super::missing_directory_path(),
                    test_utils::make_test_output_path(&[
                        "compute_output_format_dynamic_io_third_image_save_error_rgba8x4_4",
                    ]),
                ]),
                super::missing_error_message(),
            );
            super::assert_not_files(&[
                test_utils::make_test_output_path(&[
                    "compute_output_format_dynamic_io_third_image_save_error_rgba8x4_1.png",
                ]),
                test_utils::make_test_output_path(&[
                    "compute_output_format_dynamic_io_third_image_save_error_rgba8x4_2.png",
                ]),
                test_utils::make_test_output_path(&[
                    "compute_output_format_dynamic_io_third_image_save_error_rgba8x4_4.png",
                ]),
            ]);
        }
    }

    mod fourth_image_save_error {
        #[test]
        fn rgba8x4() {
            test_utils::assert_error_contains(
                super::valid_rgba8x4().save_add_extension(&[
                    test_utils::make_test_output_path(&[
                        "compute_output_format_dynamic_io_fourth_image_save_error_rgba8x4_1",
                    ]),
                    test_utils::make_test_output_path(&[
                        "compute_output_format_dynamic_io_fourth_image_save_error_rgba8x4_2",
                    ]),
                    test_utils::make_test_output_path(&[
                        "compute_output_format_dynamic_io_fourth_image_save_error_rgba8x4_3",
                    ]),
                    super::missing_directory_path(),
                ]),
                super::missing_error_message(),
            );
            super::assert_not_files(&[
                test_utils::make_test_output_path(&[
                    "compute_output_format_dynamic_io_fourth_image_save_error_rgba8x4_1.png",
                ]),
                test_utils::make_test_output_path(&[
                    "compute_output_format_dynamic_io_fourth_image_save_error_rgba8x4_2.png",
                ]),
                test_utils::make_test_output_path(&[
                    "compute_output_format_dynamic_io_fourth_image_save_error_rgba8x4_3.png",
                ]),
            ]);
        }
    }
}
