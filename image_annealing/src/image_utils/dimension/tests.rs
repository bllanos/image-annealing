use super::ImageDimensions;
use image::RgbaImage;
use std::error::Error;

#[test]
fn zero_width() {
    let r = ImageDimensions::new(0, 1);
    r.expect_err("An error should be raised if the width is zero");
}

#[test]
fn zero_height() {
    let r = ImageDimensions::new(1, 0);
    r.expect_err("An error should be raised if the height is zero");
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
    assert_eq!(dim.width(), image.width() as usize);
    assert_eq!(dim.height(), image.height() as usize);
    Ok(())
}

mod make_linear_index {
    use super::super::ImageDimensions;
    use std::error::Error;

    #[test]
    fn negative_x() -> Result<(), Box<dyn Error>> {
        let dim = ImageDimensions::new(3, 5)?;
        let r = dim.make_linear_index(-1, 0);
        r.expect_err("An error should be raised if the x-coordinate is negative");
        Ok(())
    }

    #[test]
    fn negative_y() -> Result<(), Box<dyn Error>> {
        let dim = ImageDimensions::new(3, 5)?;
        let r = dim.make_linear_index(0, -1);
        r.expect_err("An error should be raised if the y-coordinate is negative");
        Ok(())
    }

    #[test]
    fn large_x() -> Result<(), Box<dyn Error>> {
        let dim = ImageDimensions::new(3, 5)?;
        let r = dim.make_linear_index(3, 0);
        r.expect_err("An error should be raised if the x-coordinate is too large");
        Ok(())
    }

    #[test]
    fn large_y() -> Result<(), Box<dyn Error>> {
        let dim = ImageDimensions::new(3, 5)?;
        let r = dim.make_linear_index(0, 5);
        r.expect_err("An error should be raised if the y-coordinate is too large");
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
