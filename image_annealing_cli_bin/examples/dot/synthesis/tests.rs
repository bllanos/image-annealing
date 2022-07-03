mod dot_geometry {
    use super::super::DotGeometry;
    use image_annealing::ImageDimensions;
    use std::error::Error;

    #[test]
    fn dimensions_1_1() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::new(1, 1)?;
        assert_eq!(
            DotGeometry::new(&dimensions),
            DotGeometry {
                radius_squared: 0.0,
                x: 0.5,
                y: 0.5,
            }
        );
        Ok(())
    }

    #[test]
    fn dimensions_1_2() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::new(1, 2)?;
        assert_eq!(
            DotGeometry::new(&dimensions),
            DotGeometry {
                radius_squared: 0.0,
                x: 0.5,
                y: 1.0,
            }
        );
        Ok(())
    }

    #[test]
    fn dimensions_2_1() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::new(2, 1)?;
        assert_eq!(
            DotGeometry::new(&dimensions),
            DotGeometry {
                radius_squared: 0.0,
                x: 1.0,
                y: 0.5,
            }
        );
        Ok(())
    }

    #[test]
    fn dimensions_2_2() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::new(2, 2)?;
        assert_eq!(
            DotGeometry::new(&dimensions),
            DotGeometry {
                radius_squared: 1.0,
                x: 1.0,
                y: 1.0,
            }
        );
        Ok(())
    }

    #[test]
    fn dimensions_2_3() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::new(2, 3)?;
        assert_eq!(
            DotGeometry::new(&dimensions),
            DotGeometry {
                radius_squared: 1.0,
                x: 1.0,
                y: 1.5,
            }
        );
        Ok(())
    }

    #[test]
    fn dimensions_3_2() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::new(3, 2)?;
        assert_eq!(
            DotGeometry::new(&dimensions),
            DotGeometry {
                radius_squared: 1.0,
                x: 1.5,
                y: 1.0,
            }
        );
        Ok(())
    }

    #[test]
    fn dimensions_3_3() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::new(3, 3)?;
        assert_eq!(
            DotGeometry::new(&dimensions),
            DotGeometry {
                radius_squared: 1.0,
                x: 1.5,
                y: 1.5,
            }
        );
        Ok(())
    }

    #[test]
    fn dimensions_4_4() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::new(4, 4)?;
        assert_eq!(
            DotGeometry::new(&dimensions),
            DotGeometry {
                radius_squared: 4.0,
                x: 2.0,
                y: 2.0,
            }
        );
        Ok(())
    }
}

mod white_dot {
    use super::super::white_dot;
    use image_annealing::ImageDimensions;
    use std::error::Error;

    #[test]
    fn dimensions_1_1() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::new(1, 1)?;
        assert_eq!(
            white_dot(&dimensions),
            image::RgbaImage::from_pixel(
                dimensions.width().try_into()?,
                dimensions.height().try_into()?,
                image::Rgba([255, 255, 255, 255])
            )
        );
        Ok(())
    }

    #[test]
    fn dimensions_1_2() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::new(1, 2)?;
        assert_eq!(
            white_dot(&dimensions),
            image::RgbaImage::from_pixel(
                dimensions.width().try_into()?,
                dimensions.height().try_into()?,
                image::Rgba([0, 0, 0, 255])
            )
        );
        Ok(())
    }

    #[test]
    fn dimensions_2_1() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::new(2, 1)?;
        assert_eq!(
            white_dot(&dimensions),
            image::RgbaImage::from_pixel(
                dimensions.width().try_into()?,
                dimensions.height().try_into()?,
                image::Rgba([0, 0, 0, 255])
            )
        );
        Ok(())
    }

    #[test]
    fn dimensions_2_2() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::new(2, 2)?;
        assert_eq!(
            white_dot(&dimensions),
            image::RgbaImage::from_pixel(
                dimensions.width().try_into()?,
                dimensions.height().try_into()?,
                image::Rgba([255, 255, 255, 255])
            )
        );
        Ok(())
    }

    #[test]
    fn dimensions_2_3() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::new(2, 3)?;
        assert_eq!(
            white_dot(&dimensions),
            image::RgbaImage::from_vec(
                dimensions.width().try_into()?,
                dimensions.height().try_into()?,
                vec![
                    0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0,
                    255, 0, 0, 0, 255
                ]
            )
            .unwrap()
        );
        Ok(())
    }

    #[test]
    fn dimensions_3_2() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::new(3, 2)?;
        assert_eq!(
            white_dot(&dimensions),
            image::RgbaImage::from_vec(
                dimensions.width().try_into()?,
                dimensions.height().try_into()?,
                vec![
                    0, 0, 0, 255, 255, 255, 255, 255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 255,
                    255, 0, 0, 0, 255
                ]
            )
            .unwrap()
        );
        Ok(())
    }

    #[test]
    fn dimensions_3_3() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::new(3, 3)?;
        assert_eq!(
            white_dot(&dimensions),
            image::RgbaImage::from_vec(
                dimensions.width().try_into()?,
                dimensions.height().try_into()?,
                vec![
                    0, 0, 0, 255, 255, 255, 255, 255, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 0, 0, 0, 255, 255, 255, 255, 255, 0, 0, 0, 255,
                ]
            )
            .unwrap()
        );
        Ok(())
    }

    #[test]
    fn dimensions_4_4() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::new(4, 4)?;
        assert_eq!(
            white_dot(&dimensions),
            image::RgbaImage::from_vec(
                dimensions.width().try_into()?,
                dimensions.height().try_into()?,
                vec![
                    0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 255,
                ]
            )
            .unwrap()
        );
        Ok(())
    }
}

mod dot_goal {
    use super::super::dot_goal;
    use image_annealing::compute::conversion::{self, VectorFieldEntry, VectorFieldEntryComponent};
    use image_annealing::ImageDimensions;
    use std::error::Error;

    #[test]
    fn dimensions_1_1() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::new(1, 1)?;
        assert_eq!(
            dot_goal(&dimensions).as_ref(),
            &conversion::to_image(
                &dimensions,
                &vec![VectorFieldEntry(
                    VectorFieldEntryComponent::MAX,
                    VectorFieldEntryComponent::MAX,
                )]
            ),
        );
        Ok(())
    }

    #[test]
    fn dimensions_1_2() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::new(1, 2)?;
        assert_eq!(
            dot_goal(&dimensions).as_ref(),
            &conversion::to_image(
                &dimensions,
                &vec![VectorFieldEntry(0, 1,), VectorFieldEntry(0, -1,)]
            ),
        );
        Ok(())
    }

    #[test]
    fn dimensions_2_1() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::new(2, 1)?;
        assert_eq!(
            dot_goal(&dimensions).as_ref(),
            &conversion::to_image(
                &dimensions,
                &vec![VectorFieldEntry(1, 0,), VectorFieldEntry(-1, 0,)]
            ),
        );
        Ok(())
    }

    #[test]
    fn dimensions_2_2() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::new(2, 2)?;
        assert_eq!(
            dot_goal(&dimensions).as_ref(),
            &conversion::to_image(
                &dimensions,
                &vec![
                    VectorFieldEntry(
                        -VectorFieldEntryComponent::MAX,
                        -VectorFieldEntryComponent::MAX,
                    ),
                    VectorFieldEntry(
                        VectorFieldEntryComponent::MAX,
                        -VectorFieldEntryComponent::MAX,
                    ),
                    VectorFieldEntry(
                        -VectorFieldEntryComponent::MAX,
                        VectorFieldEntryComponent::MAX,
                    ),
                    VectorFieldEntry(
                        VectorFieldEntryComponent::MAX,
                        VectorFieldEntryComponent::MAX,
                    )
                ]
            ),
        );
        Ok(())
    }

    #[test]
    fn dimensions_2_3() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::new(2, 3)?;
        assert_eq!(
            dot_goal(&dimensions).as_ref(),
            &conversion::to_image(
                &dimensions,
                &vec![
                    VectorFieldEntry(1, 1,),
                    VectorFieldEntry(-1, 1,),
                    VectorFieldEntry(-VectorFieldEntryComponent::MAX, 0,),
                    VectorFieldEntry(VectorFieldEntryComponent::MAX, 0,),
                    VectorFieldEntry(1, -1,),
                    VectorFieldEntry(-1, -1,),
                ]
            ),
        );
        Ok(())
    }

    #[test]
    fn dimensions_3_2() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::new(3, 2)?;
        assert_eq!(
            dot_goal(&dimensions).as_ref(),
            &conversion::to_image(
                &dimensions,
                &vec![
                    VectorFieldEntry(1, 1,),
                    VectorFieldEntry(0, -VectorFieldEntryComponent::MAX,),
                    VectorFieldEntry(-1, 1,),
                    VectorFieldEntry(1, -1,),
                    VectorFieldEntry(0, VectorFieldEntryComponent::MAX,),
                    VectorFieldEntry(-1, -1,),
                ]
            ),
        );
        Ok(())
    }

    #[test]
    fn dimensions_3_3() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::new(3, 3)?;
        assert_eq!(
            dot_goal(&dimensions).as_ref(),
            &conversion::to_image(
                &dimensions,
                &vec![
                    VectorFieldEntry(1, 1,),
                    VectorFieldEntry(0, -VectorFieldEntryComponent::MAX,),
                    VectorFieldEntry(-1, 1,),
                    VectorFieldEntry(-VectorFieldEntryComponent::MAX, 0),
                    VectorFieldEntry(
                        VectorFieldEntryComponent::MAX,
                        VectorFieldEntryComponent::MAX
                    ),
                    VectorFieldEntry(VectorFieldEntryComponent::MAX, 0),
                    VectorFieldEntry(1, -1,),
                    VectorFieldEntry(0, VectorFieldEntryComponent::MAX,),
                    VectorFieldEntry(-1, -1,),
                ]
            ),
        );
        Ok(())
    }

    #[test]
    fn dimensions_4_4() -> Result<(), Box<dyn Error>> {
        let dimensions = ImageDimensions::new(4, 4)?;
        let max_div2 = VectorFieldEntryComponent::MAX / 2;
        let max_sub1 = VectorFieldEntryComponent::MAX - 1;
        assert_eq!(
            dot_goal(&dimensions).as_ref(),
            &conversion::to_image(
                &dimensions,
                &vec![
                    VectorFieldEntry(2, 2,),
                    VectorFieldEntry(-max_div2, -max_sub1,),
                    VectorFieldEntry(max_div2, -max_sub1,),
                    VectorFieldEntry(-2, 2,),
                    VectorFieldEntry(-max_sub1, -max_div2),
                    VectorFieldEntry(
                        -VectorFieldEntryComponent::MAX,
                        -VectorFieldEntryComponent::MAX
                    ),
                    VectorFieldEntry(
                        VectorFieldEntryComponent::MAX,
                        -VectorFieldEntryComponent::MAX
                    ),
                    VectorFieldEntry(max_sub1, -max_div2),
                    VectorFieldEntry(-max_sub1, max_div2),
                    VectorFieldEntry(
                        -VectorFieldEntryComponent::MAX,
                        VectorFieldEntryComponent::MAX
                    ),
                    VectorFieldEntry(
                        VectorFieldEntryComponent::MAX,
                        VectorFieldEntryComponent::MAX
                    ),
                    VectorFieldEntry(max_sub1, max_div2),
                    VectorFieldEntry(2, -2,),
                    VectorFieldEntry(-max_div2, max_sub1,),
                    VectorFieldEntry(max_div2, max_sub1,),
                    VectorFieldEntry(-2, -2,),
                ]
            ),
        );
        Ok(())
    }
}
