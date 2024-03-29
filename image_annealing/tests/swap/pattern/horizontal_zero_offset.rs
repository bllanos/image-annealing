mod accept_all_swap {
    use image_annealing::compute::{self, Config, OutputStatus, SwapInput};
    use image_annealing::{CandidatePermutation, DisplacementGoal};
    use std::error::Error;
    use test_util::algorithm::{
        assert_correct_default_swap_full_output, assert_correct_default_swap_full_output_async,
        assert_step_until_success, assert_step_until_success_async,
    };
    use test_util::operation::{
        assert_correct_swap_count_output, assert_correct_swap_count_output_async, SwapAcceptedCount,
    };
    use test_util::permutation::DimensionsAndPermutation;

    #[test]
    fn run_once_identity() -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation {
            permutation,
            dimensions,
        } = test_util::permutation::identity();
        let expected_permutation = test_util::operation::swap(&permutation);
        let displacement_goal =
            DisplacementGoal::from_raw_candidate_permutation(expected_permutation.clone())?;
        let expected_displacement_goal = displacement_goal.as_ref().clone();

        let dispatcher = compute::create_dispatcher_block(&Config {
            image_dimensions: dimensions,
        })?;
        let swap_parameters = test_util::algorithm::default_swap_parameters();
        let mut algorithm = dispatcher.swap(
            SwapInput {
                candidate_permutation: Some(CandidatePermutation::from_vector_field(
                    permutation.clone(),
                )?),
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

    async fn run_once_identity_async_inner() -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation {
            permutation,
            dimensions,
        } = test_util::permutation::identity();
        let expected_permutation = test_util::operation::swap(&permutation);
        let displacement_goal =
            DisplacementGoal::from_raw_candidate_permutation(expected_permutation.clone())?;
        let expected_displacement_goal = displacement_goal.as_ref().clone();

        let dispatcher = compute::create_dispatcher(&Config {
            image_dimensions: dimensions,
        })
        .await?;
        let swap_parameters = test_util::algorithm::default_swap_parameters();
        let mut algorithm = dispatcher.swap(
            SwapInput {
                candidate_permutation: Some(CandidatePermutation::from_vector_field(
                    permutation.clone(),
                )?),
                displacement_goal: Some(displacement_goal),
            },
            &swap_parameters,
        );
        assert_step_until_success_async(algorithm.as_mut(), OutputStatus::FinalPartialOutput)
            .await?;

        assert_correct_default_swap_full_output_async(
            algorithm.as_mut(),
            &permutation,
            &expected_displacement_goal,
            &expected_permutation,
        )
        .await;
        assert_correct_swap_count_output_async(
            algorithm.as_mut(),
            &swap_parameters,
            &dimensions,
            SwapAcceptedCount::All,
        )
        .await;
        Ok(())
    }

    #[test]
    fn run_once_identity_async() -> Result<(), Box<dyn Error>> {
        futures::executor::block_on(run_once_identity_async_inner())
    }

    #[test]
    fn reflect_around_center() -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation {
            permutation,
            dimensions,
        } = test_util::permutation::reflect_around_center();
        let expected_permutation = test_util::operation::swap(&permutation);
        let displacement_goal =
            DisplacementGoal::from_raw_candidate_permutation(expected_permutation.clone())?;
        let expected_displacement_goal = displacement_goal.as_ref().clone();

        let dispatcher = compute::create_dispatcher_block(&Config {
            image_dimensions: dimensions,
        })?;
        let swap_parameters = test_util::algorithm::default_swap_parameters();
        let mut algorithm = dispatcher.swap(
            SwapInput {
                candidate_permutation: Some(CandidatePermutation::from_vector_field(
                    permutation.clone(),
                )?),
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
    use image_annealing::{CandidatePermutation, DisplacementGoal, VectorField};
    use std::error::Error;
    use test_util::algorithm::{
        assert_correct_default_swap_full_output, assert_step_until_success,
    };
    use test_util::operation::{assert_correct_swap_count_output, SwapAcceptedCount};
    use test_util::permutation::DimensionsAndPermutation;

    #[test]
    fn identity_goal() -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation {
            permutation,
            dimensions,
        } = test_util::permutation::non_identity();
        let v = vec![
            VectorFieldEntry(0, 1),
            VectorFieldEntry(0, 0),
            VectorFieldEntry(0, -1),
            VectorFieldEntry(-1, 1),
            VectorFieldEntry(1, -1),
            VectorFieldEntry(0, 0),
        ];
        let expected_permutation = conversion::to_image(&dimensions, &v);
        let displacement_goal = DisplacementGoal::identity(&dimensions);
        let expected_displacement_goal = displacement_goal.as_ref().clone();

        let dispatcher = compute::create_dispatcher_block(&Config {
            image_dimensions: dimensions,
        })?;
        let swap_parameters = test_util::algorithm::default_swap_parameters();
        let mut algorithm = dispatcher.swap(
            SwapInput {
                candidate_permutation: Some(CandidatePermutation::from_vector_field(
                    permutation.clone(),
                )?),
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
    use image_annealing::{CandidatePermutation, DisplacementGoal, VectorField};
    use std::error::Error;
    use test_util::algorithm::{
        assert_correct_default_swap_full_output, assert_step_until_success,
    };
    use test_util::operation::{assert_correct_swap_count_output, SwapAcceptedCount};
    use test_util::permutation::DimensionsAndPermutation;

    #[test]
    fn increasing_threshold() -> Result<(), Box<dyn Error>> {
        let epsilon: f32 = 0.000001;

        let DimensionsAndPermutation {
            permutation,
            dimensions,
        } = test_util::permutation::non_identity();
        let expected_permutation = permutation.clone();
        let displacement_goal = DisplacementGoal::identity(&dimensions);
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
                candidate_permutation: Some(CandidatePermutation::from_vector_field(
                    permutation.clone(),
                )?),
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
