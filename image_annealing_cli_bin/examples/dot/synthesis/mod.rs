//! # Example data synthesis
//!
//! Input data generation functions for the 'dot' example

use image_annealing::compute::conversion::{VectorFieldEntry, VectorFieldEntryComponent};
use image_annealing::compute::format::Rgba8Image;
use image_annealing::{DisplacementGoal, ImageDimensions};

/// The geometrical parameters of a dot (a disc)
#[derive(Debug, PartialEq)]
struct DotGeometry {
    /// Squared radius of the dot
    radius_squared: f64,
    /// x-coordinate of the center of the dot
    x: f64,
    /// y-coordinate of the center of the dot
    y: f64,
}

impl DotGeometry {
    /// Find the parameters of a dot centered in an image with the given dimensions
    ///
    /// The dot just fits inside the image borders.
    /// The radius of the dot is the floor of half the smaller of the image width and height.
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

/// Create an image of a white dot on a black background
///
/// The dot's geometrical parameters are created using [`DotGeometry::new(dimensions)`](DotGeometry::new)
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

/// Create a displacement goal where a disc-shaped region in the center of the image
/// wants to move directly away from the center, whereas the surrounding regions
/// want to move towards the center
///
/// The disc's geometrical parameters are created using [`DotGeometry::new(dimensions)`](DotGeometry::new)
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
                // Pixel is outside the disc
                VectorFieldEntry(-vector_from_center_integer.0, -vector_from_center_integer.1)
                    .to_pixel()
            } else {
                // Pixel is inside the disc
                //
                // Create a vector that points as far as possible away from the center of the image
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
                    // Scale the vector such that it points as far as possible away from the center
                    // of the image
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
