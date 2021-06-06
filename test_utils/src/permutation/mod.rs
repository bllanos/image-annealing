use image_annealing::compute::conversion::{self, PermutationEntry};
use image_annealing::compute::format::LosslessImageBuffer;
use image_annealing::compute::format::PermutationImageBuffer;
use image_annealing::image_utils::ImageDimensions;

pub struct DimensionsAndPermutation {
    pub permutation: PermutationImageBuffer,
    pub dimensions: ImageDimensions,
}

pub fn identity() -> DimensionsAndPermutation {
    let dimensions = ImageDimensions::new(3, 5).unwrap();
    let v = vec![PermutationEntry(0, 0); dimensions.count()];
    DimensionsAndPermutation {
        permutation: conversion::as_image(&dimensions, &v),
        dimensions,
    }
}

pub fn identity_permute(image: &LosslessImageBuffer) -> LosslessImageBuffer {
    image.clone()
}

pub fn duplicate() -> DimensionsAndPermutation {
    let dimensions = ImageDimensions::new(2, 3).unwrap();
    let v = vec![
        PermutationEntry(0, 1),
        PermutationEntry(0, 0),
        PermutationEntry(0, 1),
        PermutationEntry(0, 0),
        PermutationEntry(0, -1),
        PermutationEntry(0, 0),
    ];
    DimensionsAndPermutation {
        permutation: conversion::as_image(&dimensions, &v),
        dimensions,
    }
}

pub fn non_identity() -> DimensionsAndPermutation {
    let dimensions = ImageDimensions::new(2, 3).unwrap();
    let v = vec![
        PermutationEntry(0, 1),
        PermutationEntry(0, 0),
        PermutationEntry(0, -1),
        PermutationEntry(-1, 1),
        PermutationEntry(1, 0),
        PermutationEntry(0, -1),
    ];
    DimensionsAndPermutation {
        permutation: conversion::as_image(&dimensions, &v),
        dimensions,
    }
}

pub fn non_identity_forward_permute(image: &LosslessImageBuffer) -> LosslessImageBuffer {
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
    return permuted_image;
}

pub fn bit_interpretation_cases() -> DimensionsAndPermutation {
    let dimensions = ImageDimensions::new(513, 513).unwrap();
    let mut v = vec![PermutationEntry(0, 0); dimensions.count()];
    // One cycle
    v[0] = PermutationEntry(1, 128); // (0, 0) to (1, 128)
    v[65665] = PermutationEntry(128, 1); // (1, 128) to (129, 129)
    v[66306] = PermutationEntry(129, 256); // (129, 129) to (258, 385)
    v[197763] = PermutationEntry(-127, -384); // (258, 385) to (131, 1)
    v[644] = PermutationEntry(-131, -1); // (131, 1) to (0, 0)
                                         // Swap
    v[1] = PermutationEntry(257, 384); // (1, 0) to (258, 384)
    v[197250] = PermutationEntry(-257, -384); // (258, 384) to (1, 0)
                                              // Swap
    v[262661] = PermutationEntry(385, -512); // (5, 512) to (390, 0)
    v[390] = PermutationEntry(-385, 512); // (390, 0) to (5, 512)
                                          // Swap
    v[513] = PermutationEntry(511, 383); // (0, 1) to (511, 384)
    v[197503] = PermutationEntry(-511, -383); // (511, 384) to (0, 1)
                                              // Swap
    v[2] = PermutationEntry(256, 255); // (2, 0) to (258, 255)
    v[131073] = PermutationEntry(-256, -255); // (258, 255) to (2, 0)
                                              // Swap
    v[3] = PermutationEntry(128, 100); // (3, 0) to (131, 100)
    v[51431] = PermutationEntry(-128, -100); // (131, 100) to (3, 0)

    DimensionsAndPermutation {
        permutation: conversion::as_image(&dimensions, &v),
        dimensions,
    }
}

pub fn bit_interpretation_cases_forward_permute(
    image: &LosslessImageBuffer,
) -> LosslessImageBuffer {
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

    return permuted_image;
}
