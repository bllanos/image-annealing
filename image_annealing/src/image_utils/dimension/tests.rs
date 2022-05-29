use super::ImageDimensions;
use image::RgbaImage;
use std::convert::TryInto;
use std::error::Error;

mod new {
    use super::super::ImageDimensions;
    use std::error::Error;

    #[test]
    fn negative_width() {
        test_utils::assert_error_contains(
            ImageDimensions::new(-1, 1),
            "failed to convert -1 to the required type for dimensions",
        )
    }

    #[test]
    fn negative_height() {
        test_utils::assert_error_contains(
            ImageDimensions::new(1, -1),
            "failed to convert -1 to the required type for dimensions",
        )
    }

    #[test]
    fn zero_width() {
        test_utils::assert_error_contains(ImageDimensions::new(0, 1), "width is zero")
    }

    #[test]
    fn zero_height() {
        test_utils::assert_error_contains(ImageDimensions::new(1, 0), "height is zero")
    }

    #[test]
    fn valid_dimensions() -> Result<(), Box<dyn Error>> {
        let dim = ImageDimensions::new(63, 64)?;
        assert_eq!(dim.width(), 63);
        assert_eq!(dim.height(), 64);
        assert_eq!(dim.count(), 63 * 64);
        Ok(())
    }
}

#[test]
fn from_image() -> Result<(), Box<dyn Error>> {
    let image = RgbaImage::new(4, 5);
    let dim = ImageDimensions::from_image(&image)?;
    assert_eq!(dim.width(), image.width().try_into().unwrap());
    assert_eq!(dim.height(), image.height().try_into().unwrap());
    Ok(())
}

mod from_image_path {
    use super::ImageDimensions;
    use std::error::Error;

    #[test]
    fn from_image_path() -> Result<(), Box<dyn Error>> {
        let path = test_utils::make_test_data_path(&["image", "image", "stripes.png"]);
        let dim = ImageDimensions::from_image_path(path)?;
        assert_eq!(dim.width(), 20);
        assert_eq!(dim.height(), 25);
        Ok(())
    }

    #[test]
    fn missing_image() {
        let path = test_utils::make_test_data_path(&["image", "image", "not_found.png"]);
        test_utils::assert_error_contains(
            ImageDimensions::from_image_path(path),
            "No such file or directory",
        );
    }

    #[test]
    fn non_image() {
        let path = test_utils::make_test_data_path(&["empty.txt"]);
        test_utils::assert_error_contains(
            ImageDimensions::from_image_path(path),
            "The file extension `.\"txt\"` was not recognized as an image format",
        );
    }
}

mod make_linear_index {
    use super::super::ImageDimensions;
    use std::error::Error;

    #[test]
    fn negative_x() -> Result<(), Box<dyn Error>> {
        let dim = ImageDimensions::new(3, 5)?;
        test_utils::assert_error_contains(
            dim.make_linear_index(-1, 0),
            "failed to convert -1 to the required type for coordinates",
        );
        Ok(())
    }

    #[test]
    fn negative_y() -> Result<(), Box<dyn Error>> {
        let dim = ImageDimensions::new(3, 5)?;
        test_utils::assert_error_contains(
            dim.make_linear_index(0, -1),
            "failed to convert -1 to the required type for coordinates",
        );
        Ok(())
    }

    #[test]
    fn large_x() -> Result<(), Box<dyn Error>> {
        let dim = ImageDimensions::new(3, 5)?;
        test_utils::assert_error_contains(
            dim.make_linear_index(3, 0),
            "x = 3 is out of bounds (width, height) = (3, 5)",
        );
        Ok(())
    }

    #[test]
    fn large_y() -> Result<(), Box<dyn Error>> {
        let dim = ImageDimensions::new(3, 5)?;
        test_utils::assert_error_contains(
            dim.make_linear_index(0, 5),
            "y = 5 is out of bounds (width, height) = (3, 5)",
        );
        Ok(())
    }

    #[test]
    fn valid_coordinates() -> Result<(), Box<dyn Error>> {
        let dim = ImageDimensions::new(3, 5)?;
        let mut i = dim.make_linear_index(0, 0)?;
        assert_eq!(i, 0);
        i = dim.make_linear_index(1, 0)?;
        assert_eq!(i, 1);
        i = dim.make_linear_index(0, 1)?;
        assert_eq!(i, 3);
        i = dim.make_linear_index(1, 1)?;
        assert_eq!(i, 4);
        i = dim.make_linear_index(2, 4)?;
        assert_eq!(i, 3 * 5 - 1);
        Ok(())
    }
}

mod make_coordinates {
    use super::super::ImageDimensions;
    use std::error::Error;

    #[test]
    fn negative() -> Result<(), Box<dyn Error>> {
        let dim = ImageDimensions::new(3, 5)?;
        test_utils::assert_error_contains(
            dim.make_coordinates(-1),
            "failed to convert -1 to the required type for coordinates",
        );
        Ok(())
    }

    #[test]
    fn large() -> Result<(), Box<dyn Error>> {
        let dim = ImageDimensions::new(3, 5)?;
        test_utils::assert_error_contains(
            dim.make_coordinates(15),
            "linear index 15 is out of bounds (width, height) = (3, 5)",
        );
        Ok(())
    }

    #[test]
    fn valid_indices() -> Result<(), Box<dyn Error>> {
        let dim = ImageDimensions::new(3, 5)?;
        let mut coordinates = dim.make_coordinates(0)?;
        assert_eq!(coordinates.0, 0);
        assert_eq!(coordinates.1, 0);
        coordinates = dim.make_coordinates(1)?;
        assert_eq!(coordinates.0, 1);
        assert_eq!(coordinates.1, 0);
        coordinates = dim.make_coordinates(2)?;
        assert_eq!(coordinates.0, 2);
        assert_eq!(coordinates.1, 0);
        coordinates = dim.make_coordinates(3)?;
        assert_eq!(coordinates.0, 0);
        assert_eq!(coordinates.1, 1);
        coordinates = dim.make_coordinates(11)?;
        assert_eq!(coordinates.0, 2);
        assert_eq!(coordinates.1, 3);
        coordinates = dim.make_coordinates(12)?;
        assert_eq!(coordinates.0, 0);
        assert_eq!(coordinates.1, 4);
        coordinates = dim.make_coordinates(14)?;
        assert_eq!(coordinates.0, 2);
        assert_eq!(coordinates.1, 4);
        Ok(())
    }
}

#[test]
fn to_extent() -> Result<(), Box<dyn Error>> {
    let dim = ImageDimensions::new(63, 64)?;
    assert_eq!(
        dim.to_extent(),
        wgpu::Extent3d {
            width: 63,
            height: 64,
            depth_or_array_layers: 1
        }
    );
    Ok(())
}

mod try_from_extent {
    use super::super::ImageDimensions;
    use std::convert::TryFrom;
    use std::error::Error;

    #[test]
    fn zero_depth() {
        test_utils::assert_error_contains(
            <ImageDimensions as TryFrom<wgpu::Extent3d>>::try_from(wgpu::Extent3d {
                width: 63,
                height: 64,
                depth_or_array_layers: 0,
            }),
            "depth is not 1",
        )
    }

    #[test]
    fn zero_width() {
        test_utils::assert_error_contains(
            <ImageDimensions as TryFrom<wgpu::Extent3d>>::try_from(wgpu::Extent3d {
                width: 0,
                height: 64,
                depth_or_array_layers: 1,
            }),
            "width is zero",
        )
    }

    #[test]
    fn zero_height() {
        test_utils::assert_error_contains(
            <ImageDimensions as TryFrom<wgpu::Extent3d>>::try_from(wgpu::Extent3d {
                width: 63,
                height: 0,
                depth_or_array_layers: 1,
            }),
            "height is zero",
        )
    }

    #[test]
    fn large_depth() {
        test_utils::assert_error_contains(
            <ImageDimensions as TryFrom<wgpu::Extent3d>>::try_from(wgpu::Extent3d {
                width: 63,
                height: 64,
                depth_or_array_layers: 2,
            }),
            "depth is not 1",
        )
    }

    #[test]
    fn valid_dimensions() -> Result<(), Box<dyn Error>> {
        let dim = <ImageDimensions as TryFrom<wgpu::Extent3d>>::try_from(wgpu::Extent3d {
            width: 63,
            height: 64,
            depth_or_array_layers: 1,
        })?;
        assert_eq!(dim.width(), 63);
        assert_eq!(dim.height(), 64);
        Ok(())
    }
}

mod partial_eq_extent {
    use super::super::ImageDimensions;
    use std::error::Error;

    #[test]
    fn zero_depth() -> Result<(), Box<dyn Error>> {
        let dim = ImageDimensions::new(63, 64)?;
        assert!(
            dim != wgpu::Extent3d {
                width: 63,
                height: 64,
                depth_or_array_layers: 0,
            }
        );
        Ok(())
    }

    #[test]
    fn large_depth() -> Result<(), Box<dyn Error>> {
        let dim = ImageDimensions::new(63, 64)?;
        assert!(
            dim != wgpu::Extent3d {
                width: 63,
                height: 64,
                depth_or_array_layers: 2,
            }
        );
        Ok(())
    }

    #[test]
    fn different_width() -> Result<(), Box<dyn Error>> {
        let dim = ImageDimensions::new(63, 64)?;
        assert!(
            dim != wgpu::Extent3d {
                width: 64,
                height: 64,
                depth_or_array_layers: 1,
            }
        );
        Ok(())
    }

    #[test]
    fn different_height() -> Result<(), Box<dyn Error>> {
        let dim = ImageDimensions::new(63, 64)?;
        assert!(
            dim != wgpu::Extent3d {
                width: 63,
                height: 63,
                depth_or_array_layers: 1,
            }
        );
        Ok(())
    }

    #[test]
    fn equal() -> Result<(), Box<dyn Error>> {
        let dim = ImageDimensions::new(63, 64)?;
        assert!(
            dim == wgpu::Extent3d {
                width: 63,
                height: 64,
                depth_or_array_layers: 1,
            }
        );
        Ok(())
    }
}
