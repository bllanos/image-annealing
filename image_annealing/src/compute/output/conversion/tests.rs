use super::super::format::{VectorFieldImageBuffer, VectorFieldImageBufferComponent};
use super::VectorFieldEntry;
use crate::image_utils::ImageDimensions;
use image::Rgba;
use std::error::Error;

fn make_pixels(x: u32, y: u32) -> Rgba<VectorFieldImageBufferComponent> {
    match x {
        0 => match y {
            0 => Rgba([0, 0, 0, 0]),
            1 => Rgba([0, 0, 0, 1]),
            2 => Rgba([0, 0, 1, 0]),
            3 => Rgba([0, 1, 0, 0]),
            4 => Rgba([1, 0, 0, 0]),
            _ => panic!(),
        },
        1 => match y {
            0 => Rgba([1, 1, 1, 1]),
            1 => Rgba([0, 0, 0, 255]),
            2 => Rgba([0, 0, 255, 0]),
            3 => Rgba([0, 255, 0, 0]),
            4 => Rgba([255, 0, 0, 0]),
            _ => panic!(),
        },
        2 => match y {
            0 => Rgba([255, 255, 255, 255]),
            1 => Rgba([0, 0, 1, 255]),
            2 => Rgba([0, 0, 255, 1]),
            3 => Rgba([1, 255, 0, 0]),
            4 => Rgba([255, 1, 0, 0]),
            _ => panic!(),
        },
        _ => panic!(),
    }
}

fn make_vector() -> Vec<VectorFieldEntry> {
    vec![
        VectorFieldEntry(0, 0),
        VectorFieldEntry(257, 257),
        VectorFieldEntry(-1, -1),
        VectorFieldEntry(0, 1),
        VectorFieldEntry(0, 255),
        VectorFieldEntry(0, 511),
        VectorFieldEntry(0, 256),
        VectorFieldEntry(0, -256),
        VectorFieldEntry(0, -255),
        VectorFieldEntry(1, 0),
        VectorFieldEntry(255, 0),
        VectorFieldEntry(511, 0),
        VectorFieldEntry(256, 0),
        VectorFieldEntry(-256, 0),
        VectorFieldEntry(-255, 0),
    ]
}

#[test]
fn image_to_vec() {
    let image = VectorFieldImageBuffer::from_fn(3, 5, make_pixels);
    let v = super::to_vec(&image);
    assert_eq!(v, make_vector());
}

#[test]
fn vec_to_image() -> Result<(), Box<dyn Error>> {
    let image = super::to_image(&ImageDimensions::new(3, 5)?, &make_vector());
    for (x, y, px) in image.enumerate_pixels() {
        assert_eq!(*px, make_pixels(x, y));
    }
    Ok(())
}

#[test]
#[should_panic(expected = "vector length and image dimensions are incompatible")]
fn vec_to_image_impossible() {
    super::to_image(
        &ImageDimensions::new(3, 5).unwrap(),
        &[
            VectorFieldEntry(0, 0),
            VectorFieldEntry(257, 257),
            VectorFieldEntry(-1, -1),
            VectorFieldEntry(0, 1),
        ],
    );
}
