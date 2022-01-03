mod accept_all_swap {
    use image_annealing::compute::{self, OutputStatus, SwapInput, SwapParameters};
    use image_annealing::{CandidatePermutation, DisplacementGoal};
    use std::error::Error;
    use test_utils::algorithm::assert_step_until_success;
    use test_utils::permutation::DimensionsAndPermutation;

    #[test]
    fn run_once_identity() -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation {
            permutation,
            dimensions,
        } = test_utils::permutation::identity();
        let expected_permutation = test_utils::operation::swap(&permutation);
        let displacement_goal = DisplacementGoal::from_candidate_permutation(
            CandidatePermutation(expected_permutation.clone()),
        )?;
        let expected_displacement_goal = displacement_goal.as_ref().clone();

        let dispatcher = compute::create_dispatcher(&dimensions)?;
        let mut algorithm = dispatcher.swap(
            SwapInput {
                candidate_permutation: Some(CandidatePermutation(permutation.clone())),
                displacement_goal: Some(displacement_goal),
            },
            SwapParameters {},
        );
        assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

        let output = algorithm.full_output().unwrap();
        assert_eq!(*output.input_permutation.unwrap().as_ref(), permutation);
        assert_eq!(
            *output.input_displacement_goal.unwrap().as_ref(),
            expected_displacement_goal
        );
        assert_eq!(*output.output_permutation.as_ref(), expected_permutation);
        assert!(algorithm.full_output().is_none());
        Ok(())
    }

    #[test]
    fn reflect_around_center() -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation {
            permutation,
            dimensions,
        } = test_utils::permutation::reflect_around_center();
        let expected_permutation = test_utils::operation::swap(&permutation);
        let displacement_goal = DisplacementGoal::from_candidate_permutation(
            CandidatePermutation(expected_permutation.clone()),
        )?;
        let expected_displacement_goal = displacement_goal.as_ref().clone();

        let dispatcher = compute::create_dispatcher(&dimensions)?;
        let mut algorithm = dispatcher.swap(
            SwapInput {
                candidate_permutation: Some(CandidatePermutation(permutation.clone())),
                displacement_goal: Some(displacement_goal),
            },
            SwapParameters {},
        );
        assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

        let output = algorithm.full_output().unwrap();
        assert_eq!(*output.input_permutation.unwrap().as_ref(), permutation);
        assert_eq!(
            *output.input_displacement_goal.unwrap().as_ref(),
            expected_displacement_goal
        );
        assert_eq!(*output.output_permutation.as_ref(), expected_permutation);
        Ok(())
    }
}

mod select_swap {
    use image_annealing::compute::conversion::{self, VectorFieldEntry};
    use image_annealing::compute::{self, OutputStatus, SwapInput, SwapParameters};
    use image_annealing::{CandidatePermutation, DisplacementGoal};
    use std::error::Error;
    use test_utils::algorithm::assert_step_until_success;
    use test_utils::displacement_goal;
    use test_utils::permutation::DimensionsAndPermutation;

    #[test]
    fn identity_goal() -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation {
            permutation,
            dimensions,
        } = test_utils::permutation::non_identity();
        let v = vec![
            VectorFieldEntry(0, 1),
            VectorFieldEntry(0, 0),
            VectorFieldEntry(0, -1),
            VectorFieldEntry(-1, 1),
            VectorFieldEntry(1, -1),
            VectorFieldEntry(0, 0),
        ];
        let expected_permutation = conversion::to_image(&dimensions, &v);
        let displacement_goal = displacement_goal::identity(&dimensions);
        let expected_displacement_goal = displacement_goal.as_ref().clone();

        let dispatcher = compute::create_dispatcher(&dimensions)?;
        let mut algorithm = dispatcher.swap(
            SwapInput {
                candidate_permutation: Some(CandidatePermutation(permutation.clone())),
                displacement_goal: Some(displacement_goal),
            },
            SwapParameters {},
        );
        assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

        let output = algorithm.full_output().unwrap();
        assert_eq!(*output.input_permutation.unwrap().as_ref(), permutation);
        assert_eq!(
            *output.input_displacement_goal.unwrap().as_ref(),
            expected_displacement_goal
        );
        assert_eq!(*output.output_permutation.as_ref(), expected_permutation);
        assert!(algorithm.full_output().is_none());
        Ok(())
    }

    #[test]
    fn reject_out_of_bounds() -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation {
            permutation,
            dimensions,
        } = test_utils::permutation::identity_with_dimensions(3, 3);
        let v = vec![
            VectorFieldEntry(0, 0),
            VectorFieldEntry(0, 0),
            VectorFieldEntry(1, 0),
            VectorFieldEntry(0, 0),
            VectorFieldEntry(0, 0),
            VectorFieldEntry(1, 0),
            VectorFieldEntry(0, 0),
            VectorFieldEntry(0, 0),
            VectorFieldEntry(1, 0),
        ];
        let expected_permutation = permutation.clone();
        let displacement_goal =
            DisplacementGoal::from_vector_field(conversion::to_image(&dimensions, &v));
        let expected_displacement_goal = displacement_goal.as_ref().clone();

        let dispatcher = compute::create_dispatcher(&dimensions)?;
        let mut algorithm = dispatcher.swap(
            SwapInput {
                candidate_permutation: Some(CandidatePermutation(permutation.clone())),
                displacement_goal: Some(displacement_goal),
            },
            SwapParameters {},
        );
        assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

        let output = algorithm.full_output().unwrap();
        assert_eq!(*output.input_permutation.unwrap().as_ref(), permutation);
        assert_eq!(
            *output.input_displacement_goal.unwrap().as_ref(),
            expected_displacement_goal
        );
        assert_eq!(*output.output_permutation.as_ref(), expected_permutation);
        assert!(algorithm.full_output().is_none());
        Ok(())
    }
}
