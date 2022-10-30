use image_annealing::compute::{self, Config, OutputStatus, ValidatePermutationInput};
use image_annealing::{CandidatePermutation, ImageDimensions};
use std::error::Error;
use test_utils::algorithm::{
    assert_step_until_error, assert_step_until_success, assert_step_until_success_async,
};
use test_utils::permutation::{self, DimensionsAndPermutation};

#[test]
fn run_once_identity() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = permutation::identity();
    let expected = permutation.clone();
    let dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm = dispatcher.validate_permutation(
        ValidatePermutationInput {
            candidate_permutation: CandidatePermutation::new(permutation)?,
        },
        &Default::default(),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;
    let output = algorithm.full_output_block().unwrap().validated_permutation;
    assert_eq!(*output.as_ref(), expected);
    assert!(algorithm.full_output_block().is_none());
    Ok(())
}

async fn run_once_identity_async_inner() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = permutation::identity();
    let expected = permutation.clone();
    let dispatcher = compute::create_dispatcher(&Config {
        image_dimensions: dimensions,
    })
    .await?;
    let mut algorithm = dispatcher.validate_permutation(
        ValidatePermutationInput {
            candidate_permutation: CandidatePermutation::new(permutation)?,
        },
        &Default::default(),
    );
    assert_step_until_success_async(algorithm.as_mut(), OutputStatus::FinalFullOutput).await?;
    let output = algorithm.full_output().await.unwrap().validated_permutation;
    assert_eq!(*output.as_ref(), expected);
    assert!(algorithm.full_output().await.is_none());
    Ok(())
}

#[test]
fn run_once_identity_async() -> Result<(), Box<dyn Error>> {
    futures::executor::block_on(run_once_identity_async_inner())
}

#[test]
fn run_twice_invalid_valid() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = permutation::non_identity();
    let expected = permutation.clone();
    let DimensionsAndPermutation {
        permutation: invalid_image,
        dimensions: other_dimensions,
    } = permutation::duplicate();
    assert_eq!(dimensions, other_dimensions);

    let mut dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: dimensions,
    })?;
    let mut algorithm = dispatcher.validate_permutation(
        ValidatePermutationInput {
            candidate_permutation: CandidatePermutation::new(invalid_image)?,
        },
        &Default::default(),
    );
    assert_step_until_error(algorithm.as_mut(), OutputStatus::FinalFullOutput, "entries (x, y, delta_x, delta_y) = (0, 0, 0, 1) and (x, y, delta_x, delta_y) = (0, 2, 0, -1) both map to location (x, y) = (0, 1)");

    dispatcher = algorithm.return_to_dispatcher();
    algorithm = dispatcher.validate_permutation(
        ValidatePermutationInput {
            candidate_permutation: CandidatePermutation::new(permutation)?,
        },
        &Default::default(),
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;
    let output = algorithm.full_output_block().unwrap().validated_permutation;
    assert_eq!(*output.as_ref(), expected);
    Ok(())
}

#[test]
fn invalid_dimensions() -> Result<(), Box<dyn Error>> {
    let DimensionsAndPermutation {
        permutation,
        dimensions,
    } = permutation::non_identity();
    let invalid_dimensions =
        ImageDimensions::try_new(dimensions.width() + 1, dimensions.height()).unwrap();

    let dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: invalid_dimensions,
    })?;
    let mut algorithm = dispatcher.validate_permutation(
        ValidatePermutationInput {
            candidate_permutation: CandidatePermutation::new(permutation)?,
        },
        &Default::default(),
    );
    assert_step_until_error(
        algorithm.as_mut(),
        OutputStatus::FinalFullOutput,
        "mismatch in image dimensions, (width, height) = (3, 3) and (width, height) = (2, 3)",
    );
    Ok(())
}
