use super::test_utils::{self, DimensionsAndImage};
use super::validate_permutation;
use crate::compute::conversion::{self, PermutationEntry};
use crate::image_utils::ImageDimensions;
use std::error::Error;

#[test]
fn identity() -> Result<(), Box<dyn Error>> {
    let DimensionsAndImage { image, .. } = test_utils::identity();
    let expected = image.clone();
    let image = validate_permutation(image)?;
    assert_eq!(*image, expected);
    Ok(())
}

#[test]
fn out_of_bounds_right() -> Result<(), Box<dyn Error>> {
    let image = conversion::as_image(
        &ImageDimensions::new(1, 3)?,
        &[
            PermutationEntry(0, 0),
            PermutationEntry(0, 1),
            PermutationEntry(1, -1),
        ],
    );
    let r = validate_permutation(image);
    match r {
        Ok(_) => panic!("An error should be raised for an out-of-bounds mapping"),
        Err(_) => Ok(()),
    }
}

#[test]
fn out_of_bounds_up() -> Result<(), Box<dyn Error>> {
    let image = conversion::as_image(
        &ImageDimensions::new(1, 3)?,
        &[
            PermutationEntry(0, -1),
            PermutationEntry(0, 1),
            PermutationEntry(0, -1),
        ],
    );
    let r = validate_permutation(image);
    match r {
        Ok(_) => panic!("An error should be raised for an out-of-bounds mapping"),
        Err(_) => Ok(()),
    }
}

#[test]
fn out_of_bounds_left() -> Result<(), Box<dyn Error>> {
    let image = conversion::as_image(
        &ImageDimensions::new(1, 3)?,
        &[
            PermutationEntry(0, 0),
            PermutationEntry(-2, 1),
            PermutationEntry(0, -1),
        ],
    );
    let r = validate_permutation(image);
    match r {
        Ok(_) => panic!("An error should be raised for an out-of-bounds mapping"),
        Err(_) => Ok(()),
    }
}

#[test]
fn out_of_bounds_down() -> Result<(), Box<dyn Error>> {
    let image = conversion::as_image(
        &ImageDimensions::new(1, 3)?,
        &[
            PermutationEntry(0, 3),
            PermutationEntry(0, 1),
            PermutationEntry(0, -1),
        ],
    );
    let r = validate_permutation(image);
    match r {
        Ok(_) => panic!("An error should be raised for an out-of-bounds mapping"),
        Err(_) => Ok(()),
    }
}

#[test]
fn duplicate() -> Result<(), Box<dyn Error>> {
    let DimensionsAndImage { image, .. } = test_utils::duplicate();
    let r = validate_permutation(image);
    match r {
        Ok(_) => panic!("An error should be raised for a conflicting mapping"),
        Err(_) => Ok(()),
    }
}

#[test]
fn non_identity() -> Result<(), Box<dyn Error>> {
    let DimensionsAndImage { image, .. } = test_utils::non_identity();
    let expected = image.clone();
    let image = validate_permutation(image)?;
    assert_eq!(*image, expected);
    Ok(())
}
