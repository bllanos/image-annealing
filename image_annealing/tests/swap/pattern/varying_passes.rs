use image_annealing::compute::conversion::{self, VectorFieldEntry};
use image_annealing::compute::{
    self, Config, OutputStatus, SwapInput, SwapParameters, SwapPassSequence,
};
use image_annealing::{CandidatePermutation, DisplacementGoal};
use std::error::Error;
use test_util::algorithm::assert_step_until_success;
use test_util::operation::{assert_correct_swap_count_output, SwapAcceptedCount};
use test_util::permutation::DimensionsAndPermutation;

fn test_swap_pass_sequence<T, U>(
    sequence: SwapPassSequence,
    expected_permutation_vectors: T,
    expected_swap_counts: Vec<usize>,
) -> Result<(), Box<dyn Error>>
where
    T: IntoIterator<Item = U>,
    U: AsRef<[VectorFieldEntry]>,
{
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_util::permutation::eight_cycle();
    let expected_permutation_iter = expected_permutation_vectors
        .into_iter()
        .map(|v| conversion::to_image(&dimensions, v.as_ref()));
    let displacement_goal = DisplacementGoal::from_raw_candidate_permutation(
        test_util::permutation::eight_cycle2().permutation,
    )?;
    let expected_displacement_goal = displacement_goal.as_ref().clone();

    let dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;
    let swap_parameters = SwapParameters {
        sequence,
        ..test_util::algorithm::default_swap_parameters()
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

    let full_output_iter = sequence.iter().map(|_| {
        assert_step_until_success(algorithm.as_mut(), OutputStatus::NewFullOutput).unwrap();
        let output = algorithm.full_output_block().unwrap();
        assert!(algorithm.full_output_block().is_none());
        output
    });

    assert_eq!(
        full_output_iter
            .zip(expected_permutation_iter)
            .zip(sequence.iter())
            .enumerate()
            .map(|(i, ((output, expected_permutation), pass))| {
                if i == 0 {
                    let returned_input = output.input.as_ref().unwrap();
                    assert_eq!(
                        returned_input.permutation.as_ref().unwrap().as_ref(),
                        &permutation
                    );
                    assert_eq!(
                        returned_input.displacement_goal.as_ref().unwrap().as_ref(),
                        &expected_displacement_goal
                    );
                } else {
                    assert!(output.input.is_none());
                }

                assert_eq!(output.output_permutation.as_ref(), &expected_permutation);
                assert_eq!(&output.pass, pass);
                i
            })
            .last(),
        Some(sequence.iter().count() - 1)
    );

    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalPartialOutput)?;
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

fn eight_cycle_horizontal_vertical_swap() -> Vec<VectorFieldEntry> {
    vec![
        VectorFieldEntry(2, 0),
        VectorFieldEntry(0, 0),
        VectorFieldEntry(0, 2),
        VectorFieldEntry(0, -1),
        VectorFieldEntry(0, 0),
        VectorFieldEntry(0, 0),
        VectorFieldEntry(0, 0),
        VectorFieldEntry(-1, -1),
        VectorFieldEntry(-1, 0),
    ]
}

fn eight_cycle_offset_vertical_offset_horizontal_swap() -> Vec<VectorFieldEntry> {
    vec![
        VectorFieldEntry(1, 0),
        VectorFieldEntry(1, 1),
        VectorFieldEntry(0, 0),
        VectorFieldEntry(0, 0),
        VectorFieldEntry(0, 0),
        VectorFieldEntry(-1, 1),
        VectorFieldEntry(0, -2),
        VectorFieldEntry(1, 0),
        VectorFieldEntry(-2, 0),
    ]
}

mod single_pass {
    use image_annealing::compute::SwapPass;
    use std::error::Error;

    #[test]
    fn horizontal() -> Result<(), Box<dyn Error>> {
        super::test_swap_pass_sequence(
            SwapPass::Horizontal.into(),
            [super::eight_cycle_horizontal_swap()],
            vec![2],
        )
    }

    #[test]
    fn vertical() -> Result<(), Box<dyn Error>> {
        super::test_swap_pass_sequence(
            SwapPass::Vertical.into(),
            [super::eight_cycle_vertical_swap()],
            vec![2],
        )
    }

    #[test]
    fn offset_horizontal() -> Result<(), Box<dyn Error>> {
        super::test_swap_pass_sequence(
            SwapPass::OffsetHorizontal.into(),
            [super::eight_cycle_offset_horizontal_swap()],
            vec![2],
        )
    }

    #[test]
    fn offset_vertical() -> Result<(), Box<dyn Error>> {
        super::test_swap_pass_sequence(
            SwapPass::OffsetVertical.into(),
            [super::eight_cycle_offset_vertical_swap()],
            vec![2],
        )
    }
}

mod two_passes {
    use image_annealing::compute::conversion::VectorFieldEntry;
    use image_annealing::compute::{SwapPass, SwapPassSequence};
    use std::error::Error;

    #[test]
    fn horizontal_vertical() -> Result<(), Box<dyn Error>> {
        super::test_swap_pass_sequence(
            SwapPassSequence::from_passes([SwapPass::Horizontal, SwapPass::Vertical])?,
            [
                super::eight_cycle_horizontal_swap(),
                super::eight_cycle_horizontal_vertical_swap(),
            ],
            vec![2, 1],
        )
    }

    #[test]
    fn horizontal_offset_horizontal() -> Result<(), Box<dyn Error>> {
        super::test_swap_pass_sequence(
            SwapPassSequence::from_passes([SwapPass::Horizontal, SwapPass::OffsetHorizontal])?,
            [
                super::eight_cycle_horizontal_swap(),
                vec![
                    VectorFieldEntry(2, 0),
                    VectorFieldEntry(1, 1),
                    VectorFieldEntry(-1, 0),
                    VectorFieldEntry(0, -1),
                    VectorFieldEntry(0, 0),
                    VectorFieldEntry(0, 1),
                    VectorFieldEntry(0, 0),
                    VectorFieldEntry(-1, -1),
                    VectorFieldEntry(-1, 0),
                ],
            ],
            vec![2, 1],
        )
    }

    #[test]
    fn horizontal_offset_vertical() -> Result<(), Box<dyn Error>> {
        super::test_swap_pass_sequence(
            SwapPassSequence::from_passes([SwapPass::Horizontal, SwapPass::OffsetVertical])?,
            [
                super::eight_cycle_horizontal_swap(),
                vec![
                    VectorFieldEntry(2, 0),
                    VectorFieldEntry(0, 0),
                    VectorFieldEntry(0, 1),
                    VectorFieldEntry(0, 1),
                    VectorFieldEntry(0, 0),
                    VectorFieldEntry(-1, 1),
                    VectorFieldEntry(0, -2),
                    VectorFieldEntry(-1, -1),
                    VectorFieldEntry(0, 0),
                ],
            ],
            vec![2, 2],
        )
    }

    #[test]
    fn vertical_horizontal() -> Result<(), Box<dyn Error>> {
        super::test_swap_pass_sequence(
            SwapPassSequence::from_passes([SwapPass::Vertical, SwapPass::Horizontal])?,
            [
                super::eight_cycle_vertical_swap(),
                vec![
                    VectorFieldEntry(2, 0),
                    VectorFieldEntry(-1, 0),
                    VectorFieldEntry(0, 2),
                    VectorFieldEntry(1, -1),
                    VectorFieldEntry(0, 0),
                    VectorFieldEntry(0, 0),
                    VectorFieldEntry(0, 0),
                    VectorFieldEntry(-1, -1),
                    VectorFieldEntry(-1, 0),
                ],
            ],
            vec![2, 2],
        )
    }

    #[test]
    fn vertical_offset_horizontal() -> Result<(), Box<dyn Error>> {
        super::test_swap_pass_sequence(
            SwapPassSequence::from_passes([SwapPass::Vertical, SwapPass::OffsetHorizontal])?,
            [
                super::eight_cycle_vertical_swap(),
                vec![
                    VectorFieldEntry(0, 0),
                    VectorFieldEntry(1, 0),
                    VectorFieldEntry(0, 2),
                    VectorFieldEntry(1, -1),
                    VectorFieldEntry(0, 0),
                    VectorFieldEntry(0, 0),
                    VectorFieldEntry(0, -1),
                    VectorFieldEntry(0, 0),
                    VectorFieldEntry(-2, 0),
                ],
            ],
            vec![2, 1],
        )
    }

    #[test]
    fn vertical_offset_vertical() -> Result<(), Box<dyn Error>> {
        super::test_swap_pass_sequence(
            SwapPassSequence::from_passes([SwapPass::Vertical, SwapPass::OffsetVertical])?,
            [
                super::eight_cycle_vertical_swap(),
                vec![
                    VectorFieldEntry(0, 0),
                    VectorFieldEntry(1, 0),
                    VectorFieldEntry(0, 2),
                    VectorFieldEntry(1, -1),
                    VectorFieldEntry(0, 0),
                    VectorFieldEntry(-1, 1),
                    VectorFieldEntry(0, -1),
                    VectorFieldEntry(-1, 0),
                    VectorFieldEntry(0, -1),
                ],
            ],
            vec![2, 1],
        )
    }

    #[test]
    fn offset_horizontal_horizontal() -> Result<(), Box<dyn Error>> {
        super::test_swap_pass_sequence(
            SwapPassSequence::from_passes([SwapPass::OffsetHorizontal, SwapPass::Horizontal])?,
            [
                super::eight_cycle_offset_horizontal_swap(),
                vec![
                    VectorFieldEntry(1, 0),
                    VectorFieldEntry(1, 1),
                    VectorFieldEntry(0, 0),
                    VectorFieldEntry(0, -1),
                    VectorFieldEntry(0, 0),
                    VectorFieldEntry(0, 1),
                    VectorFieldEntry(1, 0),
                    VectorFieldEntry(-1, -1),
                    VectorFieldEntry(-2, 0),
                ],
            ],
            vec![2, 1],
        )
    }

    #[test]
    fn offset_horizontal_vertical() -> Result<(), Box<dyn Error>> {
        super::test_swap_pass_sequence(
            SwapPassSequence::from_passes([SwapPass::OffsetHorizontal, SwapPass::Vertical])?,
            [
                super::eight_cycle_offset_horizontal_swap(),
                vec![
                    VectorFieldEntry(0, 0),
                    VectorFieldEntry(1, 1),
                    VectorFieldEntry(0, 2),
                    VectorFieldEntry(1, -1),
                    VectorFieldEntry(0, 0),
                    VectorFieldEntry(0, -1),
                    VectorFieldEntry(0, -1),
                    VectorFieldEntry(0, 0),
                    VectorFieldEntry(-2, 0),
                ],
            ],
            vec![2, 2],
        )
    }

    #[test]
    fn offset_horizontal_offset_vertical() -> Result<(), Box<dyn Error>> {
        super::test_swap_pass_sequence(
            SwapPassSequence::from_passes([SwapPass::OffsetHorizontal, SwapPass::OffsetVertical])?,
            [
                super::eight_cycle_offset_horizontal_swap(),
                vec![
                    VectorFieldEntry(1, 0),
                    VectorFieldEntry(1, 1),
                    VectorFieldEntry(0, 0),
                    VectorFieldEntry(0, 0),
                    VectorFieldEntry(0, 0),
                    VectorFieldEntry(0, 1),
                    VectorFieldEntry(0, -2),
                    VectorFieldEntry(0, 0),
                    VectorFieldEntry(-2, 0),
                ],
            ],
            vec![2, 1],
        )
    }

    #[test]
    fn offset_vertical_horizontal() -> Result<(), Box<dyn Error>> {
        super::test_swap_pass_sequence(
            SwapPassSequence::from_passes([SwapPass::OffsetVertical, SwapPass::Horizontal])?,
            [
                super::eight_cycle_offset_vertical_swap(),
                vec![
                    VectorFieldEntry(2, 0),
                    VectorFieldEntry(0, 0),
                    VectorFieldEntry(0, 1),
                    VectorFieldEntry(0, 0),
                    VectorFieldEntry(0, 0),
                    VectorFieldEntry(-1, 1),
                    VectorFieldEntry(0, -2),
                    VectorFieldEntry(-1, 0),
                    VectorFieldEntry(0, 0),
                ],
            ],
            vec![2, 1],
        )
    }

    #[test]
    fn offset_vertical_vertical() -> Result<(), Box<dyn Error>> {
        super::test_swap_pass_sequence(
            SwapPassSequence::from_passes([SwapPass::OffsetVertical, SwapPass::Vertical])?,
            [
                super::eight_cycle_offset_vertical_swap(),
                vec![
                    VectorFieldEntry(0, 1),
                    VectorFieldEntry(1, 0),
                    VectorFieldEntry(0, 1),
                    VectorFieldEntry(1, -1),
                    VectorFieldEntry(0, 0),
                    VectorFieldEntry(-1, 1),
                    VectorFieldEntry(0, -2),
                    VectorFieldEntry(-1, 0),
                    VectorFieldEntry(0, 0),
                ],
            ],
            vec![2, 1],
        )
    }

    #[test]
    fn offset_vertical_offset_horizontal() -> Result<(), Box<dyn Error>> {
        super::test_swap_pass_sequence(
            SwapPassSequence::from_passes([SwapPass::OffsetVertical, SwapPass::OffsetHorizontal])?,
            [
                super::eight_cycle_offset_vertical_swap(),
                super::eight_cycle_offset_vertical_offset_horizontal_swap(),
            ],
            vec![2, 2],
        )
    }
}

mod all_passes {
    use image_annealing::compute::conversion::VectorFieldEntry;
    use image_annealing::compute::SwapPassSequence;
    use std::error::Error;

    #[test]
    fn all() -> Result<(), Box<dyn Error>> {
        let second_permutation = super::eight_cycle_horizontal_vertical_swap();
        super::test_swap_pass_sequence(
            SwapPassSequence::all(),
            [
                super::eight_cycle_horizontal_swap(),
                second_permutation.clone(),
                second_permutation,
                vec![
                    VectorFieldEntry(2, 0),
                    VectorFieldEntry(0, 0),
                    VectorFieldEntry(0, 2),
                    VectorFieldEntry(0, 1),
                    VectorFieldEntry(0, 0),
                    VectorFieldEntry(-1, 1),
                    VectorFieldEntry(0, -2),
                    VectorFieldEntry(-1, -1),
                    VectorFieldEntry(0, -1),
                ],
            ],
            vec![2, 1, 0, 2],
        )
    }

    #[test]
    fn all_reverse() -> Result<(), Box<dyn Error>> {
        let third_permutation = vec![
            VectorFieldEntry(0, 1),
            VectorFieldEntry(1, 1),
            VectorFieldEntry(0, 0),
            VectorFieldEntry(1, -1),
            VectorFieldEntry(0, 0),
            VectorFieldEntry(-1, 1),
            VectorFieldEntry(0, -2),
            VectorFieldEntry(1, 0),
            VectorFieldEntry(-2, 0),
        ];
        super::test_swap_pass_sequence(
            SwapPassSequence::from_passes(SwapPassSequence::all().into_iter().rev())?,
            [
                super::eight_cycle_offset_vertical_swap(),
                super::eight_cycle_offset_vertical_offset_horizontal_swap(),
                third_permutation.clone(),
                third_permutation,
            ],
            vec![2, 2, 1, 0],
        )
    }
}

mod reject_out_of_bounds {
    use image_annealing::compute::conversion::{self, VectorFieldEntry};
    use image_annealing::compute::{self, Config, OutputStatus, SwapInput};
    use image_annealing::{CandidatePermutation, DisplacementGoal};
    use std::error::Error;
    use test_util::algorithm::{
        assert_correct_default_swap_full_output, assert_step_until_success,
    };
    use test_util::operation::{assert_correct_swap_count_output, SwapAcceptedCount};
    use test_util::permutation::DimensionsAndPermutation;

    fn run_test(
        width: usize,
        height: usize,
        displacement_goal_vector: &[VectorFieldEntry],
    ) -> Result<(), Box<dyn Error>> {
        let DimensionsAndPermutation {
            permutation,
            dimensions,
        } = test_util::permutation::identity_with_dimensions(width, height);
        let expected_permutation = permutation.clone();
        let displacement_goal = DisplacementGoal::from_vector_field(conversion::to_image(
            &dimensions,
            displacement_goal_vector,
        ))?;
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
            SwapAcceptedCount::None,
        );
        Ok(())
    }

    #[test]
    fn horizontal() -> Result<(), Box<dyn Error>> {
        run_test(
            3,
            1,
            &[
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
            &[
                VectorFieldEntry(0, 0),
                VectorFieldEntry(0, 0),
                VectorFieldEntry(0, 1),
            ],
        )
    }

    #[test]
    fn offset_horizontal() -> Result<(), Box<dyn Error>> {
        run_test(2, 1, &[VectorFieldEntry(-1, 0), VectorFieldEntry(1, 0)])
    }

    #[test]
    fn offset_vertical() -> Result<(), Box<dyn Error>> {
        run_test(1, 2, &[VectorFieldEntry(0, -1), VectorFieldEntry(0, 1)])
    }
}
