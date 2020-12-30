mod prepare_image {
    use crate::gpu_prototype::prepare_image;
    use crate::image_utils::rectangle::Rectangle;
    use image::DynamicImage;
    use image::GenericImageView;
    use std::error::Error;

    #[test]
    fn no_region() -> Result<(), Box<dyn Error>> {
        let img = DynamicImage::new_bgr8(100, 200);
        let img_converted = img.to_rgba16();
        let img_out = prepare_image(&img, None)?;
        assert_eq!(img.dimensions(), img_out.dimensions());
        assert_eq!(img_converted, img_out);
        Ok(())
    }

    #[test]
    fn good_crop_region() -> Result<(), Box<dyn Error>> {
        let img = DynamicImage::new_bgr8(100, 200);
        let rect = Rectangle::from_corners(20, 40, 60, 100)?;
        let img_out = prepare_image(&img, Some(&rect))?;
        assert_eq!(img_out.width(), rect.width());
        assert_eq!(img_out.height(), rect.height());
        Ok(())
    }

    #[test]
    fn bad_crop_region() -> Result<(), Box<dyn Error>> {
        let img = DynamicImage::new_bgr8(100, 200);
        let rect = Rectangle::from_corners(20, 40, 60, 201)?;
        let r = prepare_image(&img, Some(&rect));
        r.expect_err("An invalid crop region should trigger an error");
        Ok(())
    }
}

mod process_image {
    use crate::gpu_prototype::process_image;
    use crate::gpu_prototype::TextureImageBuffer;
    use std::error::Error;

    #[test]
    fn runs_without_error() -> Result<(), Box<dyn Error>> {
        let img = TextureImageBuffer::new(40, 45);
        let img_out = process_image(&img)?;
        assert_eq!(img_out.width(), img.width());
        assert_eq!(img_out.height(), img.height());
        Ok(())
    }
}
