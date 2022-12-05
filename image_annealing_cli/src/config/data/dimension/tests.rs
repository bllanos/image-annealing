mod unverified_image_dimensions_config {
    mod try_into_dimensions {
        use super::super::super::UnverifiedImageDimensionsConfig;
        use image_annealing::ImageDimensions;
        use std::error::Error;

        #[test]
        fn success() -> Result<(), Box<dyn Error>> {
            let config = UnverifiedImageDimensionsConfig {
                width: 20,
                height: 25,
            };
            assert_eq!(
                <UnverifiedImageDimensionsConfig as TryInto<ImageDimensions>>::try_into(config)?,
                ImageDimensions::try_new(20, 25)?
            );
            Ok(())
        }

        #[test]
        fn zero_width() {
            test_util::assert_error_contains(
                <UnverifiedImageDimensionsConfig as TryInto<ImageDimensions>>::try_into(
                    UnverifiedImageDimensionsConfig {
                        width: 0,
                        height: 25,
                    },
                ),
                "width is zero",
            );
        }

        #[test]
        fn zero_height() {
            test_util::assert_error_contains(
                <UnverifiedImageDimensionsConfig as TryInto<ImageDimensions>>::try_into(
                    UnverifiedImageDimensionsConfig {
                        width: 20,
                        height: 0,
                    },
                ),
                "height is zero",
            );
        }
    }
}
