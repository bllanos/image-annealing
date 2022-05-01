mod new {
    use image::Rgba;

    fn zero_width_rgba8() -> image::RgbaImage {
        image::RgbaImage::from_pixel(0, 1, Rgba([0, 0, 0, 0]))
    }

    fn valid_rgba8() -> image::RgbaImage {
        image::RgbaImage::from_pixel(2, 1, Rgba([0, 0, 0, 0]))
    }

    fn large_rgba8() -> image::RgbaImage {
        image::RgbaImage::from_pixel(2, 3, Rgba([0, 0, 0, 0]))
    }

    fn mismatch_error_message() -> &'static str {
        "mismatch in image dimensions, (width, height) = (2, 1) and (width, height) = (2, 3)"
    }

    mod first_image_error {
        use super::super::super::{Rgba8x2Image, Rgba8x3Image};

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
    }

    mod second_image_error {
        use super::super::super::{Rgba8x2Image, Rgba8x3Image};

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
    }

    mod third_image_error {
        use super::super::super::Rgba8x3Image;

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
    }

    mod first_second_mismatch {
        use super::super::super::{Rgba8x2Image, Rgba8x3Image};

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
    }

    mod first_third_mismatch {
        use super::super::super::Rgba8x3Image;

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
    }
}
