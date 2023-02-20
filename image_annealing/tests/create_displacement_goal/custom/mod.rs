use image_annealing::compute::format::VectorFieldImageBuffer;
use image_annealing::{DisplacementGoal, ImageDimensions};

mod parameters;
mod pipeline_operation;
mod resource;

const CUSTOM_SHADER_ENTRY_POINT: &str = "entry_point";

fn make_filled_rectangle_displacement_goal(
    image_dimensions: &ImageDimensions,
    rectangle_dimensions: &ImageDimensions,
) -> DisplacementGoal {
    let mut image = VectorFieldImageBuffer::from_pixel(
        image_dimensions.width().try_into().unwrap(),
        image_dimensions.height().try_into().unwrap(),
        image::Rgba([0; 4]),
    );
    let rectangle_width = rectangle_dimensions.width().try_into().unwrap();
    let rectangle_height = rectangle_dimensions.height().try_into().unwrap();
    for (x, y, px) in image.enumerate_pixels_mut() {
        if x < rectangle_width && y < rectangle_height {
            *px = image::Rgba([1, 2, 3, 4]);
        }
    }
    DisplacementGoal::from_vector_field(image).unwrap()
}
