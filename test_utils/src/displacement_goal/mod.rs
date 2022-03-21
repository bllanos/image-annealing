use image_annealing::compute::conversion::{self, VectorFieldEntry};
use image_annealing::{DisplacementGoal, ImageDimensions};

pub fn identity(dimensions: &ImageDimensions) -> DisplacementGoal {
    let v = vec![VectorFieldEntry(0, 0); dimensions.count()];
    DisplacementGoal::from_vector_field(conversion::to_image(dimensions, &v)).unwrap()
}
