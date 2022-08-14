mod accept_all_swap {
    use image_annealing::compute::{self, Config, OutputStatus, SwapInput};
    use image_annealing::{CandidatePermutation, DisplacementGoal};
    use std::error::Error;
    use test_utils::algorithm::{
        assert_correct_default_swap_full_output, assert_step_until_success,
    };
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

        let dispatcher = compute::create_dispatcher_block(&Config {
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
        assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialOutput)?;

        assert_correct_default_swap_full_output(
            algorithm.as_mut(),
            &permutation,
            &expected_displacement_goal,
            &expected_permutation,
        );
        assert_correct_swap_count_output(
            algorithm.as_mut(),
            &swap_parameters,
            &dimensions,
            SwapAcceptedCount::All,
        );
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

        let dispatcher = compute::create_dispatcher_block(&Config {
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
        assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialOutput)?;

        assert_correct_default_swap_full_output(
            algorithm.as_mut(),
            &permutation,
            &expected_displacement_goal,
            &expected_permutation,
        );
        assert_correct_swap_count_output(
            algorithm.as_mut(),
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
    use image_annealing::CandidatePermutation;
    use std::error::Error;
    use test_utils::algorithm::{
        assert_correct_default_swap_full_output, assert_step_until_success,
    };
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

        let dispatcher = compute::create_dispatcher_block(&Config {
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
        assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialOutput)?;

        assert_correct_default_swap_full_output(
            algorithm.as_mut(),
            &permutation,
            &expected_displacement_goal,
            &expected_permutation,
        );
        assert_correct_swap_count_output(
            algorithm.as_mut(),
            &swap_parameters,
            &dimensions,
            SwapAcceptedCount::Some(vec![1]),
        );
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
    use test_utils::algorithm::{
        assert_correct_default_swap_full_output, assert_step_until_success,
    };
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

        let mut dispatcher = compute::create_dispatcher_block(&Config {
            image_dimensions: dimensions,
        })?;
        let mut swap_acceptance_threshold = -2.0;
        let pass = SwapPass::Horizontal;
        let mut swap_parameters = SwapParameters {
            sequence: pass.into(),
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
        assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialOutput)?;

        assert_correct_default_swap_full_output(
            algorithm.as_mut(),
            &permutation,
            &expected_displacement_goal,
            &expected_permutation,
        );
        assert_correct_swap_count_output(
            algorithm.as_mut(),
            &swap_parameters,
            &dimensions,
            SwapAcceptedCount::None,
        );

        dispatcher = algorithm.return_to_dispatcher();

        swap_acceptance_threshold = 1.0 - 2.0_f32.sqrt();
        swap_parameters.swap_acceptance_threshold = swap_acceptance_threshold - epsilon;
        algorithm = dispatcher.swap(Default::default(), &swap_parameters);
        assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialOutput)?;

        let output = algorithm.full_output_block().unwrap();
        assert!(output.input.is_none());
        assert_eq!(*output.output_permutation.as_ref(), expected_permutation);
        assert_eq!(output.pass, pass);
        assert!(algorithm.full_output_block().is_none());
        assert_correct_swap_count_output(
            algorithm.as_mut(),
            &swap_parameters,
            &dimensions,
            SwapAcceptedCount::None,
        );

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
        assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialOutput)?;

        let output = algorithm.full_output_block().unwrap();
        assert!(output.input.is_none());
        assert_eq!(
            *output.output_permutation.as_ref(),
            expected_permutation_last_row_swapped
        );
        assert_eq!(output.pass, pass);
        assert!(algorithm.full_output_block().is_none());
        assert_correct_swap_count_output(
            algorithm.as_mut(),
            &swap_parameters,
            &dimensions,
            SwapAcceptedCount::Some(vec![1]),
        );

        dispatcher = algorithm.return_to_dispatcher();

        swap_acceptance_threshold = 0.0;
        swap_parameters.swap_acceptance_threshold = swap_acceptance_threshold - epsilon;
        algorithm = dispatcher.swap(Default::default(), &swap_parameters);
        assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialOutput)?;

        let output = algorithm.full_output_block().unwrap();
        assert!(output.input.is_none());
        assert_eq!(
            *output.output_permutation.as_ref(),
            expected_permutation_last_row_swapped
        );
        assert_eq!(output.pass, pass);
        assert!(algorithm.full_output_block().is_none());
        assert_correct_swap_count_output(
            algorithm.as_mut(),
            &swap_parameters,
            &dimensions,
            SwapAcceptedCount::None,
        );

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
        assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialOutput)?;

        let output = algorithm.full_output_block().unwrap();
        assert!(output.input.is_none());
        assert_eq!(
            *output.output_permutation.as_ref(),
            expected_permutation_last_two_rows_swapped
        );
        assert_eq!(output.pass, pass);
        assert!(algorithm.full_output_block().is_none());
        assert_correct_swap_count_output(
            algorithm.as_mut(),
            &swap_parameters,
            &dimensions,
            SwapAcceptedCount::Some(vec![1]),
        );

        dispatcher = algorithm.return_to_dispatcher();

        algorithm = dispatcher.swap(Default::default(), &swap_parameters);
        assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialOutput)?;

        let output = algorithm.full_output_block().unwrap();
        assert!(output.input.is_none());
        assert_eq!(
            *output.output_permutation.as_ref(),
            expected_permutation_last_row_swapped
        );
        assert_eq!(output.pass, pass);
        assert!(algorithm.full_output_block().is_none());
        assert_correct_swap_count_output(
            algorithm.as_mut(),
            &swap_parameters,
            &dimensions,
            SwapAcceptedCount::Some(vec![1]),
        );

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
        assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialOutput)?;

        let output = algorithm.full_output_block().unwrap();
        assert!(output.input.is_none());
        assert_eq!(
            *output.output_permutation.as_ref(),
            expected_permutation_middle_row_swapped
        );
        assert_eq!(output.pass, pass);
        assert!(algorithm.full_output_block().is_none());
        assert_correct_swap_count_output(
            algorithm.as_mut(),
            &swap_parameters,
            &dimensions,
            SwapAcceptedCount::Some(vec![2]),
        );

        dispatcher = algorithm.return_to_dispatcher();

        algorithm = dispatcher.swap(Default::default(), &swap_parameters);
        assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialOutput)?;

        let output = algorithm.full_output_block().unwrap();
        assert!(output.input.is_none());
        assert_eq!(
            *output.output_permutation.as_ref(),
            expected_permutation_last_row_swapped
        );
        assert_eq!(output.pass, pass);
        assert!(algorithm.full_output_block().is_none());
        assert_correct_swap_count_output(
            algorithm.as_mut(),
            &swap_parameters,
            &dimensions,
            SwapAcceptedCount::Some(vec![2]),
        );

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
        assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialOutput)?;

        let output = algorithm.full_output_block().unwrap();
        assert!(output.input.is_none());
        assert_eq!(
            *output.output_permutation.as_ref(),
            expected_permutation_first_two_rows_swapped
        );
        assert_eq!(output.pass, pass);
        assert!(algorithm.full_output_block().is_none());
        assert_correct_swap_count_output(
            algorithm.as_mut(),
            &swap_parameters,
            &dimensions,
            SwapAcceptedCount::All,
        );

        Ok(())
    }
}
