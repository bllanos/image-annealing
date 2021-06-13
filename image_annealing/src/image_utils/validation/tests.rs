use super::validate_permutation;
use crate::compute::conversion::{self, PermutationEntry};
use crate::image_utils::ImageDimensions;
use std::error::Error;
use test_utils::permutation::{self, DimensionsAndPermutation};

#[test]
fn identity() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation { permutation, .. } = permutation::identity();
    let expected = permutation.clone();
    let permutation = validate_permutation(permutation)?;
    assert_eq!(*permutation.as_ref(), expected);
    Ok(())
}

#[test]
fn out_of_bounds_right() -> Result<(), Box<dyn Error>> {
    let permutation = conversion::to_image(
        &ImageDimensions::new(1, 3)?,
        &[
            PermutationEntry(0, 0),
            PermutationEntry(0, 1),
            PermutationEntry(1, -1),
        ],
    );
    let r = validate_permutation(permutation);
    match r {
        Ok(_) => panic!("An error should be raised for an out-of-bounds mapping"),
        Err(_) => Ok(()),
    }
}

#[test]
fn out_of_bounds_up() -> Result<(), Box<dyn Error>> {
    let permutation = conversion::to_image(
        &ImageDimensions::new(1, 3)?,
        &[
            PermutationEntry(0, -1),
            PermutationEntry(0, 1),
            PermutationEntry(0, -1),
        ],
    );
    let r = validate_permutation(permutation);
    match r {
        Ok(_) => panic!("An error should be raised for an out-of-bounds mapping"),
        Err(_) => Ok(()),
    }
}

#[test]
fn out_of_bounds_left() -> Result<(), Box<dyn Error>> {
    let permutation = conversion::to_image(
        &ImageDimensions::new(1, 3)?,
        &[
            PermutationEntry(0, 0),
            PermutationEntry(-2, 1),
            PermutationEntry(0, -1),
        ],
    );
    let r = validate_permutation(permutation);
    match r {
        Ok(_) => panic!("An error should be raised for an out-of-bounds mapping"),
        Err(_) => Ok(()),
    }
}

#[test]
fn out_of_bounds_down() -> Result<(), Box<dyn Error>> {
    let permutation = conversion::to_image(
        &ImageDimensions::new(1, 3)?,
        &[
            PermutationEntry(0, 3),
            PermutationEntry(0, 1),
            PermutationEntry(0, -1),
        ],
    );
    let r = validate_permutation(permutation);
    match r {
        Ok(_) => panic!("An error should be raised for an out-of-bounds mapping"),
        Err(_) => Ok(()),
    }
}

#[test]
fn duplicate() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation { permutation, .. } = permutation::duplicate();
    let r = validate_permutation(permutation);
    match r {
        Ok(_) => panic!("An error should be raised for a conflicting mapping"),
        Err(_) => Ok(()),
    }
}

#[test]
fn non_identity() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation { permutation, .. } = permutation::non_identity();
    let expected = permutation.clone();
    let permutation = validate_permutation(permutation)?;
    assert_eq!(*permutation.as_ref(), expected);
    Ok(())
}
