use super::System;
use crate::ImageDimensions;

fn create_system_single_pixel() -> System {
    System::new(&ImageDimensions::new(1, 1).unwrap()).unwrap()
}

mod output_permutation {
    use std::error::Error;

    #[test]
    fn no_preceding_operations() -> Result<(), Box<dyn Error>> {
        let mut system = super::create_system_single_pixel();
        test_utils::assert_error_contains(
            system.output_permutation(),
            "an output permutation does not exist or has been invalidated",
        );
        Ok(())
    }
}

mod output_permuted_image {
    use std::error::Error;

    #[test]
    fn no_preceding_operations() -> Result<(), Box<dyn Error>> {
        let mut system = super::create_system_single_pixel();
        test_utils::assert_error_contains(
            system.output_permuted_image(),
            "an output image does not exist or has been invalidated",
        );
        Ok(())
    }
}
