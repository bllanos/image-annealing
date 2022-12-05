mod displacement_goal {
    use super::super::validation;
    use super::super::DisplacementGoal;
    use crate::compute::format::Rgba8Image;
    use crate::{CandidatePermutation, ImageDimensions, ImageDimensionsHolder};
    use std::error::Error;
    use test_util::permutation::{self, DimensionsAndPermutation};

    #[test]
    fn from_vector_field_image() -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation { permutation, .. } = permutation::non_identity();
        let expected = permutation.clone();
        let displacement_goal =
            DisplacementGoal::from_vector_field_image(Rgba8Image::new(permutation)?);
        assert_eq!(*displacement_goal.as_ref(), expected);
        Ok(())
    }

    #[test]
    fn from_vector_field() -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation { permutation, .. } = permutation::non_identity();
        let expected = permutation.clone();
        let displacement_goal = DisplacementGoal::from_vector_field(permutation)?;
        assert_eq!(*displacement_goal.as_ref(), expected);
        Ok(())
    }

    #[test]
    fn from_raw_candidate_permutation() -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation { permutation, .. } = permutation::non_identity();
        let expected = validation::validate_permutation(permutation.clone())?.inverse();
        let displacement_goal = DisplacementGoal::from_raw_candidate_permutation(permutation)?;
        assert_eq!(displacement_goal.as_ref(), expected.as_ref());
        Ok(())
    }

    #[test]
    fn from_candidate_permutation() -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation { permutation, .. } = permutation::non_identity();
        let expected = validation::validate_permutation(permutation.clone())?.inverse();
        let displacement_goal =
            DisplacementGoal::from_candidate_permutation(CandidatePermutation::new(permutation)?)?;
        assert_eq!(displacement_goal.as_ref(), expected.as_ref());
        Ok(())
    }

    #[test]
    fn from_validated_permutation() -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation { permutation, .. } = permutation::non_identity();
        let validated_permutation = validation::validate_permutation(permutation)?;
        let expected = validated_permutation.inverse();
        let displacement_goal = DisplacementGoal::from(validated_permutation);
        assert_eq!(displacement_goal.as_ref(), expected.as_ref());
        Ok(())
    }

    #[test]
    fn into_inner() -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation { permutation, .. } = permutation::non_identity();
        let expected = permutation.clone();
        let displacement_goal = DisplacementGoal::from_vector_field(permutation)?;
        assert_eq!(displacement_goal.into_inner(), expected);
        Ok(())
    }

    #[test]
    fn as_raw_slice() -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation { permutation, .. } = permutation::non_identity();
        let expected = permutation.clone();
        let displacement_goal = DisplacementGoal::from_vector_field(permutation)?;
        assert_eq!(
            displacement_goal.as_raw_slice(),
            expected.as_raw().as_slice()
        );
        Ok(())
    }

    #[test]
    fn dimensions() -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation { permutation, .. } = permutation::non_identity();
        let expected = ImageDimensions::from_image(&permutation)?;
        let displacement_goal = DisplacementGoal::from_vector_field(permutation)?;
        assert_eq!(*displacement_goal.dimensions(), expected);
        Ok(())
    }

    mod io {
        use super::super::super::DisplacementGoal;
        use crate::compute::format::{ImageFileReader, ImageFileWriter, VectorFieldImageBuffer};
        use std::error::Error;
        use test_util::permutation::{self, DimensionsAndPermutation};

        #[test]
        fn success() -> Result<(), Box<dyn Error>> {
            let DimensionsAndPermutation { permutation, .. } = permutation::non_identity();
            let expected = permutation.clone();
            let displacement_goal = DisplacementGoal::from_vector_field(permutation)?;
            assert_eq!(*displacement_goal.as_ref(), expected);

            let path =
                test_util::make_test_output_path_string(["image_utils_displacement_goal_io_image"]);
            let expected_output_path = VectorFieldImageBuffer::make_filename(&path);
            assert!(!expected_output_path.is_file());

            let full_output_path = displacement_goal.save_add_extension(path)?;
            assert_eq!(full_output_path, expected_output_path);

            let read_image = VectorFieldImageBuffer::load(&full_output_path)?;
            assert_eq!(read_image, expected);

            let input_displacement_goal = DisplacementGoal::load(&full_output_path)?;
            assert_eq!(*input_displacement_goal.as_ref(), expected);

            assert_eq!(input_displacement_goal, displacement_goal);

            Ok(std::fs::remove_file(expected_output_path)?)
        }

        #[test]
        fn load_missing_image() {
            let path = test_util::make_test_data_path(["image", "image", "not_found.png"]);
            test_util::assert_error_contains(
                DisplacementGoal::load(path),
                "No such file or directory",
            );
        }

        #[test]
        fn save_missing_directory() -> Result<(), Box<dyn Error>> {
            let DimensionsAndPermutation { permutation, .. } = permutation::non_identity();
            let displacement_goal = DisplacementGoal::from_vector_field(permutation)?;
            test_util::assert_error_contains(
                displacement_goal.save_add_extension(&test_util::make_test_output_path([
                    "not_found",
                    "cannot_create",
                ])),
                "No such file or directory",
            );
            Ok(())
        }
    }
}
