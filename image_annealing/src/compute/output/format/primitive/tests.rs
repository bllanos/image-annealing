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

    mod byte_manipulation {
        use super::super::super::{
            Rgba16Image, Rgba16ImageBuffer, Rgba16ImageBufferComponent, Rgba8Image,
        };

        fn make_first_vec() -> Vec<u8> {
            vec![
                1, 2, 3, 4, 255, 254, 253, 252, 5, 6, 7, 8, 251, 250, 249, 248, 9, 10, 11, 12, 247,
                246, 245, 244,
            ]
        }

        fn make_second_vec() -> Vec<u8> {
            vec![
                13, 14, 15, 16, 243, 242, 241, 240, 17, 18, 19, 20, 239, 238, 237, 236, 21, 22, 23,
                24, 235, 234, 233, 232,
            ]
        }

        fn make_first_image() -> Rgba8Image {
            Rgba8Image::new(image::RgbaImage::from_vec(2, 3, make_first_vec()).unwrap()).unwrap()
        }

        fn make_second_image() -> Rgba8Image {
            Rgba8Image::new(image::RgbaImage::from_vec(2, 3, make_second_vec()).unwrap()).unwrap()
        }

        fn make_paired_image() -> Rgba16Image {
            Rgba16Image::new(
                Rgba16ImageBuffer::from_vec(
                    2,
                    3,
                    make_first_vec()
                        .into_iter()
                        .zip(make_second_vec().into_iter())
                        .map(|(component1, component2)| {
                            Rgba16ImageBufferComponent::from_ne_bytes([component1, component2])
                        })
                        .collect(),
                )
                .unwrap(),
            )
            .unwrap()
        }

        mod from_pair {
            use super::super::super::super::{Rgba16Image, Rgba8Image};
            use std::error::Error;

            #[test]
            fn dimensions_mismatch() -> Result<(), Box<dyn Error>> {
                let larger_image =
                    Rgba8Image::new(image::RgbaImage::from_pixel(2, 4, image::Rgba([1; 4])))?;

                test_util::assert_error_contains(
                    Rgba16Image::from_pair(super::make_first_image(), larger_image),
                    "mismatch in image dimensions, (width, height) = (2, 3) and (width, height) = (2, 4)",
                );
                Ok(())
            }

            #[test]
            fn from_pair() -> Result<(), Box<dyn Error>> {
                assert_eq!(
                    Rgba16Image::from_pair(super::make_first_image(), super::make_second_image())?,
                    super::make_paired_image()
                );
                Ok(())
            }
        }

        mod from_raw_pair {
            use super::super::super::super::Rgba16Image;
            use std::error::Error;

            #[test]
            fn dimensions_mismatch() {
                let larger_image = image::RgbaImage::from_pixel(2, 4, image::Rgba([1; 4]));

                test_util::assert_error_contains(
                    Rgba16Image::from_raw_pair(super::make_first_image().into_inner(), larger_image),
                    "mismatch in image dimensions, (width, height) = (2, 3) and (width, height) = (2, 4)",
                );
            }

            #[test]
            fn from_raw_pair() -> Result<(), Box<dyn Error>> {
                assert_eq!(
                    Rgba16Image::from_raw_pair(
                        super::make_first_image().into_inner(),
                        super::make_second_image().into_inner()
                    )?,
                    super::make_paired_image().into_inner()
                );
                Ok(())
            }
        }

        mod clone_byte {
            #[test]
            fn first_byte() {
                let image = super::make_paired_image();
                let expected = super::make_first_image();
                assert_eq!(image.clone_byte(0), expected);
                assert_eq!(image.clone_first_byte(), expected);
            }

            #[test]
            fn second_byte() {
                let image = super::make_paired_image();
                let expected = super::make_second_image();
                assert_eq!(image.clone_byte(1), expected);
                assert_eq!(image.clone_second_byte(), expected);
            }

            #[test]
            #[should_panic(expected = "index out of bounds: the len is 2 but the index is 2")]
            fn byte_index_out_of_bounds() {
                super::make_paired_image().clone_byte(2);
            }
        }
    }
}
