use super::super::validation;
use super::invert_permutation;
use crate::compute::conversion::{self, VectorFieldEntry};
use crate::ImageDimensions;
use std::error::Error;
use test_utils::permutation::{self, DimensionsAndPermutation};

#[test]
fn identity() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation { permutation, .. } = permutation::identity();
    let expected = permutation.clone();
    let inverse = invert_permutation(&validation::validate_permutation(permutation)?);
    assert_eq!(inverse, expected);
    Ok(())
}

#[test]
fn non_identity() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation { permutation, .. } = permutation::non_identity();
    let v = vec![
        VectorFieldEntry(0, 1),
        VectorFieldEntry(0, 0),
        VectorFieldEntry(0, -1),
        VectorFieldEntry(0, 1),
        VectorFieldEntry(1, -1),
        VectorFieldEntry(-1, 0),
    ];
    let dimensions = ImageDimensions::try_new(2, 3).unwrap();
    let expected = conversion::to_image(&dimensions, &v);
    let inverse = invert_permutation(&validation::validate_permutation(permutation)?);
    assert_eq!(inverse, expected);
    Ok(())
}
