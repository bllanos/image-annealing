use image::{ImageBuffer, Rgba};

fn make_image_buffer<T: TryFrom<u32> + 'static>(
) -> ImageBuffer<image::Rgba<T>, Vec<<image::Rgba<T> as image::Pixel>::Subpixel>>
where
    <T as TryFrom<u32>>::Error: std::fmt::Debug,
    image::Rgba<T>: image::Pixel,
{
    ImageBuffer::from_fn(2, 3, |x, y| {
        Rgba([
            x.try_into().unwrap(),
            (x + 1).try_into().unwrap(),
            y.try_into().unwrap(),
            (y + 1).try_into().unwrap(),
        ])
    })
}

mod vector_field_image_buffer {
    use super::super::super::{ImageFileReader, ImageFormat};
    use super::super::VectorFieldImageBuffer;

    #[test]
    fn load_unexpected_format() {
        let path = test_util::make_test_data_path(["image", "image", "red.png"]);

        test_util::assert_error_contains(
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
        let path = test_util::make_test_data_path(["image", "image", "stripes.png"]);

        test_util::assert_error_contains(
            Rgba16ImageBuffer::load(path),
            &format!("not the expected format of {}", ImageFormat::Rgba16),
        );
    }
}

mod identity {
    use crate::ImageDimensions;
    use std::error::Error;

    #[test]
    fn identity() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::try_new(2, 3)?;
        let image = super::super::identity(&dimensions);
        assert_eq!(ImageDimensions::from_image(&image)?, dimensions);
        assert!(super::super::is_identity(&image));
        Ok(())
    }
}

mod is_identity {
    use crate::ImageDimensions;
    use std::error::Error;

    #[test]
    fn identity() -> Result<(), Box<dyn Error>> {
        assert!(super::super::is_identity(&super::super::identity(
            &ImageDimensions::try_new(2, 3)?
        )));
        Ok(())
    }

    #[test]
    fn non_identity() {
        let mut image = image::RgbaImage::from_pixel(2, 3, image::Rgba([0; 4]));
        image.put_pixel(1, 2, image::Rgba([0, 0, 0, 1]));
        assert!(!super::super::is_identity(&image));
    }
}

mod rgba8_image {
    use super::super::Rgba8Image;
    use std::error::Error;

    #[test]
    fn into_inner() -> Result<(), Box<dyn Error>> {
        let image = super::make_image_buffer();
        let expected = image.clone();
        let wrapped_image = Rgba8Image::new(image)?;
        assert_eq!(wrapped_image.into_inner(), expected);
        Ok(())
    }
}

mod rgba16_image {
    use super::super::Rgba16Image;
    use std::error::Error;

    #[test]
    fn into_inner() -> Result<(), Box<dyn Error>> {
        let image = super::make_image_buffer();
        let expected = image.clone();
        let wrapped_image = Rgba16Image::new(image)?;
        assert_eq!(wrapped_image.into_inner(), expected);
        Ok(())
    }
}
