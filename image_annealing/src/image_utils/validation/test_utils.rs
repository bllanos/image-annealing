use crate::compute::conversion::{self, PermutationEntry};
use crate::compute::format::PermutationImageBuffer;
use crate::image_utils::ImageDimensions;

pub struct DimensionsAndImage {
    pub image: PermutationImageBuffer,
    pub dimensions: ImageDimensions,
}

pub fn identity() -> DimensionsAndImage {
    let dimensions = ImageDimensions::new(3, 5).unwrap();
    let v = vec![PermutationEntry(0, 0); dimensions.count()];
    DimensionsAndImage {
        image: conversion::as_image(&dimensions, &v),
        dimensions,
    }
}

pub fn duplicate() -> DimensionsAndImage {
    let dimensions = ImageDimensions::new(1, 3).unwrap();
    let v = vec![
        PermutationEntry(0, 1),
        PermutationEntry(0, 1),
        PermutationEntry(0, -1),
    ];
    DimensionsAndImage {
        image: conversion::as_image(&dimensions, &v),
        dimensions,
    }
}

pub fn non_identity() -> DimensionsAndImage {
    let dimensions = ImageDimensions::new(2, 3).unwrap();
    let v = vec![
        PermutationEntry(0, 1),
        PermutationEntry(0, 0),
        PermutationEntry(0, -1),
        PermutationEntry(-1, 1),
        PermutationEntry(1, 0),
        PermutationEntry(0, -1),
    ];
    DimensionsAndImage {
        image: conversion::as_image(&dimensions, &v),
        dimensions,
    }
}
