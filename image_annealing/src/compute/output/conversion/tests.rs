use super::super::format::{VectorFieldImageBuffer, VectorFieldImageBufferComponent};
use super::VectorFieldEntry;
use crate::ImageDimensions;
use image::Rgba;
use std::error::Error;

mod vector_field_entry {
    use super::super::VectorFieldEntry;
    use crate::ImageDimensions;
    use std::error::Error;

    #[test]
    fn identity() {
        assert_eq!(VectorFieldEntry::identity(), VectorFieldEntry(0, 0));
    }

    #[test]
    fn from_pixel() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::try_new(3, 5)?;
        let vector = super::make_vector();
        for (k, entry) in vector.iter().enumerate() {
            let (x, y) = dimensions.make_coordinates(k)?;
            assert_eq!(
                &VectorFieldEntry::from_pixel(&super::make_pixels(x.try_into()?, y.try_into()?)),
                entry
            );
        }
        Ok(())
    }

    #[test]
    fn to_pixel() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::try_new(3, 5)?;
        let vector = super::make_vector();
        for (k, entry) in vector.iter().enumerate() {
            let (x, y) = dimensions.make_coordinates(k)?;
            assert_eq!(
                super::make_pixels(x.try_into()?, y.try_into()?),
                VectorFieldEntry::to_pixel(entry)
            );
        }
        Ok(())
    }
}

fn make_pixels(x: u32, y: u32) -> Rgba<VectorFieldImageBufferComponent> {
    match x {
        0 => match y {
            0 => Rgba([0, 0, 0, 0]),
            1 => Rgba([0, 0, 0, 1]),
            2 => Rgba([0, 0, 1, 0]),
            3 => Rgba([0, 1, 0, 0]),
            4 => Rgba([1, 0, 0, 0]),
            _ => unreachable!(),
        },
        1 => match y {
            0 => Rgba([1, 1, 1, 1]),
            1 => Rgba([0, 0, 0, 255]),
            2 => Rgba([0, 0, 255, 0]),
            3 => Rgba([0, 255, 0, 0]),
            4 => Rgba([255, 0, 0, 0]),
            _ => unreachable!(),
        },
        2 => match y {
            0 => Rgba([255, 255, 255, 255]),
            1 => Rgba([0, 0, 1, 255]),
            2 => Rgba([0, 0, 255, 1]),
            3 => Rgba([1, 255, 0, 0]),
            4 => Rgba([255, 1, 0, 0]),
            _ => unreachable!(),
        },
        _ => unreachable!(),
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
    let image = super::to_image(&ImageDimensions::try_new(3, 5)?, &make_vector());
    for (x, y, px) in image.enumerate_pixels() {
        assert_eq!(*px, make_pixels(x, y));
    }
    Ok(())
}

#[test]
#[should_panic(expected = "vector length and image dimensions are incompatible")]
fn vec_to_image_impossible() {
    super::to_image(
        &ImageDimensions::try_new(3, 5).unwrap(),
        &[
            VectorFieldEntry(0, 0),
            VectorFieldEntry(257, 257),
            VectorFieldEntry(-1, -1),
            VectorFieldEntry(0, 1),
        ],
    );
}
