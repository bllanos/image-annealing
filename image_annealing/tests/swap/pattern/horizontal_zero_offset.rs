mod accept_all_swap {
    use image_annealing::compute::{self, Config, OutputStatus, SwapInput};
    use image_annealing::{CandidatePermutation, DisplacementGoal};
    use std::error::Error;
    use test_utils::algorithm::assert_step_until_success;
    use test_utils::operation::{assert_correct_swap_count_output, SwapAcceptedCount};
    use test_utils::permutation::DimensionsAndPermutation;

    #[test]
    fn run_once_identity() -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation {
            permutation,
            dimensions,
        } = test_utils::permutation::identity();
        let expected_permutation = test_utils::operation::swap(&permutation);
        let displacement_goal =
            DisplacementGoal::from_raw_candidate_permutation(expected_permutation.clone())?;
        let expected_displacement_goal = displacement_goal.as_ref().clone();

        let dispatcher = compute::create_dispatcher(&Config {
            image_dimensions: dimensions,
        })?;
        let swap_parameters = test_utils::algorithm::default_swap_parameters();
        let mut algorithm = dispatcher.swap(
            SwapInput {
                candidate_permutation: Some(CandidatePermutation::new(permutation.clone())?),
                displacement_goal: Some(displacement_goal),
            },
            &swap_parameters,
        );
        assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialAndFullOutput)?;

        let output = algorithm.full_output().unwrap();
        assert_eq!(*output.input_permutation.unwrap().as_ref(), permutation);
        assert_eq!(
            *output.input_displacement_goal.unwrap().as_ref(),
            expected_displacement_goal
        );
        assert_eq!(*output.output_permutation.as_ref(), expected_permutation);
        assert!(algorithm.full_output().is_none());
        assert_correct_swap_count_output(
            algorithm.partial_output(),
            &swap_parameters,
            &dimensions,
            SwapAcceptedCount::All,
        );
        assert!(algorithm.partial_output().is_none());
        Ok(())
    }

    #[test]
    fn reflect_around_center() -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation {
            permutation,
            dimensions,
        } = test_utils::permutation::reflect_around_center();
        let expected_permutation = test_utils::operation::swap(&permutation);
        let displacement_goal =
            DisplacementGoal::from_raw_candidate_permutation(expected_permutation.clone())?;
        let expected_displacement_goal = displacement_goal.as_ref().clone();

        let dispatcher = compute::create_dispatcher(&Config {
            image_dimensions: dimensions,
        })?;
        let swap_parameters = test_utils::algorithm::default_swap_parameters();
        let mut algorithm = dispatcher.swap(
            SwapInput {
                candidate_permutation: Some(CandidatePermutation::new(permutation.clone())?),
                displacement_goal: Some(displacement_goal),
            },
            &swap_parameters,
        );
        assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialAndFullOutput)?;

        let output = algorithm.full_output().unwrap();
        assert_eq!(*output.input_permutation.unwrap().as_ref(), permutation);
        assert_eq!(
            *output.input_displacement_goal.unwrap().as_ref(),
            expected_displacement_goal
        );
        assert_eq!(*output.output_permutation.as_ref(), expected_permutation);
        assert_correct_swap_count_output(
            algorithm.partial_output(),
            &swap_parameters,
            &dimensions,
            SwapAcceptedCount::All,
        );
        Ok(())
    }
}

mod select_swap {
    use image_annealing::compute::conversion::{self, VectorFieldEntry};
    use image_annealing::compute::{self, Config, OutputStatus, SwapInput};
    use image_annealing::{CandidatePermutation, DisplacementGoal};
    use std::error::Error;
    use test_utils::algorithm::assert_step_until_success;
    use test_utils::displacement_goal;
    use test_utils::operation::{assert_correct_swap_count_output, SwapAcceptedCount};
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

        let dispatcher = compute::create_dispatcher(&Config {
            image_dimensions: dimensions,
        })?;
        let swap_parameters = test_utils::algorithm::default_swap_parameters();
        let mut algorithm = dispatcher.swap(
            SwapInput {
                candidate_permutation: Some(CandidatePermutation::new(permutation.clone())?),
                displacement_goal: Some(displacement_goal),
            },
            &swap_parameters,
        );
        assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialAndFullOutput)?;

        let output = algorithm.full_output().unwrap();
        assert_eq!(*output.input_permutation.unwrap().as_ref(), permutation);
        assert_eq!(
            *output.input_displacement_goal.unwrap().as_ref(),
            expected_displacement_goal
        );
        assert_eq!(*output.output_permutation.as_ref(), expected_permutation);
        assert!(algorithm.full_output().is_none());
        assert_correct_swap_count_output(
            algorithm.partial_output(),
            &swap_parameters,
            &dimensions,
            SwapAcceptedCount::Some(vec![1]),
        );
        assert!(algorithm.partial_output().is_none());
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
            DisplacementGoal::from_vector_field(conversion::to_image(&dimensions, &v))?;
        let expected_displacement_goal = displacement_goal.as_ref().clone();

        let dispatcher = compute::create_dispatcher(&Config {
            image_dimensions: dimensions,
        })?;
        let swap_parameters = test_utils::algorithm::default_swap_parameters();
        let mut algorithm = dispatcher.swap(
            SwapInput {
                candidate_permutation: Some(CandidatePermutation::new(permutation.clone())?),
                displacement_goal: Some(displacement_goal),
            },
            &swap_parameters,
        );
        assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialAndFullOutput)?;

        let output = algorithm.full_output().unwrap();
        assert_eq!(*output.input_permutation.unwrap().as_ref(), permutation);
        assert_eq!(
            *output.input_displacement_goal.unwrap().as_ref(),
            expected_displacement_goal
        );
        assert_eq!(*output.output_permutation.as_ref(), expected_permutation);
        assert!(algorithm.full_output().is_none());
        assert_correct_swap_count_output(
            algorithm.partial_output(),
            &swap_parameters,
            &dimensions,
            SwapAcceptedCount::None,
        );
        assert!(algorithm.partial_output().is_none());
        Ok(())
    }
}

mod swap_acceptance_threshold {
    use image_annealing::compute::conversion::{self, VectorFieldEntry};
    use image_annealing::compute::{
        self, Config, OutputStatus, SwapInput, SwapParameters, SwapPass,
    };
    use image_annealing::CandidatePermutation;
    use std::error::Error;
    use test_utils::algorithm::assert_step_until_success;
    use test_utils::displacement_goal;
    use test_utils::operation::{assert_correct_swap_count_output, SwapAcceptedCount};
    use test_utils::permutation::DimensionsAndPermutation;

    #[test]
    fn increasing_threshold() -> Result<(), Box<dyn Error>> {
        let epsilon: f32 = 0.000001;

        let DimensionsAndPermutation {
            permutation,
            dimensions,
        } = test_utils::permutation::non_identity();
        let expected_permutation = permutation.clone();
        let displacement_goal = displacement_goal::identity(&dimensions);
        let expected_displacement_goal = displacement_goal.as_ref().clone();

        let mut dispatcher = compute::create_dispatcher(&Config {
            image_dimensions: dimensions,
        })?;
        let mut swap_acceptance_threshold = -2.0;
        let mut swap_parameters = SwapParameters {
            sequence: SwapPass::Horizontal.into(),
            swap_acceptance_threshold,
            count_swap: true,
        };
        let mut algorithm = dispatcher.swap(
            SwapInput {
                candidate_permutation: Some(CandidatePermutation::new(permutation.clone())?),
                displacement_goal: Some(displacement_goal),
            },
            &swap_parameters,
        );
        assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialAndFullOutput)?;

        let output = algorithm.full_output().unwrap();
        assert_eq!(*output.input_permutation.unwrap().as_ref(), permutation);
        assert_eq!(
            *output.input_displacement_goal.unwrap().as_ref(),
            expected_displacement_goal
        );
        assert_eq!(*output.output_permutation.as_ref(), expected_permutation);
        assert!(algorithm.full_output().is_none());
        assert_correct_swap_count_output(
            algorithm.partial_output(),
            &swap_parameters,
            &dimensions,
            SwapAcceptedCount::None,
        );
        assert!(algorithm.partial_output().is_none());

        dispatcher = algorithm.return_to_dispatcher();

        swap_acceptance_threshold = 1.0 - 2.0_f32.sqrt();
        swap_parameters.swap_acceptance_threshold = swap_acceptance_threshold - epsilon;
        algorithm = dispatcher.swap(Default::default(), &swap_parameters);
        assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialAndFullOutput)?;

        let output = algorithm.full_output().unwrap();
        assert!(output.input_permutation.is_none());
        assert!(output.input_displacement_goal.is_none());
        assert_eq!(*output.output_permutation.as_ref(), expected_permutation);
        assert!(algorithm.full_output().is_none());
        assert_correct_swap_count_output(
            algorithm.partial_output(),
            &swap_parameters,
            &dimensions,
            SwapAcceptedCount::None,
        );
        assert!(algorithm.partial_output().is_none());

        dispatcher = algorithm.return_to_dispatcher();

        swap_parameters.swap_acceptance_threshold = swap_acceptance_threshold + epsilon;
        let mut v = vec![
            VectorFieldEntry(0, 1),
            VectorFieldEntry(0, 0),
            VectorFieldEntry(0, -1),
            VectorFieldEntry(-1, 1),
            VectorFieldEntry(1, -1),
            VectorFieldEntry(0, 0),
        ];
        let expected_permutation_last_row_swapped = conversion::to_image(&dimensions, &v);
        algorithm = dispatcher.swap(Default::default(), &swap_parameters);
        assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialAndFullOutput)?;

        let output = algorithm.full_output().unwrap();
        assert!(output.input_permutation.is_none());
        assert!(output.input_displacement_goal.is_none());
        assert_eq!(
            *output.output_permutation.as_ref(),
            expected_permutation_last_row_swapped
        );
        assert!(algorithm.full_output().is_none());
        assert_correct_swap_count_output(
            algorithm.partial_output(),
            &swap_parameters,
            &dimensions,
            SwapAcceptedCount::Some(vec![1]),
        );
        assert!(algorithm.partial_output().is_none());

        dispatcher = algorithm.return_to_dispatcher();

        swap_acceptance_threshold = 0.0;
        swap_parameters.swap_acceptance_threshold = swap_acceptance_threshold - epsilon;
        algorithm = dispatcher.swap(Default::default(), &swap_parameters);
        assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialAndFullOutput)?;

        let output = algorithm.full_output().unwrap();
        assert!(output.input_permutation.is_none());
        assert!(output.input_displacement_goal.is_none());
        assert_eq!(
            *output.output_permutation.as_ref(),
            expected_permutation_last_row_swapped
        );
        assert!(algorithm.full_output().is_none());
        assert_correct_swap_count_output(
            algorithm.partial_output(),
            &swap_parameters,
            &dimensions,
            SwapAcceptedCount::None,
        );
        assert!(algorithm.partial_output().is_none());

        dispatcher = algorithm.return_to_dispatcher();

        swap_parameters.swap_acceptance_threshold = swap_acceptance_threshold + epsilon;
        v = vec![
            VectorFieldEntry(0, 1),
            VectorFieldEntry(0, 0),
            VectorFieldEntry(0, 1),
            VectorFieldEntry(-1, -1),
            VectorFieldEntry(1, -1),
            VectorFieldEntry(0, 0),
        ];
        let expected_permutation_last_two_rows_swapped = conversion::to_image(&dimensions, &v);
        algorithm = dispatcher.swap(Default::default(), &swap_parameters);
        assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialAndFullOutput)?;

        let output = algorithm.full_output().unwrap();
        assert!(output.input_permutation.is_none());
        assert!(output.input_displacement_goal.is_none());
        assert_eq!(
            *output.output_permutation.as_ref(),
            expected_permutation_last_two_rows_swapped
        );
        assert!(algorithm.full_output().is_none());
        assert_correct_swap_count_output(
            algorithm.partial_output(),
            &swap_parameters,
            &dimensions,
            SwapAcceptedCount::Some(vec![1]),
        );
        assert!(algorithm.partial_output().is_none());

        dispatcher = algorithm.return_to_dispatcher();

        algorithm = dispatcher.swap(Default::default(), &swap_parameters);
        assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialAndFullOutput)?;

        let output = algorithm.full_output().unwrap();
        assert!(output.input_permutation.is_none());
        assert!(output.input_displacement_goal.is_none());
        assert_eq!(
            *output.output_permutation.as_ref(),
            expected_permutation_last_row_swapped
        );
        assert!(algorithm.full_output().is_none());
        assert_correct_swap_count_output(
            algorithm.partial_output(),
            &swap_parameters,
            &dimensions,
            SwapAcceptedCount::Some(vec![1]),
        );
        assert!(algorithm.partial_output().is_none());

        dispatcher = algorithm.return_to_dispatcher();

        swap_acceptance_threshold = 2.0 - 2.0_f32.sqrt();
        swap_parameters.swap_acceptance_threshold = swap_acceptance_threshold - epsilon;
        v = vec![
            VectorFieldEntry(0, 1),
            VectorFieldEntry(0, 0),
            VectorFieldEntry(0, 1),
            VectorFieldEntry(-1, -1),
            VectorFieldEntry(1, 0),
            VectorFieldEntry(0, -1),
        ];
        let expected_permutation_middle_row_swapped = conversion::to_image(&dimensions, &v);
        algorithm = dispatcher.swap(Default::default(), &swap_parameters);
        assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialAndFullOutput)?;

        let output = algorithm.full_output().unwrap();
        assert!(output.input_permutation.is_none());
        assert!(output.input_displacement_goal.is_none());
        assert_eq!(
            *output.output_permutation.as_ref(),
            expected_permutation_middle_row_swapped
        );
        assert!(algorithm.full_output().is_none());
        assert_correct_swap_count_output(
            algorithm.partial_output(),
            &swap_parameters,
            &dimensions,
            SwapAcceptedCount::Some(vec![2]),
        );
        assert!(algorithm.partial_output().is_none());

        dispatcher = algorithm.return_to_dispatcher();

        algorithm = dispatcher.swap(Default::default(), &swap_parameters);
        assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialAndFullOutput)?;

        let output = algorithm.full_output().unwrap();
        assert!(output.input_permutation.is_none());
        assert!(output.input_displacement_goal.is_none());
        assert_eq!(
            *output.output_permutation.as_ref(),
            expected_permutation_last_row_swapped
        );
        assert!(algorithm.full_output().is_none());
        assert_correct_swap_count_output(
            algorithm.partial_output(),
            &swap_parameters,
            &dimensions,
            SwapAcceptedCount::Some(vec![2]),
        );
        assert!(algorithm.partial_output().is_none());

        dispatcher = algorithm.return_to_dispatcher();

        swap_parameters.swap_acceptance_threshold = swap_acceptance_threshold + epsilon;
        v = vec![
            VectorFieldEntry(1, 0),
            VectorFieldEntry(-1, 1),
            VectorFieldEntry(0, 1),
            VectorFieldEntry(-1, -1),
            VectorFieldEntry(1, 0),
            VectorFieldEntry(0, -1),
        ];
        let expected_permutation_first_two_rows_swapped = conversion::to_image(&dimensions, &v);
        algorithm = dispatcher.swap(Default::default(), &swap_parameters);
        assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialAndFullOutput)?;

        let output = algorithm.full_output().unwrap();
        assert!(output.input_permutation.is_none());
        assert!(output.input_displacement_goal.is_none());
        assert_eq!(
            *output.output_permutation.as_ref(),
            expected_permutation_first_two_rows_swapped
        );
        assert!(algorithm.full_output().is_none());
        assert_correct_swap_count_output(
            algorithm.partial_output(),
            &swap_parameters,
            &dimensions,
            SwapAcceptedCount::All,
        );
        assert!(algorithm.partial_output().is_none());

        Ok(())
    }
}
