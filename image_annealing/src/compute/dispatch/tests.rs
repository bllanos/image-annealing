mod dimensions {
    use super::super::Config;
    use crate::ImageDimensions;
    use std::error::Error;

    #[test]
    fn same_dimensions() -> Result<(), Box<dyn Error>> {
        let image_dimensions = ImageDimensions::try_new(3, 2)?;
        let dispatcher = super::super::create_dispatcher_block(&Config { image_dimensions })?;
        assert_eq!(dispatcher.dimensions(), &image_dimensions);
        Ok(())
    }
}
