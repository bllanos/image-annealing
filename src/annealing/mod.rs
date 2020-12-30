use crate::image_utils::rectangle::Rectangle;
use image::DynamicImage;
use image::GenericImageView;
use image::ImageBuffer;
use std::error::Error;
use std::fmt;

mod compute;

#[derive(Debug, Clone)]
pub struct InvalidRegionError;

impl fmt::Display for InvalidRegionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "region is not enclosed by the image boundaries")
    }
}

impl Error for InvalidRegionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

type TextureComponent = u16;
const N_TEXTURE_COMPONENTS: usize = 4;
type TextureImageBuffer = ImageBuffer<image::Rgba<TextureComponent>, Vec<TextureComponent>>;

pub fn prepare_image(
    img: &DynamicImage,
    region: Option<&Rectangle>,
) -> Result<TextureImageBuffer, InvalidRegionError> {
    if let Some(rect) = region {
        let image_rect = Rectangle::from_dimensions(img.dimensions());
        if image_rect.encloses(&rect) {
            Ok(img
                .crop_imm(rect.x(), rect.y(), rect.width(), rect.height())
                .to_rgba16())
        } else {
            Err(InvalidRegionError)
        }
    } else {
        Ok(img.to_rgba16())
    }
}

pub fn process_image(img: &TextureImageBuffer) -> Result<TextureImageBuffer, Box<dyn Error>> {
    let output = futures::executor::block_on(compute::execute_gpu(img, vec![5, 23, 10, 9]))?;
    println!("Output: {:?}", output.1);
    Ok(output.0)
}

// The module could also be implemented in this file
#[cfg(test)]
mod tests;
