use image::{ImageBuffer, Rgba};
use image_annealing::compute::format::{
    ImageFormat, LosslessImage, Rgba16Image, Rgba16ImageBuffer, Rgba16ImageBufferComponent,
    Rgba16Rgba8Image, Rgba16Rgba8x2Image, Rgba16x2Image, Rgba8Image, Rgba8x2Image, Rgba8x3Image,
    Rgba8x4Image, VectorFieldImageBuffer, VectorFieldImageBufferComponent,
};
use image_annealing::ImageDimensions;

pub struct DimensionsAndRgbaBuffer<Component: 'static>
where
    image::Rgba<Component>: image::Pixel,
{
    pub image: ImageBuffer<
        image::Rgba<Component>,
        Vec<<image::Rgba<Component> as image::Pixel>::Subpixel>,
    >,
    pub dimensions: ImageDimensions,
}

pub type DimensionsAndRgba8Buffer = DimensionsAndRgbaBuffer<VectorFieldImageBufferComponent>;
pub type DimensionsAndRgba16Buffer = DimensionsAndRgbaBuffer<Rgba16ImageBufferComponent>;

pub fn coordinates_to_colors(dimensions: &ImageDimensions) -> Rgba16ImageBuffer {
    Rgba16ImageBuffer::from_fn(
        dimensions.width().try_into().unwrap(),
        dimensions.height().try_into().unwrap(),
        |x, y| {
            Rgba([
                x.try_into().unwrap(),
                (x + 1).try_into().unwrap(),
                y.try_into().unwrap(),
                (y + 1).try_into().unwrap(),
            ])
        },
    )
}

pub fn coordinates_to_zero_alpha_colors(dimensions: &ImageDimensions) -> Rgba16ImageBuffer {
    Rgba16ImageBuffer::from_fn(
        dimensions.width().try_into().unwrap(),
        dimensions.height().try_into().unwrap(),
        |x, y| {
            Rgba([
                x.try_into().unwrap(),
                (x + 1).try_into().unwrap(),
                y.try_into().unwrap(),
                0,
            ])
        },
    )
}

pub fn linear_indices_with_bias_to_colors<
    Bias: TryInto<usize> + std::fmt::Debug + Copy,
    Component: 'static + TryFrom<usize>,
>(
    bias: Bias,
) -> DimensionsAndRgbaBuffer<Component>
where
    <Bias as TryInto<usize>>::Error: std::fmt::Debug,
    <Component as TryFrom<usize>>::Error: std::fmt::Debug,
    image::Rgba<Component>: image::Pixel,
{
    let dimensions = ImageDimensions::try_new(2, 3).unwrap();
    let channel_bias = dimensions.count();
    let image = ImageBuffer::from_fn(
        dimensions.width().try_into().unwrap(),
        dimensions.height().try_into().unwrap(),
        |x, y| {
            let index = dimensions
                .make_linear_index(x, y)
                .unwrap()
                .checked_add(bias.try_into().unwrap())
                .unwrap();
            Rgba([
                index.try_into().unwrap(),
                index.checked_add(channel_bias).unwrap().try_into().unwrap(),
                index
                    .checked_add(channel_bias.checked_mul(2).unwrap())
                    .unwrap()
                    .try_into()
                    .unwrap(),
                index
                    .checked_add(channel_bias.checked_mul(3).unwrap())
                    .unwrap()
                    .try_into()
                    .unwrap(),
            ])
        },
    );
    DimensionsAndRgbaBuffer { image, dimensions }
}

pub fn nonzero_rgba8_colors() -> DimensionsAndRgba8Buffer {
    linear_indices_with_bias_to_colors(1)
}

pub struct LosslessImageAndByteSection {
    pub image: LosslessImage,
    pub byte_index: usize,
    pub byte_image: VectorFieldImageBuffer,
}

pub const IMAGE_COMPONENT_BYTE_COUNT: usize = 4;

pub fn lossless_image_with_nonzero_byte(
    format: ImageFormat,
    byte_index: usize,
) -> LosslessImageAndByteSection {
    assert!(byte_index < IMAGE_COMPONENT_BYTE_COUNT);
    let DimensionsAndRgba8Buffer { dimensions, image } = nonzero_rgba8_colors();
    let width = dimensions.width().try_into().unwrap();
    let height = dimensions.height().try_into().unwrap();

    let zero_rgba8 =
        image::RgbaImage::from_pixel(width, height, image::Rgba([0; IMAGE_COMPONENT_BYTE_COUNT]));
    let zero_rgba16 = Rgba16ImageBuffer::from_pixel(
        width,
        height,
        image::Rgba::<Rgba16ImageBufferComponent>([0; IMAGE_COMPONENT_BYTE_COUNT]),
    );

    let byte_image = image.clone();

    match format {
        ImageFormat::Rgba8 => match byte_index {
            0 => LosslessImageAndByteSection {
                image: LosslessImage::Rgba8(Rgba8Image::new(byte_image.clone()).unwrap()),
                byte_index,
                byte_image,
            },
            _ => LosslessImageAndByteSection {
                image: LosslessImage::Rgba8(Rgba8Image::new(zero_rgba8.clone()).unwrap()),
                byte_index,
                byte_image: zero_rgba8,
            },
        },
        ImageFormat::Rgba8x2 => match byte_index {
            0 => LosslessImageAndByteSection {
                image: LosslessImage::Rgba8x2(Rgba8x2Image::new(image, zero_rgba8).unwrap()),
                byte_index,
                byte_image,
            },
            1 => LosslessImageAndByteSection {
                image: LosslessImage::Rgba8x2(Rgba8x2Image::new(zero_rgba8, image).unwrap()),
                byte_index,
                byte_image,
            },
            _ => LosslessImageAndByteSection {
                image: LosslessImage::Rgba8x2(
                    Rgba8x2Image::new(zero_rgba8.clone(), zero_rgba8.clone()).unwrap(),
                ),
                byte_index,
                byte_image: zero_rgba8,
            },
        },
        ImageFormat::Rgba8x3 => match byte_index {
            0 => LosslessImageAndByteSection {
                image: LosslessImage::Rgba8x3(
                    Rgba8x3Image::new(image, zero_rgba8.clone(), zero_rgba8).unwrap(),
                ),
                byte_index,
                byte_image,
            },
            1 => LosslessImageAndByteSection {
                image: LosslessImage::Rgba8x3(
                    Rgba8x3Image::new(zero_rgba8.clone(), image, zero_rgba8).unwrap(),
                ),
                byte_index,
                byte_image,
            },
            2 => LosslessImageAndByteSection {
                image: LosslessImage::Rgba8x3(
                    Rgba8x3Image::new(zero_rgba8.clone(), zero_rgba8, image).unwrap(),
                ),
                byte_index,
                byte_image,
            },
            _ => LosslessImageAndByteSection {
                image: LosslessImage::Rgba8x3(
                    Rgba8x3Image::new(zero_rgba8.clone(), zero_rgba8.clone(), zero_rgba8.clone())
                        .unwrap(),
                ),
                byte_index,
                byte_image: zero_rgba8,
            },
        },
        ImageFormat::Rgba8x4 => match byte_index {
            0 => LosslessImageAndByteSection {
                image: LosslessImage::Rgba8x4(
                    Rgba8x4Image::new(image, zero_rgba8.clone(), zero_rgba8.clone(), zero_rgba8)
                        .unwrap(),
                ),
                byte_index,
                byte_image,
            },
            1 => LosslessImageAndByteSection {
                image: LosslessImage::Rgba8x4(
                    Rgba8x4Image::new(zero_rgba8.clone(), image, zero_rgba8.clone(), zero_rgba8)
                        .unwrap(),
                ),
                byte_index,
                byte_image,
            },
            2 => LosslessImageAndByteSection {
                image: LosslessImage::Rgba8x4(
                    Rgba8x4Image::new(zero_rgba8.clone(), zero_rgba8.clone(), image, zero_rgba8)
                        .unwrap(),
                ),
                byte_index,
                byte_image,
            },
            _ => LosslessImageAndByteSection {
                image: LosslessImage::Rgba8x4(
                    Rgba8x4Image::new(zero_rgba8.clone(), zero_rgba8.clone(), zero_rgba8, image)
                        .unwrap(),
                ),
                byte_index,
                byte_image,
            },
        },
        ImageFormat::Rgba16 => match byte_index {
            0 => LosslessImageAndByteSection {
                image: LosslessImage::Rgba16(
                    Rgba16Image::from_raw_pair(image, zero_rgba8).unwrap(),
                ),
                byte_index,
                byte_image,
            },
            1 => LosslessImageAndByteSection {
                image: LosslessImage::Rgba16(
                    Rgba16Image::from_raw_pair(zero_rgba8, image).unwrap(),
                ),
                byte_index,
                byte_image,
            },
            _ => LosslessImageAndByteSection {
                image: LosslessImage::Rgba16(Rgba16Image::new(zero_rgba16).unwrap()),
                byte_index,
                byte_image: zero_rgba8,
            },
        },
        ImageFormat::Rgba16x2 => match byte_index {
            0 => LosslessImageAndByteSection {
                image: LosslessImage::Rgba16x2(
                    Rgba16x2Image::new(
                        Rgba16Image::from_raw_pair(image, zero_rgba8)
                            .unwrap()
                            .into_inner(),
                        zero_rgba16,
                    )
                    .unwrap(),
                ),
                byte_index,
                byte_image,
            },
            1 => LosslessImageAndByteSection {
                image: LosslessImage::Rgba16x2(
                    Rgba16x2Image::new(
                        Rgba16Image::from_raw_pair(zero_rgba8, image)
                            .unwrap()
                            .into_inner(),
                        zero_rgba16,
                    )
                    .unwrap(),
                ),
                byte_index,
                byte_image,
            },
            2 => LosslessImageAndByteSection {
                image: LosslessImage::Rgba16x2(
                    Rgba16x2Image::new(
                        zero_rgba16,
                        Rgba16Image::from_raw_pair(image, zero_rgba8)
                            .unwrap()
                            .into_inner(),
                    )
                    .unwrap(),
                ),
                byte_index,
                byte_image,
            },
            _ => LosslessImageAndByteSection {
                image: LosslessImage::Rgba16x2(
                    Rgba16x2Image::new(
                        zero_rgba16,
                        Rgba16Image::from_raw_pair(zero_rgba8, image)
                            .unwrap()
                            .into_inner(),
                    )
                    .unwrap(),
                ),
                byte_index,
                byte_image,
            },
        },
        ImageFormat::Rgba16Rgba8 => match byte_index {
            0 => LosslessImageAndByteSection {
                image: LosslessImage::Rgba16Rgba8(
                    Rgba16Rgba8Image::new(
                        Rgba16Image::from_raw_pair(image, zero_rgba8.clone())
                            .unwrap()
                            .into_inner(),
                        zero_rgba8,
                    )
                    .unwrap(),
                ),
                byte_index,
                byte_image,
            },
            1 => LosslessImageAndByteSection {
                image: LosslessImage::Rgba16Rgba8(
                    Rgba16Rgba8Image::new(
                        Rgba16Image::from_raw_pair(zero_rgba8.clone(), image)
                            .unwrap()
                            .into_inner(),
                        zero_rgba8,
                    )
                    .unwrap(),
                ),
                byte_index,
                byte_image,
            },
            2 => LosslessImageAndByteSection {
                image: LosslessImage::Rgba16Rgba8(
                    Rgba16Rgba8Image::new(zero_rgba16, image).unwrap(),
                ),
                byte_index,
                byte_image,
            },
            _ => LosslessImageAndByteSection {
                image: LosslessImage::Rgba16Rgba8(
                    Rgba16Rgba8Image::new(zero_rgba16, zero_rgba8.clone()).unwrap(),
                ),
                byte_index,
                byte_image: zero_rgba8,
            },
        },
        ImageFormat::Rgba16Rgba8x2 => match byte_index {
            0 => LosslessImageAndByteSection {
                image: LosslessImage::Rgba16Rgba8x2(
                    Rgba16Rgba8x2Image::new(
                        Rgba16Image::from_raw_pair(image, zero_rgba8.clone())
                            .unwrap()
                            .into_inner(),
                        zero_rgba8.clone(),
                        zero_rgba8,
                    )
                    .unwrap(),
                ),
                byte_index,
                byte_image,
            },
            1 => LosslessImageAndByteSection {
                image: LosslessImage::Rgba16Rgba8x2(
                    Rgba16Rgba8x2Image::new(
                        Rgba16Image::from_raw_pair(zero_rgba8.clone(), image)
                            .unwrap()
                            .into_inner(),
                        zero_rgba8.clone(),
                        zero_rgba8,
                    )
                    .unwrap(),
                ),
                byte_index,
                byte_image,
            },
            2 => LosslessImageAndByteSection {
                image: LosslessImage::Rgba16Rgba8x2(
                    Rgba16Rgba8x2Image::new(zero_rgba16, image, zero_rgba8).unwrap(),
                ),
                byte_index,
                byte_image,
            },
            _ => LosslessImageAndByteSection {
                image: LosslessImage::Rgba16Rgba8x2(
                    Rgba16Rgba8x2Image::new(zero_rgba16, zero_rgba8, image).unwrap(),
                ),
                byte_index,
                byte_image,
            },
        },
    }
}

pub fn all_lossless_images_with_nonzero_bytes() -> impl Iterator<Item = LosslessImageAndByteSection>
{
    [
        ImageFormat::Rgba8,
        ImageFormat::Rgba8x2,
        ImageFormat::Rgba8x3,
        ImageFormat::Rgba8x4,
        ImageFormat::Rgba16,
        ImageFormat::Rgba16x2,
        ImageFormat::Rgba16Rgba8,
        ImageFormat::Rgba16Rgba8x2,
    ]
    .iter()
    .flat_map(|format| {
        (0..IMAGE_COMPONENT_BYTE_COUNT)
            .map(|byte_index| lossless_image_with_nonzero_byte(*format, byte_index))
    })
}
