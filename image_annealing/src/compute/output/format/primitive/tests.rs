mod vector_field_image_buffer {
    use super::super::super::{ImageFileReader, ImageFormat};
    use super::super::VectorFieldImageBuffer;

    #[test]
    fn load_unexpected_format() {
        let path = test_utils::make_test_data_path(&["image", "image", "red.png"]);

        test_utils::assert_error_contains(
            VectorFieldImageBuffer::load(path),
            &format!("not the expected format of {}", ImageFormat::Rgba8),
        );
    }
}

mod rgba16_image_buffer {
    use super::super::super::{ImageFileReader, ImageFormat};
    use super::super::Rgba16ImageBuffer;

    #[test]
    fn load_unexpected_format() {
        let path = test_utils::make_test_data_path(&["image", "image", "stripes.png"]);

        test_utils::assert_error_contains(
            Rgba16ImageBuffer::load(path),
            &format!("not the expected format of {}", ImageFormat::Rgba16),
        );
    }
}
