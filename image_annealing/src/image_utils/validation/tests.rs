mod validated_permutation {
    use super::super::manipulation;
    use crate::{ImageDimensions, ImageDimensionsHolder};
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

    #[test]
    fn as_raw_slice() -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation { permutation, .. } = permutation::non_identity();
        let expected = permutation.clone();
        let validated_permutation = super::super::validate_permutation(permutation)?;
        assert_eq!(
            validated_permutation.as_raw_slice(),
            expected.as_raw().as_slice()
        );
        Ok(())
    }

    #[test]
    fn into_inner() -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation { permutation, .. } = permutation::non_identity();
        let expected = permutation.clone();
        let validated_permutation = super::super::validate_permutation(permutation)?;
        assert_eq!(validated_permutation.into_inner(), expected);
        Ok(())
    }

    #[test]
    fn dimensions() -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation { permutation, .. } = permutation::non_identity();
        let expected = ImageDimensions::from_image(&permutation)?;
        let validated_permutation = super::super::validate_permutation(permutation)?;
        assert_eq!(*validated_permutation.dimensions(), expected);
        Ok(())
    }

    mod io {
        use crate::compute::format::{ImageFileReader, ImageFileWriter, VectorFieldImageBuffer};
        use std::error::Error;
        use test_utils::permutation::{self, DimensionsAndPermutation};

        #[test]
        fn success() -> Result<(), Box<dyn Error>> {
            let DimensionsAndPermutation { permutation, .. } = permutation::non_identity();
            let expected = permutation.clone();
            let validated_permutation = super::super::super::validate_permutation(permutation)?;
            assert_eq!(*validated_permutation.as_ref(), expected);

            let path = test_utils::make_test_output_path_string(&[
                "image_utils_validation_validated_permutation_io_image",
            ]);
            let expected_output_path = VectorFieldImageBuffer::make_filename(&path);
            if expected_output_path.is_file() {
                panic!("Image already exists in the filesystem")
            }

            let full_output_path = validated_permutation.save_add_extension(path)?;
            assert_eq!(full_output_path, expected_output_path);

            let read_image = VectorFieldImageBuffer::load(&full_output_path)?;
            assert_eq!(read_image, expected);

            Ok(std::fs::remove_file(expected_output_path)?)
        }

        #[test]
        fn save_missing_directory() -> Result<(), Box<dyn Error>> {
            let DimensionsAndPermutation { permutation, .. } = permutation::non_identity();
            let validated_permutation = super::super::super::validate_permutation(permutation)?;
            test_utils::assert_error_contains(
                validated_permutation.save_add_extension(&test_utils::make_test_output_path(&[
                    "not_found",
                    "cannot_create",
                ])),
                "No such file or directory",
            );
            Ok(())
        }
    }
}

mod validate_permutation {
    use super::super::validate_permutation;
    use crate::compute::conversion::{self, VectorFieldEntry};
    use crate::ImageDimensions;
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
}
