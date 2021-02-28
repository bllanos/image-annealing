use super::validate_permutation;
use crate::compute::conversion::{self, PermutationEntry};
use crate::image_utils::ImageDimensions;
use std::error::Error;

#[test]
fn identity() -> Result<(), Box<dyn Error>> {
    let dim = ImageDimensions::new(3, 5)?;
    let v = vec![PermutationEntry(0, 0); dim.count()];
    let p = conversion::as_image(&dim, &v);
    let expected = p.clone();
    let q = validate_permutation(p)?;
    assert_eq!(*q, expected);
    Ok(())
}

#[test]
fn out_of_bounds_right() -> Result<(), Box<dyn Error>> {
    let image = conversion::as_image(
        &ImageDimensions::new(1, 3)?,
        &vec![
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
        &vec![
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
        &vec![
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
        &vec![
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
    let image = conversion::as_image(
        &ImageDimensions::new(1, 3)?,
        &vec![
            PermutationEntry(0, 1),
            PermutationEntry(0, 1),
            PermutationEntry(0, -1),
        ],
    );
    let r = validate_permutation(image);
    match r {
        Ok(_) => panic!("An error should be raised for a conflicting mapping"),
        Err(_) => Ok(()),
    }
}

#[test]
fn non_identity() -> Result<(), Box<dyn Error>> {
    let image = conversion::as_image(
        &ImageDimensions::new(2, 3)?,
        &vec![
            PermutationEntry(0, 1),
            PermutationEntry(0, 0),
            PermutationEntry(0, -1),
            PermutationEntry(-1, 1),
            PermutationEntry(1, 0),
            PermutationEntry(0, -1),
        ],
    );
    let expected = image.clone();
    let validated = validate_permutation(image)?;
    assert_eq!(*validated, expected);
    Ok(())
}
