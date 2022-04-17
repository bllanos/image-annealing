use image_annealing::compute::format::{
    LosslessImage, Rgba16Image, Rgba16Rgba8Image, Rgba16Rgba8x2Image, Rgba16x2Image, Rgba8Image,
    Rgba8x2Image, Rgba8x3Image, Rgba8x4Image,
};
use image_annealing::compute::{self, Config, OutputStatus, PermuteInput};
use image_annealing::{CandidatePermutation, ImageDimensionsHolder};
use std::default::Default;
use std::error::Error;
use test_utils::algorithm::assert_step_until_success;
use test_utils::image::{DimensionsAndRgba16Buffer, DimensionsAndRgba8Buffer};
use test_utils::permutation::DimensionsAndPermutation;

fn permute_lossless_image(original_image: LosslessImage) -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = test_utils::permutation::non_identity();
    assert_eq!(&dimensions, original_image.dimensions());
    let expected_permutation = permutation.clone();
    let permuted_image = match original_image {
        LosslessImage::Rgba8(ref image) => LosslessImage::Rgba8(Rgba8Image::new(
            test_utils::permutation::non_identity_forward_permute(image.as_ref()),
        )?),
        LosslessImage::Rgba8x2(ref image) => LosslessImage::Rgba8x2(Rgba8x2Image::new(
            test_utils::permutation::non_identity_forward_permute(image.first_inner()),
            test_utils::permutation::non_identity_forward_permute(image.second_inner()),
        )?),
        LosslessImage::Rgba8x3(ref image) => LosslessImage::Rgba8x3(Rgba8x3Image::new(
            test_utils::permutation::non_identity_forward_permute(image.first_inner()),
            test_utils::permutation::non_identity_forward_permute(image.second_inner()),
            test_utils::permutation::non_identity_forward_permute(image.third_inner()),
        )?),
        LosslessImage::Rgba8x4(ref image) => LosslessImage::Rgba8x4(Rgba8x4Image::new(
            test_utils::permutation::non_identity_forward_permute(image.first_inner()),
            test_utils::permutation::non_identity_forward_permute(image.second_inner()),
            test_utils::permutation::non_identity_forward_permute(image.third_inner()),
            test_utils::permutation::non_identity_forward_permute(image.fourth_inner()),
        )?),
        LosslessImage::Rgba16(ref image) => LosslessImage::Rgba16(Rgba16Image::new(
            test_utils::permutation::non_identity_forward_permute(image.as_ref()),
        )?),
        LosslessImage::Rgba16x2(ref image) => LosslessImage::Rgba16x2(Rgba16x2Image::new(
            test_utils::permutation::non_identity_forward_permute(image.first_inner()),
            test_utils::permutation::non_identity_forward_permute(image.second_inner()),
        )?),
        LosslessImage::Rgba16Rgba8(ref image) => LosslessImage::Rgba16Rgba8(Rgba16Rgba8Image::new(
            test_utils::permutation::non_identity_forward_permute(image.first_inner()),
            test_utils::permutation::non_identity_forward_permute(image.second_inner()),
        )?),
        LosslessImage::Rgba16Rgba8x2(ref image) => {
            LosslessImage::Rgba16Rgba8x2(Rgba16Rgba8x2Image::new(
                test_utils::permutation::non_identity_forward_permute(image.first_inner()),
                test_utils::permutation::non_identity_forward_permute(image.second_inner()),
                test_utils::permutation::non_identity_forward_permute(image.third_inner()),
            )?)
        }
    };

    let dispatcher = compute::create_dispatcher(&Config {
        image_dimensions: dimensions,
    })?;

    let mut algorithm = dispatcher.permute(
        PermuteInput {
            candidate_permutation: Some(CandidatePermutation::new(permutation)?),
            original_image: Some(original_image.clone()),
        },
        &Default::default(),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output().unwrap();
    assert_eq!(*output.permutation.unwrap().as_ref(), expected_permutation);
    assert_eq!(output.original_image.unwrap(), original_image);
    assert_eq!(output.permuted_image, permuted_image);

    Ok(())
}

#[test]
fn permute_rgba8() -> Result<(), Box<dyn Error>> {
    let DimensionsAndRgba8Buffer { image, .. } =
        test_utils::image::linear_indices_with_bias_to_colors(0);
    permute_lossless_image(LosslessImage::Rgba8(Rgba8Image::new(image)?))
}

#[test]
fn permute_rgba8x2() -> Result<(), Box<dyn Error>> {
    let DimensionsAndRgba8Buffer { image: image1, .. } =
        test_utils::image::linear_indices_with_bias_to_colors(0);
    let DimensionsAndRgba8Buffer { image: image2, .. } =
        test_utils::image::linear_indices_with_bias_to_colors(*image1.last().unwrap());
    permute_lossless_image(LosslessImage::Rgba8x2(Rgba8x2Image::new(image1, image2)?))
}

#[test]
fn permute_rgba8x3() -> Result<(), Box<dyn Error>> {
    let DimensionsAndRgba8Buffer { image: image1, .. } =
        test_utils::image::linear_indices_with_bias_to_colors(0);
    let DimensionsAndRgba8Buffer { image: image2, .. } =
        test_utils::image::linear_indices_with_bias_to_colors(*image1.last().unwrap());
    let DimensionsAndRgba8Buffer { image: image3, .. } =
        test_utils::image::linear_indices_with_bias_to_colors(*image2.last().unwrap());
    permute_lossless_image(LosslessImage::Rgba8x3(Rgba8x3Image::new(
        image1, image2, image3,
    )?))
}

#[test]
fn permute_rgba8x4() -> Result<(), Box<dyn Error>> {
    let DimensionsAndRgba8Buffer { image: image1, .. } =
        test_utils::image::linear_indices_with_bias_to_colors(0);
    let DimensionsAndRgba8Buffer { image: image2, .. } =
        test_utils::image::linear_indices_with_bias_to_colors(*image1.last().unwrap());
    let DimensionsAndRgba8Buffer { image: image3, .. } =
        test_utils::image::linear_indices_with_bias_to_colors(*image2.last().unwrap());
    let DimensionsAndRgba8Buffer { image: image4, .. } =
        test_utils::image::linear_indices_with_bias_to_colors(*image3.last().unwrap());
    permute_lossless_image(LosslessImage::Rgba8x4(Rgba8x4Image::new(
        image1, image2, image3, image4,
    )?))
}

#[test]
fn permute_rgba16() -> Result<(), Box<dyn Error>> {
    let DimensionsAndRgba16Buffer { image, .. } =
        test_utils::image::linear_indices_with_bias_to_colors(0);
    permute_lossless_image(LosslessImage::Rgba16(Rgba16Image::new(image)?))
}

#[test]
fn permute_rgba16x2() -> Result<(), Box<dyn Error>> {
    let DimensionsAndRgba16Buffer { image: image1, .. } =
        test_utils::image::linear_indices_with_bias_to_colors(0);
    let DimensionsAndRgba16Buffer { image: image2, .. } =
        test_utils::image::linear_indices_with_bias_to_colors(*image1.last().unwrap());
    permute_lossless_image(LosslessImage::Rgba16x2(Rgba16x2Image::new(image1, image2)?))
}

#[test]
fn permute_rgba16_rgba8() -> Result<(), Box<dyn Error>> {
    let DimensionsAndRgba16Buffer { image: image1, .. } =
        test_utils::image::linear_indices_with_bias_to_colors(0);
    let DimensionsAndRgba8Buffer { image: image2, .. } =
        test_utils::image::linear_indices_with_bias_to_colors(*image1.last().unwrap());
    permute_lossless_image(LosslessImage::Rgba16Rgba8(Rgba16Rgba8Image::new(
        image1, image2,
    )?))
}

#[test]
fn permute_rgba16_rgba8x2() -> Result<(), Box<dyn Error>> {
    let DimensionsAndRgba16Buffer { image: image1, .. } =
        test_utils::image::linear_indices_with_bias_to_colors(0);
    let DimensionsAndRgba8Buffer { image: image2, .. } =
        test_utils::image::linear_indices_with_bias_to_colors(*image1.last().unwrap());
    let DimensionsAndRgba8Buffer { image: image3, .. } =
        test_utils::image::linear_indices_with_bias_to_colors(*image2.last().unwrap());
    permute_lossless_image(LosslessImage::Rgba16Rgba8x2(Rgba16Rgba8x2Image::new(
        image1, image2, image3,
    )?))
}
