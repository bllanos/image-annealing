use image_annealing::compute::conversion::{VectorFieldEntry, VectorFieldEntryComponent};
use image_annealing::compute::format::Rgba8Image;
use image_annealing::{DisplacementGoal, ImageDimensions};

#[derive(Debug, PartialEq)]
struct DotGeometry {
    radius_squared: f64,
    x: f64,
    y: f64,
}

impl DotGeometry {
    pub fn new(dimensions: &ImageDimensions) -> Self {
        let min_dimension = std::cmp::min(dimensions.width(), dimensions.height());
        let radius = min_dimension / 2;
        Self {
            radius_squared: (radius * radius) as f64,
            x: dimensions.width() as f64 / 2.0,
            y: dimensions.height() as f64 / 2.0,
        }
    }
}

pub fn white_dot(dimensions: &ImageDimensions) -> Rgba8Image {
    let DotGeometry {
        radius_squared,
        x: dot_x,
        y: dot_y,
    } = DotGeometry::new(dimensions);

    let image = image::RgbaImage::from_fn(
        dimensions.width().try_into().unwrap(),
        dimensions.height().try_into().unwrap(),
        |x_integer, y_integer| {
            let x = x_integer as f64 + 0.5;
            let y = y_integer as f64 + 0.5;
            let distance_squared = (x - dot_x).powi(2) + (y - dot_y).powi(2);
            if distance_squared > radius_squared {
                image::Rgba([0, 0, 0, 255])
            } else {
                image::Rgba([255, 255, 255, 255])
            }
        },
    );

    Rgba8Image::new(image).unwrap()
}

pub fn dot_goal(dimensions: &ImageDimensions) -> DisplacementGoal {
    let DotGeometry {
        radius_squared,
        x: dot_x,
        y: dot_y,
    } = DotGeometry::new(dimensions);

    let image = image::RgbaImage::from_fn(
        dimensions.width().try_into().unwrap(),
        dimensions.height().try_into().unwrap(),
        |x_integer, y_integer| {
            let x = x_integer as f64 + 0.5;
            let y = y_integer as f64 + 0.5;
            let vector_from_center = (x - dot_x, y - dot_y);
            let vector_from_center_integer = (
                vector_from_center.0.round() as VectorFieldEntryComponent,
                vector_from_center.1.round() as VectorFieldEntryComponent,
            );
            let distance_squared = vector_from_center.0.powi(2) + vector_from_center.1.powi(2);
            if distance_squared > radius_squared {
                VectorFieldEntry(-vector_from_center_integer.0, -vector_from_center_integer.1)
                    .to_pixel()
            } else {
                let max_component = std::cmp::max(
                    vector_from_center_integer.0.abs(),
                    vector_from_center_integer.1.abs(),
                );
                let entry = if max_component == 0 {
                    VectorFieldEntry(
                        VectorFieldEntryComponent::MAX,
                        VectorFieldEntryComponent::MAX,
                    )
                } else {
                    let scale = VectorFieldEntryComponent::MAX / max_component;
                    VectorFieldEntry(
                        scale * vector_from_center_integer.0,
                        scale * vector_from_center_integer.1,
                    )
                };
                entry.to_pixel()
            }
        },
    );

    DisplacementGoal::from_vector_field(image).unwrap()
}

#[cfg(test)]
mod tests;
