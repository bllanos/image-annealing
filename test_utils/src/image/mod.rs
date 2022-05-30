use image::{ImageBuffer, Rgba};
use image_annealing::compute::format::{
    Rgba16ImageBuffer, Rgba16ImageBufferComponent, VectorFieldImageBufferComponent,
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
    let dimensions = ImageDimensions::new(2, 3).unwrap();
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
