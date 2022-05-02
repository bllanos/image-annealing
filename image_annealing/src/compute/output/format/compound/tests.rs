mod new {
    use super::super::super::Rgba16ImageBuffer;
    use image::Rgba;

    fn zero_width_rgba8() -> image::RgbaImage {
        image::RgbaImage::from_pixel(0, 1, Rgba([0, 0, 0, 0]))
    }

    fn zero_width_rgba16() -> Rgba16ImageBuffer {
        Rgba16ImageBuffer::from_pixel(0, 1, Rgba([0, 0, 0, 0]))
    }

    fn valid_rgba8() -> image::RgbaImage {
        image::RgbaImage::from_pixel(2, 1, Rgba([0, 0, 0, 0]))
    }

    fn valid_rgba16() -> Rgba16ImageBuffer {
        Rgba16ImageBuffer::from_pixel(2, 1, Rgba([0, 0, 0, 0]))
    }

    fn large_rgba8() -> image::RgbaImage {
        image::RgbaImage::from_pixel(2, 3, Rgba([0, 0, 0, 0]))
    }

    fn large_rgba16() -> Rgba16ImageBuffer {
        Rgba16ImageBuffer::from_pixel(2, 3, Rgba([0, 0, 0, 0]))
    }

    fn mismatch_error_message() -> &'static str {
        "mismatch in image dimensions, (width, height) = (2, 1) and (width, height) = (2, 3)"
    }

    mod first_image_error {
        use super::super::super::{Rgba16x2Image, Rgba8x2Image, Rgba8x3Image, Rgba8x4Image};

        #[test]
        fn rgba8x2() {
            test_utils::assert_error_contains(
                Rgba8x2Image::new(super::zero_width_rgba8(), super::valid_rgba8()),
                "width is zero",
            );
        }

        #[test]
        fn rgba8x3() {
            test_utils::assert_error_contains(
                Rgba8x3Image::new(
                    super::zero_width_rgba8(),
                    super::valid_rgba8(),
                    super::valid_rgba8(),
                ),
                "width is zero",
            );
        }

        #[test]
        fn rgba8x4() {
            test_utils::assert_error_contains(
                Rgba8x4Image::new(
                    super::zero_width_rgba8(),
                    super::valid_rgba8(),
                    super::valid_rgba8(),
                    super::valid_rgba8(),
                ),
                "width is zero",
            );
        }

        #[test]
        fn rgba16x2() {
            test_utils::assert_error_contains(
                Rgba16x2Image::new(super::zero_width_rgba16(), super::valid_rgba16()),
                "width is zero",
            );
        }
    }

    mod second_image_error {
        use super::super::super::{Rgba16x2Image, Rgba8x2Image, Rgba8x3Image, Rgba8x4Image};

        #[test]
        fn rgba8x2() {
            test_utils::assert_error_contains(
                Rgba8x2Image::new(super::valid_rgba8(), super::zero_width_rgba8()),
                "width is zero",
            );
        }

        #[test]
        fn rgba8x3() {
            test_utils::assert_error_contains(
                Rgba8x3Image::new(
                    super::valid_rgba8(),
                    super::zero_width_rgba8(),
                    super::valid_rgba8(),
                ),
                "width is zero",
            );
        }

        #[test]
        fn rgba8x4() {
            test_utils::assert_error_contains(
                Rgba8x4Image::new(
                    super::valid_rgba8(),
                    super::zero_width_rgba8(),
                    super::valid_rgba8(),
                    super::valid_rgba8(),
                ),
                "width is zero",
            );
        }

        #[test]
        fn rgba16x2() {
            test_utils::assert_error_contains(
                Rgba16x2Image::new(super::valid_rgba16(), super::zero_width_rgba16()),
                "width is zero",
            );
        }
    }

    mod third_image_error {
        use super::super::super::{Rgba8x3Image, Rgba8x4Image};

        #[test]
        fn rgba8x3() {
            test_utils::assert_error_contains(
                Rgba8x3Image::new(
                    super::valid_rgba8(),
                    super::valid_rgba8(),
                    super::zero_width_rgba8(),
                ),
                "width is zero",
            );
        }

        #[test]
        fn rgba8x4() {
            test_utils::assert_error_contains(
                Rgba8x4Image::new(
                    super::valid_rgba8(),
                    super::valid_rgba8(),
                    super::zero_width_rgba8(),
                    super::valid_rgba8(),
                ),
                "width is zero",
            );
        }
    }

    mod fourth_image_error {
        use super::super::super::Rgba8x4Image;

        #[test]
        fn rgba8x4() {
            test_utils::assert_error_contains(
                Rgba8x4Image::new(
                    super::valid_rgba8(),
                    super::valid_rgba8(),
                    super::valid_rgba8(),
                    super::zero_width_rgba8(),
                ),
                "width is zero",
            );
        }
    }

    mod first_second_mismatch {
        use super::super::super::{Rgba16x2Image, Rgba8x2Image, Rgba8x3Image, Rgba8x4Image};

        #[test]
        fn rgba8x2() {
            test_utils::assert_error_contains(
                Rgba8x2Image::new(super::valid_rgba8(), super::large_rgba8()),
                super::mismatch_error_message(),
            );
        }

        #[test]
        fn rgba8x3() {
            test_utils::assert_error_contains(
                Rgba8x3Image::new(
                    super::valid_rgba8(),
                    super::large_rgba8(),
                    super::valid_rgba8(),
                ),
                super::mismatch_error_message(),
            );
        }

        #[test]
        fn rgba8x4() {
            test_utils::assert_error_contains(
                Rgba8x4Image::new(
                    super::valid_rgba8(),
                    super::large_rgba8(),
                    super::valid_rgba8(),
                    super::valid_rgba8(),
                ),
                super::mismatch_error_message(),
            );
        }

        #[test]
        fn rgba16x2() {
            test_utils::assert_error_contains(
                Rgba16x2Image::new(super::valid_rgba16(), super::large_rgba16()),
                super::mismatch_error_message(),
            );
        }
    }

    mod first_third_mismatch {
        use super::super::super::{Rgba8x3Image, Rgba8x4Image};

        #[test]
        fn rgba8x3() {
            test_utils::assert_error_contains(
                Rgba8x3Image::new(
                    super::valid_rgba8(),
                    super::valid_rgba8(),
                    super::large_rgba8(),
                ),
                super::mismatch_error_message(),
            );
        }

        #[test]
        fn rgba8x4() {
            test_utils::assert_error_contains(
                Rgba8x4Image::new(
                    super::valid_rgba8(),
                    super::valid_rgba8(),
                    super::large_rgba8(),
                    super::valid_rgba8(),
                ),
                super::mismatch_error_message(),
            );
        }
    }

    mod first_fourth_mismatch {
        use super::super::super::Rgba8x4Image;

        #[test]
        fn rgba8x4() {
            test_utils::assert_error_contains(
                Rgba8x4Image::new(
                    super::valid_rgba8(),
                    super::valid_rgba8(),
                    super::valid_rgba8(),
                    super::large_rgba8(),
                ),
                super::mismatch_error_message(),
            );
        }
    }
}
