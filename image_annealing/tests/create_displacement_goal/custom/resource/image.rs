use image_annealing::compute::{
    self, Config, CreateDisplacementGoalInput, CreateDisplacementGoalPipelineOperation,
    OutputStatus, PipelineConfig,
};
use image_annealing::{DisplacementGoal, ImageDimensionsHolder};
use std::borrow::Cow;
use std::default::Default;
use std::error::Error;
use test_util::algorithm::assert_step_until_success;
use test_util::image::LosslessImageAndByteSection;

fn copy_image_inner(image_data: LosslessImageAndByteSection) -> Result<(), Box<dyn Error>> {
    let dimensions = image_data.image.dimensions();
    let expected_displacement_goal = DisplacementGoal::from_vector_field(image_data.byte_image)?;
    let expected_image = image_data.image.clone();

    let mut parameters = test_util::shader::make_create_displacement_goal_parameters("copy_image");
    match parameters.pipeline_operation {
        CreateDisplacementGoalPipelineOperation::Set(PipelineConfig {
            ref mut shader_config,
            ..
        }) => {
            let body = &mut shader_config.content.body;
            let new_body = body.replacen(
                "byte_index = 0u",
                &format!("byte_index = {}u", image_data.byte_index),
                1,
            );
            *body = Cow::Owned(new_body);
        }
        _ => unreachable!(),
    }

    let dispatcher = compute::create_dispatcher_block(&Config {
        image_dimensions: *dimensions,
    })?;
    let mut algorithm = dispatcher.create_displacement_goal(
        CreateDisplacementGoalInput {
            image: Some(image_data.image),
            ..Default::default()
        },
        &parameters,
    );
    assert_step_until_success(algorithm.as_mut(), OutputStatus::FinalFullOutput)?;

    let output = algorithm.full_output_block().unwrap();
    assert!(output.input_displacement_goal.is_none());
    assert!(output.permutation.is_none());
    assert_eq!(output.image.unwrap(), expected_image);
    assert_eq!(output.output_displacement_goal, expected_displacement_goal);
    assert!(algorithm.full_output_block().is_none());
    Ok(())
}

#[test]
fn copy_image() -> Result<(), Box<dyn Error>> {
    for image_data in test_util::image::all_lossless_images_with_nonzero_bytes() {
        copy_image_inner(image_data)?;
    }
    Ok(())
}
