use image::ImageBuffer;
use image_annealing::compute::conversion::{self, VectorFieldEntry};
use image_annealing::compute::format::{Rgba16ImageBuffer, VectorFieldImageBuffer};
use image_annealing::{ImageDimensions, ImageDimensionsHolder, ValidatedPermutation};
use std::convert::TryInto;

pub struct DimensionsAndPermutation {
    pub permutation: VectorFieldImageBuffer,
    pub dimensions: ImageDimensions,
}

pub fn identity_with_dimensions<T>(width: T, height: T) -> DimensionsAndPermutation
where
    T: TryInto<usize> + std::fmt::Debug + std::fmt::Display + Copy,
{
    let dimensions = ImageDimensions::new(width, height).unwrap();
    let v = vec![VectorFieldEntry(0, 0); dimensions.count()];
    DimensionsAndPermutation {
        permutation: conversion::to_image(&dimensions, &v),
        dimensions,
    }
}

pub fn identity() -> DimensionsAndPermutation {
    identity_with_dimensions(2, 3)
}

pub fn identity_permute(image: &Rgba16ImageBuffer) -> Rgba16ImageBuffer {
    image.clone()
}

pub fn duplicate() -> DimensionsAndPermutation {
    let dimensions = ImageDimensions::new(2, 3).unwrap();
    let v = vec![
        VectorFieldEntry(0, 1),
        VectorFieldEntry(0, 0),
        VectorFieldEntry(0, 1),
        VectorFieldEntry(0, 0),
        VectorFieldEntry(0, -1),
        VectorFieldEntry(0, 0),
    ];
    DimensionsAndPermutation {
        permutation: conversion::to_image(&dimensions, &v),
        dimensions,
    }
}

pub fn non_identity() -> DimensionsAndPermutation {
    let dimensions = ImageDimensions::new(2, 3).unwrap();
    let v = vec![
        VectorFieldEntry(0, 1),
        VectorFieldEntry(0, 0),
        VectorFieldEntry(0, -1),
        VectorFieldEntry(-1, 1),
        VectorFieldEntry(1, 0),
        VectorFieldEntry(0, -1),
    ];
    DimensionsAndPermutation {
        permutation: conversion::to_image(&dimensions, &v),
        dimensions,
    }
}

pub fn non_identity_forward_permute<Component: 'static + image::Primitive>(
    image: &ImageBuffer<image::Rgba<Component>, Vec<Component>>,
) -> ImageBuffer<image::Rgba<Component>, Vec<Component>> {
    assert_eq!(image.width(), 2);
    assert_eq!(image.height(), 3);
    let mut permuted_image = image.clone();
    let mut pixel1 = *permuted_image.get_pixel(0, 0);
    let mut pixel2 = *permuted_image.get_pixel(0, 1);
    permuted_image.put_pixel(0, 0, pixel2);
    permuted_image.put_pixel(0, 1, pixel1);
    pixel1 = *permuted_image.get_pixel(1, 1);
    pixel2 = *permuted_image.get_pixel(0, 2);
    let pixel3 = *permuted_image.get_pixel(1, 2);
    permuted_image.put_pixel(1, 1, pixel2);
    permuted_image.put_pixel(0, 2, pixel3);
    permuted_image.put_pixel(1, 2, pixel1);
    permuted_image
}

pub fn non_identity_horizontal_swap_forward_permute(
    image: &Rgba16ImageBuffer,
) -> Rgba16ImageBuffer {
    assert_eq!(image.width(), 2);
    assert_eq!(image.height(), 3);
    let mut permuted_image = image.clone();
    let pixel1 = *permuted_image.get_pixel(0, 0);
    let pixel2 = *permuted_image.get_pixel(1, 0);
    let pixel3 = *permuted_image.get_pixel(0, 1);
    let pixel4 = *permuted_image.get_pixel(1, 1);
    let pixel5 = *permuted_image.get_pixel(0, 2);

    permuted_image.put_pixel(0, 0, pixel2);
    permuted_image.put_pixel(1, 0, pixel3);
    permuted_image.put_pixel(0, 1, pixel5);
    permuted_image.put_pixel(1, 1, pixel1);
    permuted_image.put_pixel(0, 2, pixel4);
    permuted_image
}

pub fn bit_interpretation_cases() -> DimensionsAndPermutation {
    let dimensions = ImageDimensions::new(513, 513).unwrap();
    let mut v = vec![VectorFieldEntry(0, 0); dimensions.count()];
    // One cycle
    v[0] = VectorFieldEntry(1, 128); // (0, 0) to (1, 128)
    v[65665] = VectorFieldEntry(128, 1); // (1, 128) to (129, 129)
    v[66306] = VectorFieldEntry(129, 256); // (129, 129) to (258, 385)
    v[197763] = VectorFieldEntry(-127, -384); // (258, 385) to (131, 1)
    v[644] = VectorFieldEntry(-131, -1); // (131, 1) to (0, 0)
                                         // Swap
    v[1] = VectorFieldEntry(257, 384); // (1, 0) to (258, 384)
    v[197250] = VectorFieldEntry(-257, -384); // (258, 384) to (1, 0)
                                              // Swap
    v[262661] = VectorFieldEntry(385, -512); // (5, 512) to (390, 0)
    v[390] = VectorFieldEntry(-385, 512); // (390, 0) to (5, 512)
                                          // Swap
    v[513] = VectorFieldEntry(511, 383); // (0, 1) to (511, 384)
    v[197503] = VectorFieldEntry(-511, -383); // (511, 384) to (0, 1)
                                              // Swap
    v[2] = VectorFieldEntry(256, 255); // (2, 0) to (258, 255)
    v[131073] = VectorFieldEntry(-256, -255); // (258, 255) to (2, 0)
                                              // Swap
    v[3] = VectorFieldEntry(128, 100); // (3, 0) to (131, 100)
    v[51431] = VectorFieldEntry(-128, -100); // (131, 100) to (3, 0)

    DimensionsAndPermutation {
        permutation: conversion::to_image(&dimensions, &v),
        dimensions,
    }
}

pub fn bit_interpretation_cases_forward_permute(image: &Rgba16ImageBuffer) -> Rgba16ImageBuffer {
    assert_eq!(image.width(), 513);
    assert_eq!(image.height(), 513);
    let mut permuted_image = image.clone();
    let mut pixel1 = *permuted_image.get_pixel(0, 0);
    let mut pixel2 = *permuted_image.get_pixel(1, 128);
    permuted_image.put_pixel(0, 0, pixel2);
    pixel2 = *permuted_image.get_pixel(129, 129);
    permuted_image.put_pixel(1, 128, pixel2);
    pixel2 = *permuted_image.get_pixel(258, 385);
    permuted_image.put_pixel(129, 129, pixel2);
    pixel2 = *permuted_image.get_pixel(131, 1);
    permuted_image.put_pixel(258, 385, pixel2);
    permuted_image.put_pixel(131, 1, pixel1);

    pixel1 = *permuted_image.get_pixel(1, 0);
    pixel2 = *permuted_image.get_pixel(258, 384);
    permuted_image.put_pixel(1, 0, pixel2);
    permuted_image.put_pixel(258, 384, pixel1);

    pixel1 = *permuted_image.get_pixel(5, 512);
    pixel2 = *permuted_image.get_pixel(390, 0);
    permuted_image.put_pixel(5, 512, pixel2);
    permuted_image.put_pixel(390, 0, pixel1);

    pixel1 = *permuted_image.get_pixel(0, 1);
    pixel2 = *permuted_image.get_pixel(511, 384);
    permuted_image.put_pixel(0, 1, pixel2);
    permuted_image.put_pixel(511, 384, pixel1);

    pixel1 = *permuted_image.get_pixel(2, 0);
    pixel2 = *permuted_image.get_pixel(258, 255);
    permuted_image.put_pixel(2, 0, pixel2);
    permuted_image.put_pixel(258, 255, pixel1);

    pixel1 = *permuted_image.get_pixel(3, 0);
    pixel2 = *permuted_image.get_pixel(131, 100);
    permuted_image.put_pixel(3, 0, pixel2);
    permuted_image.put_pixel(131, 100, pixel1);

    permuted_image
}

pub fn reflect_around_center() -> DimensionsAndPermutation {
    // 257 is the smallest odd-numbered size such that permutation vector components will
    // exceed absolute values of 255 and therefore test correct handling of both bytes
    // in permutation vector components.
    let dimensions = ImageDimensions::new(257, 257).unwrap();
    let count = dimensions.count();
    let mut v: Vec<VectorFieldEntry> = Vec::with_capacity(count);
    let center_x: i16 = (dimensions.width() / 2).try_into().unwrap();
    let center_y: i16 = (dimensions.height() / 2).try_into().unwrap();
    for k in 0..count {
        let (x, y) = dimensions.make_coordinates(k).unwrap();
        v.push(VectorFieldEntry(
            2i16 * (center_x - <usize as TryInto<i16>>::try_into(x).unwrap()),
            2i16 * (center_y - <usize as TryInto<i16>>::try_into(y).unwrap()),
        ))
    }

    DimensionsAndPermutation {
        permutation: conversion::to_image(&dimensions, &v),
        dimensions,
    }
}

pub fn assert_is_identity(permutation: &ValidatedPermutation) {
    let converted_permutation = conversion::to_vec(permutation.as_ref());
    let dim = permutation.dimensions();
    let mut expected: Vec<VectorFieldEntry> = Vec::with_capacity(dim.count());
    expected.resize(dim.count(), VectorFieldEntry(0, 0));
    assert_eq!(converted_permutation, expected);
}
