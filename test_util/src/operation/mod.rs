use image_annealing::compute::conversion::VectorFieldEntry;
use image_annealing::compute::format::VectorFieldImageBuffer;
use image_annealing::compute::{SwapAlgorithm, SwapParameters, SwapPartialOutput};
use image_annealing::ImageDimensions;

pub fn swap(vector_field: &VectorFieldImageBuffer) -> VectorFieldImageBuffer {
    let width = vector_field.width();
    let width_less_one = width
        .checked_sub(1u32)
        .expect("Vector field width is zero.");
    VectorFieldImageBuffer::from_fn(width, vector_field.height(), |x, y| {
        if x % 2 == 0 {
            if x < width_less_one {
                let input_pixel = vector_field.get_pixel(x + 1, y);
                let mut entry = VectorFieldEntry::from_pixel(input_pixel);
                entry.0 += 1i16;
                entry.to_pixel()
            } else {
                *vector_field.get_pixel(x, y)
            }
        } else {
            let input_pixel = vector_field.get_pixel(x - 1, y);
            let mut entry = VectorFieldEntry::from_pixel(input_pixel);
            entry.0 -= 1i16;
            entry.to_pixel()
        }
    })
}

pub enum SwapAcceptedCount {
    None,
    Some(Vec<usize>),
    All,
}

fn assert_correct_swap_count_output_inner(
    output: Option<SwapPartialOutput>,
    parameters: &SwapParameters,
    image_dimensions: &ImageDimensions,
    swaps_accepted: SwapAcceptedCount,
) {
    assert_eq!(output.is_some(), parameters.count_swap);
    if let Some(SwapPartialOutput { counts }) = output {
        let is_none_accepted = match swaps_accepted {
            SwapAcceptedCount::None => true,
            SwapAcceptedCount::Some(ref v) => v.iter().sum::<usize>() == 0,
            SwapAcceptedCount::All => false,
        };
        assert_eq!(counts.is_none_accepted(), is_none_accepted);

        assert_eq!(counts.passes().count(), parameters.sequence.iter().count());
        if let SwapAcceptedCount::Some(v) = &swaps_accepted {
            assert_eq!(counts.passes().count(), v.len());
        }

        let (total, accepted) = parameters
            .sequence
            .iter()
            .zip(counts.passes())
            .enumerate()
            .fold((0_usize, 0_usize), |mut acc, (i, (&pass, pass_data))| {
                assert_eq!(pass, pass_data.pass());
                let total_i = pass.total_swaps(image_dimensions);
                assert_eq!(pass_data.total(), total_i);
                let accepted_i = match &swaps_accepted {
                    SwapAcceptedCount::None => 0,
                    SwapAcceptedCount::Some(v) => v[i],
                    SwapAcceptedCount::All => total_i,
                };
                assert_eq!(pass_data.accepted(), accepted_i);
                assert_eq!(pass_data.is_none_accepted(), accepted_i == 0);
                assert_eq!(
                    pass_data.accepted_fraction(),
                    if total_i == 0 {
                        0.0
                    } else {
                        accepted_i as f64 / total_i as f64
                    }
                );

                acc.0 = acc.0.checked_add(total_i).unwrap();
                acc.1 = acc.1.checked_add(accepted_i).unwrap();
                acc
            });

        assert_eq!(counts.total(), total);
        assert_eq!(counts.accepted(), accepted);
        assert_eq!(
            counts.accepted_fraction(),
            if total == 0 {
                0.0
            } else {
                accepted as f64 / total as f64
            }
        );
    }
}

pub fn assert_correct_swap_count_output(
    algorithm: &mut SwapAlgorithm,
    parameters: &SwapParameters,
    image_dimensions: &ImageDimensions,
    swaps_accepted: SwapAcceptedCount,
) {
    let output = algorithm.partial_output_block();
    assert_correct_swap_count_output_inner(output, parameters, image_dimensions, swaps_accepted);
    assert!(algorithm.partial_output_block().is_none());
}

pub async fn assert_correct_swap_count_output_async(
    algorithm: &mut SwapAlgorithm,
    parameters: &SwapParameters,
    image_dimensions: &ImageDimensions,
    swaps_accepted: SwapAcceptedCount,
) {
    let output = algorithm.partial_output().await;
    assert_correct_swap_count_output_inner(output, parameters, image_dimensions, swaps_accepted);
    assert!(algorithm.partial_output().await.is_none());
}
