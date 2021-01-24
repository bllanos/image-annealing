use super::ImageDimensions;
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
