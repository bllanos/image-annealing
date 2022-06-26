use image_annealing::compute::conversion::{self, VectorFieldEntry};
use image_annealing::compute::{
    self, Config, OutputStatus, SwapInput, SwapParameters, SwapPassSequence,
};
use image_annealing::{CandidatePermutation, DisplacementGoal};
use std::error::Error;
use test_utils::algorithm::assert_step_until_success;
use test_utils::operation::{assert_correct_swap_count_output, SwapAcceptedCount};
use test_utils::permutation::DimensionsAndPermutation;

fn test_swap_pass_sequence(
    sequence: SwapPassSequence,
    expected_permutation_vector: &[VectorFieldEntry],
    expected_swap_counts: Vec<usize>,
) -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::eight_cycle();
    let expected_permutation = conversion::to_image(&dimensions, expected_permutation_vector);
    let displacement_goal =
        DisplacementGoal::from_vector_field(test_utils::permutation::eight_cycle2().permutation)?;
    let expected_displacement_goal = displacement_goal.as_ref().clone();

    let dispatcher = compute::create_dispatcher(&Config {
        image_dimensions: dimensions,
    })?;
    let swap_parameters = SwapParameters {
        sequence,
        ..test_utils::algorithm::default_swap_parameters()
    };
    let mut algorithm = dispatcher.swap(
        SwapInput {
            candidate_permutation: Some(CandidatePermutation::new(permutation.clone())?),
            displacement_goal: Some(displacement_goal),
        },
        &swap_parameters,
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialOutput)?;

    let mut output = algorithm.full_output().unwrap();
    let returned_input = output.input.as_mut().unwrap();
    assert_eq!(
        returned_input.permutation.as_mut().unwrap().as_ref(),
        &permutation
    );
    assert_eq!(
        returned_input.displacement_goal.as_mut().unwrap().as_ref(),
        &expected_displacement_goal
    );
    assert_eq!(output.output_permutation.as_ref(), &expected_permutation);
    assert_eq!(&output.pass, sequence.iter().next().unwrap());
    assert!(algorithm.full_output().is_none());

    assert_correct_swap_count_output(
        algorithm.as_mut(),
        &swap_parameters,
        &dimensions,
        SwapAcceptedCount::Some(expected_swap_counts),
    );
    Ok(())
}

fn eight_cycle_horizontal_swap() -> Vec<VectorFieldEntry> {
    vec![
        VectorFieldEntry(2, 0),
        VectorFieldEntry(0, 0),
        VectorFieldEntry(0, 1),
        VectorFieldEntry(0, -1),
        VectorFieldEntry(0, 0),
        VectorFieldEntry(0, 1),
        VectorFieldEntry(0, 0),
        VectorFieldEntry(-1, -1),
        VectorFieldEntry(-1, 0),
    ]
}

fn eight_cycle_vertical_swap() -> Vec<VectorFieldEntry> {
    vec![
        VectorFieldEntry(0, 0),
        VectorFieldEntry(1, 0),
        VectorFieldEntry(0, 2),
        VectorFieldEntry(1, -1),
        VectorFieldEntry(0, 0),
        VectorFieldEntry(0, 0),
        VectorFieldEntry(0, -1),
        VectorFieldEntry(-1, 0),
        VectorFieldEntry(-1, 0),
    ]
}

fn eight_cycle_offset_horizontal_swap() -> Vec<VectorFieldEntry> {
    vec![
        VectorFieldEntry(1, 0),
        VectorFieldEntry(1, 1),
        VectorFieldEntry(0, 0),
        VectorFieldEntry(0, -1),
        VectorFieldEntry(0, 0),
        VectorFieldEntry(0, 1),
        VectorFieldEntry(0, -1),
        VectorFieldEntry(0, 0),
        VectorFieldEntry(-2, 0),
    ]
}

fn eight_cycle_offset_vertical_swap() -> Vec<VectorFieldEntry> {
    vec![
        VectorFieldEntry(1, 0),
        VectorFieldEntry(1, 0),
        VectorFieldEntry(0, 1),
        VectorFieldEntry(0, 0),
        VectorFieldEntry(0, 0),
        VectorFieldEntry(-1, 1),
        VectorFieldEntry(0, -2),
        VectorFieldEntry(-1, 0),
        VectorFieldEntry(0, 0),
    ]
}

mod single_pass {
    use image_annealing::compute::SwapPass;
    use std::error::Error;

    #[test]
    fn horizontal() -> Result<(), Box<dyn Error>> {
        super::test_swap_pass_sequence(
            SwapPass::Horizontal.into(),
            &super::eight_cycle_horizontal_swap(),
            vec![2],
        )
    }

    #[test]
    fn vertical() -> Result<(), Box<dyn Error>> {
        super::test_swap_pass_sequence(
            SwapPass::Vertical.into(),
            &super::eight_cycle_vertical_swap(),
            vec![2],
        )
    }

    #[test]
    fn offset_horizontal() -> Result<(), Box<dyn Error>> {
        super::test_swap_pass_sequence(
            SwapPass::OffsetHorizontal.into(),
            &super::eight_cycle_offset_horizontal_swap(),
            vec![2],
        )
    }

    #[test]
    fn offset_vertical() -> Result<(), Box<dyn Error>> {
        super::test_swap_pass_sequence(
            SwapPass::OffsetVertical.into(),
            &super::eight_cycle_offset_vertical_swap(),
            vec![2],
        )
    }
}

mod reject_out_of_bounds {
    use image_annealing::compute::conversion::{self, VectorFieldEntry};
    use image_annealing::compute::{self, Config, OutputStatus, SwapInput};
    use image_annealing::{CandidatePermutation, DisplacementGoal};
    use std::error::Error;
    use test_utils::algorithm::{
        assert_correct_default_swap_full_output, assert_step_until_success,
    };
    use test_utils::operation::{assert_correct_swap_count_output, SwapAcceptedCount};
    use test_utils::permutation::DimensionsAndPermutation;

    fn run_test(
        width: usize,
        height: usize,
        displacement_goal_vector: &[VectorFieldEntry],
    ) -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation {
            permutation,
            dimensions,
        } = test_utils::permutation::identity_with_dimensions(width, height);
        let expected_permutation = permutation.clone();
        let displacement_goal = DisplacementGoal::from_vector_field(conversion::to_image(
            &dimensions,
            displacement_goal_vector,
        ))?;
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
        Ok(())
    }

    #[test]
    fn horizontal() -> Result<(), Box<dyn Error>> {
        run_test(
            3,
            1,
            &vec![
                VectorFieldEntry(0, 0),
                VectorFieldEntry(0, 0),
                VectorFieldEntry(1, 0),
            ],
        )
    }

    #[test]
    fn vertical() -> Result<(), Box<dyn Error>> {
        run_test(
            1,
            3,
            &vec![
                VectorFieldEntry(0, 0),
                VectorFieldEntry(0, 0),
                VectorFieldEntry(0, 1),
            ],
        )
    }

    #[test]
    fn offset_horizontal() -> Result<(), Box<dyn Error>> {
        run_test(2, 1, &vec![VectorFieldEntry(-1, 0), VectorFieldEntry(1, 0)])
    }

    #[test]
    fn offset_vertical() -> Result<(), Box<dyn Error>> {
        run_test(1, 2, &vec![VectorFieldEntry(0, -1), VectorFieldEntry(0, 1)])
    }
}
