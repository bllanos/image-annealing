use super::ImageDimensions;
use image::RgbaImage;
use std::convert::TryInto;
use std::error::Error;

#[test]
fn negative_width() {
    test_utils::assert_error_contains(
        ImageDimensions::new(-1, 1),
        "failed to convert -1 to the required type for dimensions",
    )
}

#[test]
fn negative_height() {
    test_utils::assert_error_contains(
        ImageDimensions::new(1, -1),
        "failed to convert -1 to the required type for dimensions",
    )
}

#[test]
fn zero_width() {
    test_utils::assert_error_contains(ImageDimensions::new(0, 1), "width is zero")
}

#[test]
fn zero_height() {
    test_utils::assert_error_contains(ImageDimensions::new(1, 0), "height is zero")
}

#[test]
fn valid_dimensions() -> Result<(), Box<dyn Error>> {
    let dim = ImageDimensions::new(63, 64)?;
    assert_eq!(dim.width(), 63);
    assert_eq!(dim.height(), 64);
    assert_eq!(dim.count(), 63 * 64);
    Ok(())
}

#[test]
fn from_image() -> Result<(), Box<dyn Error>> {
    let image = RgbaImage::new(4, 5);
    let dim = ImageDimensions::from_image(&image)?;
    assert_eq!(dim.width(), image.width().try_into().unwrap());
    assert_eq!(dim.height(), image.height().try_into().unwrap());
    Ok(())
}

mod make_linear_index {
    use super::super::ImageDimensions;
    use std::error::Error;

    #[test]
    fn negative_x() -> Result<(), Box<dyn Error>> {
        let dim = ImageDimensions::new(3, 5)?;
        test_utils::assert_error_contains(
            dim.make_linear_index(-1, 0),
            "failed to convert -1 to the required type for coordinates",
        );
        Ok(())
    }

    #[test]
    fn negative_y() -> Result<(), Box<dyn Error>> {
        let dim = ImageDimensions::new(3, 5)?;
        test_utils::assert_error_contains(
            dim.make_linear_index(0, -1),
            "failed to convert -1 to the required type for coordinates",
        );
        Ok(())
    }

    #[test]
    fn large_x() -> Result<(), Box<dyn Error>> {
        let dim = ImageDimensions::new(3, 5)?;
        test_utils::assert_error_contains(
            dim.make_linear_index(3, 0),
            "x = 3 is out of bounds (width, height) = (3, 5)",
        );
        Ok(())
    }

    #[test]
    fn large_y() -> Result<(), Box<dyn Error>> {
        let dim = ImageDimensions::new(3, 5)?;
        test_utils::assert_error_contains(
            dim.make_linear_index(0, 5),
            "y = 5 is out of bounds (width, height) = (3, 5)",
        );
        Ok(())
    }

    #[test]
    fn valid_coordinates() -> Result<(), Box<dyn Error>> {
        let dim = ImageDimensions::new(3, 5)?;
        let mut i = dim.make_linear_index(0, 0)?;
        assert_eq!(i, 0);
        i = dim.make_linear_index(1, 0)?;
        assert_eq!(i, 1);
        i = dim.make_linear_index(0, 1)?;
        assert_eq!(i, 3);
        i = dim.make_linear_index(1, 1)?;
        assert_eq!(i, 4);
        i = dim.make_linear_index(2, 4)?;
        assert_eq!(i, 3 * 5 - 1);
        Ok(())
    }
}
