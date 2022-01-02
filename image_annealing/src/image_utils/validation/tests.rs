use super::validate_permutation;
use crate::compute::conversion::{self, VectorFieldEntry};
use crate::ImageDimensions;
use std::error::Error;
use test_utils::permutation::{self, DimensionsAndPermutation};

mod validated_permutation {
    use super::super::manipulation;
    use std::error::Error;
    use test_utils::permutation::{self, DimensionsAndPermutation};

    #[test]
    fn inverse() -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation { permutation, .. } = permutation::non_identity();
        let validated_permutation = super::super::validate_permutation(permutation)?;
        let expected = manipulation::invert_permutation(&validated_permutation);
        let inverse = validated_permutation.inverse();
        assert_eq!(*inverse.as_ref(), expected);
        Ok(())
    }
}

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
            VectorFieldEntry(0, 0),
            VectorFieldEntry(0, 1),
            VectorFieldEntry(1, -1),
        ],
    );
    test_utils::assert_error_contains(
        validate_permutation(permutation),
        "out of bounds mapping (x, y, delta_x, delta_y) = (0, 2, 1, -1) for an image of dimensions (width, height) = (1, 3)",
    );
    Ok(())
}

#[test]
fn out_of_bounds_up() -> Result<(), Box<dyn Error>> {
    let permutation = conversion::to_image(
        &ImageDimensions::new(1, 3)?,
        &[
            VectorFieldEntry(0, -1),
            VectorFieldEntry(0, 1),
            VectorFieldEntry(0, -1),
        ],
    );
    test_utils::assert_error_contains(
        validate_permutation(permutation),
        "out of bounds mapping (x, y, delta_x, delta_y) = (0, 0, 0, -1) for an image of dimensions (width, height) = (1, 3)",
    );
    Ok(())
}

#[test]
fn out_of_bounds_left() -> Result<(), Box<dyn Error>> {
    let permutation = conversion::to_image(
        &ImageDimensions::new(1, 3)?,
        &[
            VectorFieldEntry(0, 0),
            VectorFieldEntry(-2, 1),
            VectorFieldEntry(0, -1),
        ],
    );
    test_utils::assert_error_contains(
        validate_permutation(permutation),
        "out of bounds mapping (x, y, delta_x, delta_y) = (0, 1, -2, 1) for an image of dimensions (width, height) = (1, 3)",
    );
    Ok(())
}

#[test]
fn out_of_bounds_down() -> Result<(), Box<dyn Error>> {
    let permutation = conversion::to_image(
        &ImageDimensions::new(1, 3)?,
        &[
            VectorFieldEntry(0, 3),
            VectorFieldEntry(0, 1),
            VectorFieldEntry(0, -1),
        ],
    );
    test_utils::assert_error_contains(
        validate_permutation(permutation),
        "out of bounds mapping (x, y, delta_x, delta_y) = (0, 0, 0, 3) for an image of dimensions (width, height) = (1, 3)",
    );
    Ok(())
}

#[test]
fn duplicate() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation { permutation, .. } = permutation::duplicate();
    test_utils::assert_error_contains(
        validate_permutation(permutation),
        "entries (x, y, delta_x, delta_y) = (0, 0, 0, 1) and (x, y, delta_x, delta_y) = (0, 2, 0, -1) both map to location (x, y) = (0, 1)",
    );
    Ok(())
}

#[test]
fn non_identity() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation { permutation, .. } = permutation::non_identity();
    let expected = permutation.clone();
    let permutation = validate_permutation(permutation)?;
    assert_eq!(*permutation.as_ref(), expected);
    Ok(())
}
